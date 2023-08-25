use std::collections::HashMap;

use kime_engine_backend_hangul::{builtin_layouts, HangulData, Layout};
use kime_engine_core::{
	Config, EngineConfig, InputCategory, InputEngine, InputMode, InputResult,
	Key,
};
use serde::Deserialize;

#[derive(Deserialize)]
#[repr(transparent)]
struct HangulLayoutData(HashMap<Key, String>);

impl From<HangulLayoutData> for Layout {
	fn from(value: HangulLayoutData) -> Self {
		Layout::from_items(value.0)
	}
}

#[derive(Deserialize)]
struct RawConfig {
	engine: EngineConfig,
	layouts: HashMap<String, HangulLayoutData>,
}

fn parse_config(config: &str) -> serde_yaml::Result<Config> {
	let RawConfig { engine, layouts } = serde_yaml::from_str(config)?;

	let layouts = layouts.into_iter().map(|(k, v)| (k.into(), v.into()));

	let hangul_data =
		HangulData::new(&engine.hangul, builtin_layouts().chain(layouts));

	let mut config = Config::new(engine);
	config.hangul_data = hangul_data;
	Ok(config)
}

pub struct KimeEngine {
	pub config: Config,
	pub engine: InputEngine,
	pub last_input_result: InputResult,
}

impl KimeEngine {
	pub fn from_str(config: &str) -> serde_yaml::Result<Self> {
		let config = parse_config(config)?;
		let engine = InputEngine::new(&config);

		Ok(Self {
			config,
			engine,
			last_input_result: InputResult::empty(),
		})
	}

	#[inline]
	pub fn press_key(&mut self, key: Key) -> InputResult {
		let result = self.engine.press_key(key, &self.config);
		self.last_input_result = result;
		result
	}

	#[inline]
	pub fn set_input_category(&mut self, category: InputCategory) {
		self.engine.set_input_category(category)
	}

	#[inline]
	pub fn set_input_mode(&mut self, mode: InputMode) -> bool {
		self.engine.set_input_mode(mode)
	}

	#[inline]
	pub fn category(&self) -> InputCategory {
		self.engine.category()
	}

	#[inline]
	pub fn clear_commit(&mut self) {
		self.engine.clear_commit()
	}

	#[inline]
	pub fn clear_preedit(&mut self) {
		self.engine.clear_preedit()
	}

	#[inline]
	pub fn remove_preedit(&mut self) {
		self.engine.remove_preedit()
	}

	#[inline]
	pub fn preedit_str(&mut self) -> &str {
		self.engine.preedit_str()
	}

	#[inline]
	pub fn commit_str(&mut self) -> &str {
		self.engine.commit_str()
	}

	#[inline]
	pub fn reset(&mut self) {
		self.engine.reset();
		self.last_input_result = InputResult::empty();
	}

	#[inline]
	pub fn check_ready(&mut self) -> bool {
		self.engine.check_ready()
	}

	#[inline]
	pub fn end_ready(&mut self) -> InputResult {
		let result = self.engine.end_ready();
		self.last_input_result = result;
		result
	}
}
