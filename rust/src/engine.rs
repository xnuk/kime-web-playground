use alloc::string::{String, ToString};
use core::fmt;

use kime_engine_backend::InputEngineBackend;
use kime_engine_backend_hangul::{
	HangulConfig, HangulData, HangulEngine, Layout,
};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::web_keycode::Modifier;

use super::web_keycode::from_code_with_modifiers;

#[inline]
fn print_error<T>(x: Result<T, impl fmt::Display>) -> Result<T, String> {
	x.map_err(|err| err.to_string())
}

#[wasm_bindgen]
pub struct Maemmae {
	engine: HangulEngine,
	config: HangulData,
	buf: String,
}

#[wasm_bindgen]
impl Maemmae {
	#[wasm_bindgen(constructor)]
	pub fn new(yaml: &str, layout: &str) -> Result<Maemmae, String> {
		let config: HangulConfig = print_error(serde_yaml::from_str(yaml))?;
		let layout = print_error(Layout::load_from(layout))?;
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

	pub fn press_key(
		&mut self,
		code: &str,
		modifier: Modifier,
	) -> Option<bool> {
		Some(self.engine.press_key(
			&self.config,
			from_code_with_modifiers(code, modifier)?,
			&mut self.buf,
		))
	}

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
