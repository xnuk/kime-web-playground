use std::sync::{Arc, Mutex, Weak};

#[repr(transparent)]
pub struct Source<T> {
	data: Arc<Mutex<T>>,
}

impl<T> Source<T> {
	pub fn new(data: T) -> Self {
		Self {
			data: Arc::new(Mutex::new(data)),
		}
	}

	pub fn borrow(&self) -> SourceRef<T> {
		SourceRef {
			data: Arc::downgrade(&self.data),
		}
	}
}

#[repr(transparent)]
pub struct SourceRef<T> {
	data: Weak<Mutex<T>>,
}

impl<T> SourceRef<T> {
	pub fn map<R>(&self, func: impl FnOnce(&mut T) -> R) -> Option<R> {
		self.data
			.upgrade()
			.map(|data| func(&mut data.lock().unwrap()))
	}
}

impl<T> Clone for SourceRef<T> {
	fn clone(&self) -> Self {
		Self {
			data: self.data.clone(),
		}
	}
}
