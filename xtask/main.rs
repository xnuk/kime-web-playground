use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt::Write as FmtWrite;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{
	env, fs,
	io::{self, Write as IOWrite},
	thread,
};

use anyhow::anyhow;
use cargo_metadata::Metadata as CargoMetadata;
use clap::Parser;
use serde_json::json;

use notify_debouncer_mini as watcher;
use regex_lite::{NoExpand, Regex};

#[derive(Clone)]
struct Package {
	id: String,
	name: String,
	version: String,
	license: Option<String>,
	path: PathBuf,
}

#[derive(Clone)]
struct Runner {
	cargo: PathBuf,
	cwd: PathBuf,
	cargo_target_dir: PathBuf,
	pack_candidates: HashMap<String, Package>,
}

impl Runner {
	fn cmd<T, U>(&self, cmd: T, args: U) -> duct::Expression
	where
		T: duct::IntoExecutablePath,
		U::Item: Into<OsString>,
		U: IntoIterator,
	{
		duct::cmd(cmd, args).dir(self.cwd.clone())
	}

	fn resolve(&self, path: impl AsRef<Path>) -> PathBuf {
		self.cwd.as_path().join(path)
	}

	fn from_metadata(cargo: PathBuf, metadata: &CargoMetadata) -> Self {
		Runner {
			cargo,
			cwd: metadata.workspace_root.clone().into(),
			cargo_target_dir: metadata.target_directory.clone().into(),
			pack_candidates: get_lib(metadata).collect(),
		}
	}
}

fn get_lib(
	metadata: &CargoMetadata,
) -> impl Iterator<Item = (String, Package)> + '_ {
	// note: cargo hates non UTF-8 file paths, so don't care for now

	let workspace_members = &metadata.workspace_members;

	metadata
		.packages
		.iter()
		.filter(|package| {
			(
				// ours
				workspace_members.contains(&package.id)
			) && (
				// cdylib
				package
					.targets
					.iter()
					.any(|target| target.kind.contains(&"cdylib".to_string()))
			) && (
				// wasm-bindgen dependency
				package
					.dependencies
					.iter()
					.any(|dep| dep.name == "wasm-bindgen")
			)
		})
		.filter_map(|package| {
			let name = package.name.clone();

			Some((
				name.clone(),
				Package {
					id: package.id.repr.clone(),
					name,
					version: format!("{}", package.version),
					license: package.license.clone(),
					path: package.manifest_path.parent()?.to_path_buf().into(),
				},
			))
		})
}

trait Run {
	fn run(self, runner: &Runner) -> anyhow::Result<()>;
}

#[derive(clap::Parser, Debug)]
struct App {
	#[command(subcommand)]
	command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
	Pack(PackCmd),
	Env(EnvCmd),
}

impl Run for Command {
	fn run(self, runner: &Runner) -> anyhow::Result<()> {
		match self {
			Self::Pack(cmd) => cmd.run(runner),
			Self::Env(cmd) => cmd.run(runner),
		}
	}
}

fn fetch_metadata(cargo: impl AsRef<Path>) -> io::Result<CargoMetadata> {
	serde_json::de::from_str(
		&duct::cmd(
			cargo.as_ref(),
			[
				"metadata",
				"--format-version=1",
				"--offline",
				"--frozen",
				"--no-deps",
			],
		)
		.read()?,
	)
	.map_err(|e| e.into())
}

#[derive(clap::Args, Debug)]
struct PackCmd {
	#[arg(long, default_value = "dist/wasm-pkg/")]
	out_dir: PathBuf,

	#[arg(long)]
	package: Option<String>,

	#[arg(long, default_value_t)]
	verbose: bool,

	#[arg(long, default_value_t)]
	watch: bool,
}

// See this for details:
// https://github.com/WebAssembly/binaryen/blob/91114e6a9b69969e673ffb40e7dc54029d15a0f8/src/passes/MinifyImportsAndExports.cpp
fn parse_rename_map(output: &str) -> HashMap<&str, &str> {
	output
		.lines()
		.filter_map(|line| {
			line.rsplit_once(" => ")
				.map(|(original, renamed)| (renamed.trim(), original.trim()))
		})
		.collect()
}

const TRIPLE: &str = "wasm32-unknown-unknown";

impl PackCmd {
	fn package<'a>(
		&mut self,
		runner: &'a Runner,
	) -> anyhow::Result<&'a Package> {
		let candidates = &runner.pack_candidates;

		if let Some(package) =
			self.package.as_ref().and_then(|name| candidates.get(name))
		{
			return Ok(package);
		}

		if candidates.len() > 1 {
			let mut output = String::new();
			writeln!(
				&mut output,
				"There's too much packages, choose one of these:"
			)?;

			for package in candidates.values() {
				writeln!(&mut output, "- {}", package.id)?;
			}

			return Err(anyhow!("{output}"));
		}

		if let Some(package) = candidates.values().next() {
			self.package = Some(package.name.clone());
			Ok(package)
		} else {
			Err(anyhow!("There's no wasm package."))
		}
	}

	fn cargo_build(&mut self, runner: &Runner) -> anyhow::Result<()> {
		let package = self.package(runner)?;

		if self.verbose {
			eprintln!("[xtask pack] cargo building {}", package.id);
		}

		let cargo_options = [
			"build".to_string(),
			"--lib".to_string(),
			"--locked".to_string(),
			"--release".to_string(),
			format!("--target={TRIPLE}"),
			format!("--package={}", package.name),
		];

		runner.cmd(&runner.cargo, cargo_options).run()?;

		Ok(())
	}

	fn wasm_bindgen(&mut self, runner: &Runner) -> anyhow::Result<()> {
		if self.verbose {
			eprintln!("[xtask pack] extracting glue");
		}

		let package_name = &self.package(runner)?.name;

		let mut wasm_path = runner.cargo_target_dir.clone();
		wasm_path.push(TRIPLE);
		wasm_path.push("release");
		wasm_path.push(package_name.replace('-', "_"));
		wasm_path.set_extension("wasm");

		let out_dir = runner.resolve(&self.out_dir);

		wasm_bindgen_cli_support::Bindgen::new()
			.input_path(wasm_path)
			.out_name(package_name)
			.debug(false)
			.demangle(true)
			.keep_debug(false)
			.typescript(true)
			.bundler(true)?
			.remove_name_section(true)
			.remove_producers_section(true)
			.omit_default_module_path(true)
			.generate(&out_dir)
	}

	fn wasm_opt(&mut self, runner: &Runner) -> anyhow::Result<()> {
		if self.verbose {
			eprintln!("[xtask pack] wasm-opt optimizing");
		}

		let package_name = &self.package(runner)?.name;

		let out_dir = runner.resolve(&self.out_dir);
		let mut path = out_dir.join(format!("{}_bg.wasm", package_name));

		let output = runner
			.cmd(
				"wasm-opt",
				[
					&OsString::from("--quiet"),
					&OsString::from("--fast-math"),
					&OsString::from("--minify-imports-and-exports-and-modules"),
					&OsString::from("-O"),
					path.as_os_str(),
					&OsString::from("-o"),
					path.as_os_str(),
				],
			)
			.read()?;

		let renamed_map = parse_rename_map(&output);

		if !renamed_map.is_empty() {
			// it's actually encoded with 'a', but wasm-opt does not print this.
			let module_original_name = format!("./{}_bg.js", package_name);

			let json = serde_json::to_vec_pretty(&json! {{
				"module": {"a": module_original_name},
				"functions": renamed_map,
				"version": "xnuk-r1",
			}})?;

			path.set_extension("wasm.renamed.json");
			fs::write(path, json)?;
		}

		// esbuild trick
		{
			let path = out_dir.join(format!("{}_bg.js", package_name));
			let file = fs::read_to_string(&path)?;

			// minify problem
			let new_text = Regex::new(
				r#"(^|\n)let wasm;\nexport function __wbg_set_wasm\s*\(val\)\s*\{[^}]*\}"#,
			)
			.unwrap()
			.replace(
				&file,
				NoExpand(&format!(
					"\n\
					import * as wasm from \"./{package_name}_bg.wasm\";\n\
					export {{ initialized }} from \"./{package_name}_bg.wasm\";\n\
					"
				)),
			);

			// they uses just `arguments` ???? insane
			let new_text = Regex::new(
				r#"\((\) \{ return (?:handle|log)Error\(function (?:.|\s*)+?\}, )arguments\)"#,
			)
			.unwrap()
			.replace_all(&new_text, "(...args${1}args)");

			fs::write(&path, new_text.as_bytes())?;
		}
		{
			let path = out_dir.join(format!("{}.d.ts", package_name));
			let mut file = fs::OpenOptions::new().append(true).open(path)?;
			writeln!(&mut file)?;
			writeln!(&mut file, "export const initialized: Promise<void>;")?;
		}

		Ok(())
	}

	fn package_json(&mut self, runner: &Runner) -> anyhow::Result<()> {
		if self.verbose {
			eprintln!("[xtask pack] writing package.json");
		}

		let package = self.package(runner)?;

		let package_json = json!({
			"name": package.name,
			"version": package.version,
			"license": package.license,
			"type": "module",
			"files": [
				format!("./{}_bg.wasm", package.name),
				format!("./{}_bg.js", package.name),
				format!("./{}.d.ts", package.name),
			],
			"module": format!("./{}_bg.js", package.name),
			"types": format!("./{}.d.ts", package.name),
			"sideEffects": [format!("./{}.js", package.name)],
		});

		let out_dir = runner.resolve(&self.out_dir);
		fs::write(
			out_dir.join("package.json"),
			serde_json::ser::to_vec_pretty(&package_json).unwrap(),
		)?;

		Ok(())
	}

	fn build(&mut self, runner: &Runner) -> anyhow::Result<()> {
		self.cargo_build(runner)?;
		self.wasm_bindgen(runner)?;
		self.wasm_opt(runner)?;
		self.package_json(runner)?;

		if self.verbose {
			eprintln!("[xtask pack] finish");
		}

		Ok(())
	}
}

impl Run for PackCmd {
	fn run(mut self, runner: &Runner) -> anyhow::Result<()> {
		if self.watch {
			let package = self.package(runner)?;

			// first attempt to build one
			clearscreen::clear().ok();
			if self.build(runner).is_err() {
				println!("\x07");
			}

			let mut notifier = watcher::new_debouncer_opt::<
				_,
				watcher::notify::RecommendedWatcher,
			>(
				watcher::Config::default()
					.with_timeout(Duration::from_millis(500))
					.with_batch_mode(true),
				{
					let runner = runner.clone();

					move |event: watcher::DebounceEventResult| {
						clearscreen::clear().ok();
						if event.is_ok() && self.build(&runner).is_err() {
							println!("\x07");
						}
					}
				},
			)?;

			notifier.watcher().watch(
				&package.path,
				watcher::notify::RecursiveMode::Recursive,
			)?;

			loop {
				thread::sleep(Duration::from_secs(10));
			}
		} else {
			self.build(runner)
		}
	}
}

#[derive(clap::Args, Debug)]
struct EnvCmd {}

impl Run for EnvCmd {
	fn run(self, _: &Runner) -> anyhow::Result<()> {
		for (key, value) in env::vars() {
			println!("{key}: {value}");
		}

		Ok(())
	}
}

fn main() -> anyhow::Result<()> {
	let cargo =
		PathBuf::from(env::var_os("CARGO").expect("should be run in cargo"));
	let metadata = fetch_metadata(&cargo)?;
	let runner = Runner::from_metadata(cargo, &metadata);

	let a = App::parse();

	a.command.run(&runner)?;

	Ok(())
}
