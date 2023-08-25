#[inline]
pub fn boolean(func: impl FnOnce() -> Option<()>) -> bool {
	func().is_some()
}

#[inline]
pub fn utf16(s: &str) -> Vec<u16> {
	s.encode_utf16().collect()
}
