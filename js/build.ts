import * as ESBuild from 'esbuild'
import {
	isAbsolute,
	join as pathJoin,
	dirname,
	fromFileUrl,
	resolve as pathResolve,
} from 'std/path/mod.ts'
import { parse as flagParse } from 'std/flags/mod.ts'

const resolve = (path: string) =>
	fromFileUrl(new URL(path, import.meta.resolve('./')))

const { readFile, readTextFile, realPath, mkdir } = Deno

const makeImports = (
	imports: readonly WebAssembly.ModuleImportDescriptor[],
): { readonly head: string; readonly body: string } => {
	const map = Object.create(null) as { [key: string]: string[] }
	imports.forEach(entry => (map[entry.module] ||= []).push(entry.name))
	const entries = Object.entries(map)
	const importEntries = entries.map(([mod, items], index) => {
		const modPath = JSON.stringify(mod)
		const name = '$wasm_import_' + index
		const head = `import * as ${name} from ${modPath}`
		const bodyRaw = items
			.map(key => {
				const id = JSON.stringify(key)
				return `[${id}]: ${name}[${id}]`
			})
			.join(', ')
		const body = `[${modPath}]: {${bodyRaw}}`
		return { head, body }
	})

	const head = importEntries.map(v => v.head).join('\n')
	const body = '{' + importEntries.map(v => v.body).join(', ') + '}'

	return { head, body }
}

const makeExports = (
	exports: readonly WebAssembly.ModuleExportDescriptor[],
): { readonly head: string; readonly body: string } => {
	const entries = exports.map(({ name }) => ({
		head: `export let ${name};`,
		body: `${name} = $exports[${JSON.stringify(name)}];`,
	}))

	return {
		head: entries.map(v => v.head).join('\n'),
		body: entries.map(v => v.body).join('\n'),
	}
}

const generateWasmModule = async (path: string): Promise<string> => {
	const mod = await readFile(path).then(WebAssembly.compile)
	const imp = makeImports(WebAssembly.Module.imports(mod))
	const exp = makeExports(WebAssembly.Module.exports(mod))

	return `
		import $wasm_path from ${JSON.stringify(path)}
		${imp.head}
		${exp.head}
		export const initialized =
			WebAssembly.instantiateStreaming(fetch($wasm_path), ${imp.body})
				.then($wasm => {
					const $exports = $wasm.instance.exports
					${exp.body}
				})
	`
}

// const trace = <T extends unknown>(x: T, comment: string = 'trace'): T => (
// 	console.log(comment, x), x
// )

const wasmLoader: ESBuild.Plugin = {
	name: 'wasm-loader',
	setup(build) {
		const moduleNamespace = 'wasm-module'
		const binaryNamespace = 'wasm-binary'
		build.onResolve({ filter: /\.wasm$/ }, async arg => {
			const { path, resolveDir, namespace } = arg
			if (namespace === moduleNamespace) {
				return {
					namespace: binaryNamespace,
					path,
				}
			}

			if (resolveDir === '') return

			return {
				namespace: moduleNamespace,
				path: await realPath(
					isAbsolute(path) ? path : pathJoin(resolveDir, path),
				),
			}
		})

		build.onLoad({ filter: /.*/, namespace: moduleNamespace }, arg =>
			generateWasmModule(arg.path).then(contents => ({
				contents,
				resolveDir: dirname(arg.path),
			})),
		)

		build.onLoad({ filter: /.*/, namespace: binaryNamespace }, arg =>
			readFile(arg.path).then(contents => ({
				contents,
				loader: 'file',
			})),
		)
	},
}
const trimHtml = (html: string) =>
	html
		.trim()
		.replace(/\r?\n\s*/g, '\n')
		.replace(/>\n</g, '><')

const htmlMinifier: ESBuild.Plugin = {
	name: 'htmlMinifier',
	setup(build) {
		build.onLoad({ filter: /\.html$/ }, async arg => ({
			contents: trimHtml(await readTextFile(arg.path)),
			watchFiles: [arg.path],
			loader: 'copy',
		}))
	},
}

const build = async ({
	outdir,
	port,
	minify = true,
}: {
	outdir: string
	port: number
	minify: boolean
}) => {
	outdir = pathResolve(outdir)
	await mkdir(outdir, { recursive: true })

	const context = await ESBuild.context({
		// always minify html, because of whitespace sensitivity.
		plugins: [wasmLoader, htmlMinifier],
		entryPoints: [resolve('src/index.ts'), resolve('src/index.html')],
		bundle: true,
		outdir,
		format: 'esm',
		target: ['firefox109'],
		platform: 'browser',
		minify,
		charset: 'utf8',
		define: { SERVE: (port != null) + '' },
		assetNames: '[name]',
		loader: { '.html': 'copy' },
	})

	if (port != null) {
		const served = await context.serve({ port })
		console.log(`running at ${served.host}:${served.port}`)
		await context.watch()
		return context.dispose.bind(context)
	} else {
		const result = await context.rebuild()
		const hasError = result.errors.length > 0 || result.warnings.length > 0

		for (const error of result.errors) {
			console.error(error)
		}

		for (const warn of result.warnings) {
			console.warn(warn)
		}

		ESBuild.stop()
		if (hasError) return Promise.reject()
		return () => {}
	}
}

if (import.meta.main) {
	const { outdir, port, minify } = flagParse(Deno.args)
	if (outdir == null) throw 'Specify outdir to build'
	build({
		outdir,
		port,
		minify: minify !== 'false' && minify !== false,
	})
}
