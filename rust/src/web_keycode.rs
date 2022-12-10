use kime_engine_backend::{Key, KeyCode, ModifierState};
// use wasm_bindgen::prelude::wasm_bindgen;

/// KeyEvent.code to keycode slice, *sorted* by &str.
const CODE_KEYCODE: [(&str, KeyCode); 94] = [
	("AltLeft", KeyCode::AltL),
	("AltRight", KeyCode::AltR),
	("ArrowDown", KeyCode::Down),
	("ArrowLeft", KeyCode::Left),
	("ArrowRight", KeyCode::Right),
	("ArrowUp", KeyCode::Up),
	("Backquote", KeyCode::Grave),
	("Backslash", KeyCode::Backslash),
	("Backspace", KeyCode::Backspace),
	("BracketLeft", KeyCode::OpenBracket),
	("BracketRight", KeyCode::CloseBracket),
	("Comma", KeyCode::Comma),
	("ControlLeft", KeyCode::ControlL),
	("ControlRight", KeyCode::ControlR),
	("Convert", KeyCode::Henkan),
	("Delete", KeyCode::Delete),
	("Digit0", KeyCode::Zero),
	("Digit1", KeyCode::One),
	("Digit2", KeyCode::Two),
	("Digit3", KeyCode::Three),
	("Digit4", KeyCode::Four),
	("Digit5", KeyCode::Five),
	("Digit6", KeyCode::Six),
	("Digit7", KeyCode::Seven),
	("Digit8", KeyCode::Eight),
	("Digit9", KeyCode::Nine),
	("End", KeyCode::End),
	("Enter", KeyCode::Enter),
	("Equal", KeyCode::Equal),
	("Escape", KeyCode::Esc),
	("F1", KeyCode::F1),
	("F10", KeyCode::F10),
	("F11", KeyCode::F11),
	("F12", KeyCode::F12),
	("F2", KeyCode::F2),
	("F3", KeyCode::F3),
	("F4", KeyCode::F4),
	("F5", KeyCode::F5),
	("F6", KeyCode::F6),
	("F7", KeyCode::F7),
	("F8", KeyCode::F8),
	("F9", KeyCode::F9),
	("Home", KeyCode::Home),
	("Insert", KeyCode::Insert),
	("KeyA", KeyCode::A),
	("KeyB", KeyCode::B),
	("KeyC", KeyCode::C),
	("KeyD", KeyCode::D),
	("KeyE", KeyCode::E),
	("KeyF", KeyCode::F),
	("KeyG", KeyCode::G),
	("KeyH", KeyCode::H),
	("KeyI", KeyCode::I),
	("KeyJ", KeyCode::J),
	("KeyK", KeyCode::K),
	("KeyL", KeyCode::L),
	("KeyM", KeyCode::M),
	("KeyN", KeyCode::N),
	("KeyO", KeyCode::O),
	("KeyP", KeyCode::P),
	("KeyQ", KeyCode::Q),
	("KeyR", KeyCode::R),
	("KeyS", KeyCode::S),
	("KeyT", KeyCode::T),
	("KeyU", KeyCode::U),
	("KeyV", KeyCode::V),
	("KeyW", KeyCode::W),
	("KeyX", KeyCode::X),
	("KeyY", KeyCode::Y),
	("KeyZ", KeyCode::Z),
	("Lang1", KeyCode::Hangul),
	("Lang2", KeyCode::HangulHanja),
	("Minus", KeyCode::Minus),
	("NonConvert", KeyCode::Muhenkan),
	("Numpad0", KeyCode::NumZero),
	("Numpad1", KeyCode::NumOne),
	("Numpad2", KeyCode::NumTwo),
	("Numpad3", KeyCode::NumThree),
	("Numpad4", KeyCode::NumFour),
	("Numpad5", KeyCode::NumFive),
	("Numpad6", KeyCode::NumSix),
	("Numpad7", KeyCode::NumSeven),
	("Numpad8", KeyCode::NumEight),
	("Numpad9", KeyCode::NumNine),
	("PageDown", KeyCode::PageDown),
	("PageUp", KeyCode::PageUp),
	("Period", KeyCode::Period),
	("Quote", KeyCode::Quote),
	("Semicolon", KeyCode::SemiColon),
	("ShiftLeft", KeyCode::Shift),
	("ShiftRight", KeyCode::Shift),
	("Slash", KeyCode::Slash),
	("Space", KeyCode::Space),
	("Tab", KeyCode::Tab),
];

#[no_mangle]
pub static CONTROL: u32 = ModifierState::CONTROL.bits();

// #[wasm_bindgen]
pub struct Modifier {
	pub ctrl: bool,
	pub alt: bool,
	pub shift: bool,

	// did u know that you can never use 'super'
	pub win: bool,
}

impl From<Modifier> for ModifierState {
	fn from(
		Modifier {
			ctrl,
			alt,
			shift,
			win,
		}: Modifier,
	) -> Self {
		let mut modifier = ModifierState::empty();

		if ctrl {
			modifier |= ModifierState::CONTROL;
		}
		if alt {
			modifier |= ModifierState::ALT;
		}
		if shift {
			modifier |= ModifierState::SHIFT;
		}
		if win {
			modifier |= ModifierState::SUPER;
		}

		modifier
	}
}

#[inline]
pub fn from_code(code: &str) -> Option<KeyCode> {
	CODE_KEYCODE
		.binary_search_by_key(&code, |&(s, _)| s)
		.ok()
		.and_then(|i| CODE_KEYCODE.get(i))
		.map(|(_, v)| *v)
}

#[inline]
pub fn from_code_with_modifiers(code: &str, modifier: Modifier) -> Option<Key> {
	from_code(code).map(|code| Key::new(code, modifier.into()))
}
