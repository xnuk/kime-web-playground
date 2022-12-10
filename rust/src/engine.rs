use alloc::string::{String, ToString};

use wasm_bindgen::prelude::wasm_bindgen;

use kime_engine_backend::InputEngineBackend;
use kime_engine_backend_hangul::{
	HangulConfig, HangulData, HangulEngine, Layout,
};

use super::web_keycode::from_code_with_modifiers;

#[wasm_bindgen]
pub struct Maemmae {
	engine: HangulEngine,
	config: HangulData,
	buf: String,
}

#[wasm_bindgen]
pub struct MaemmaeResult {
	data: Option<Maemmae>,
	error: Option<String>,
}

#[wasm_bindgen]
impl MaemmaeResult {
	pub fn get_error(&self) -> Option<String> {
		self.error.clone()
	}

	pub fn take_data(self) -> Option<Maemmae> {
		self.data
	}
}

impl From<Result<Maemmae, String>> for MaemmaeResult {
	fn from(value: Result<Maemmae, String>) -> Self {
		match value {
			Ok(x) => MaemmaeResult {
				data: Some(x),
				error: None,
			},
			Err(x) => MaemmaeResult {
				data: None,
				error: Some(x),
			},
		}
	}
}

#[wasm_bindgen]
pub fn maemmae_new(yaml: &str, layout: &str) -> MaemmaeResult {
	Maemmae::new(yaml, layout).map_err(|e| e.to_string()).into()
}

#[wasm_bindgen]
impl Maemmae {
	fn new(yaml: &str, layout: &str) -> Result<Maemmae, serde_yaml::Error> {
		let config: HangulConfig = serde_yaml::from_str(yaml)?;
		let layout = Layout::load_from(layout)?;
		let layouts = [(config.layout.to_string().into(), layout)];

		let engine =
			HangulEngine::new(config.word_commit, config.preedit_johab);
		let config: HangulData = HangulData::new(&config, layouts.into_iter());

		Ok(Self {
			engine,
			config,
			buf: String::new(),
		})
	}

	#[inline]
	pub fn press_key(&mut self, code: &str, modifier: u8) -> bool {
		if let Some(key) = from_code_with_modifiers(code, modifier.into()) {
			self.engine.press_key(&self.config, key, &mut self.buf)
		} else {
			false
		}
	}

	#[inline]
	pub fn clear_preedit(&mut self) {
		self.engine.clear_preedit(&mut self.buf)
	}

	pub fn preedit_str(&mut self) -> String {
		let mut preedit = String::new();
		self.engine.preedit_str(&mut preedit);
		preedit
	}

	pub fn reset(&mut self) {
		self.engine.reset()
	}

	pub fn has_preedit(&self) -> bool {
		self.engine.has_preedit()
	}

	pub fn get_buf(&self) -> String {
		self.buf.clone()
	}
}
