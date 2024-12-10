pub trait Save {
    fn save(self);
}
impl<S: ToString> Save for S {
    fn save(self) {
        #[cfg(not(feature = "benchmarking"))]
        {
            use arboard::Clipboard;
            println!("Saved result: {}", self.to_string());
            Clipboard::new()
                .unwrap()
                .set_text(self.to_string())
                .unwrap();
        }
    }
}
