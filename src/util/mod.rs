pub mod cast;
pub use cast::*;

use arboard::Clipboard;

pub trait Save {
	fn save(self);
}
impl<S: ToString> Save for S {
	fn save(self) {
		println!("Saved result: {}", self.to_string());
		Clipboard::new()
			.unwrap()
			.set_text(self.to_string())
			.unwrap();
	}
}
