pub struct RootState {
    pub hello_text: String,
}

impl RootState {
    pub fn new() -> Self {
        Self {
            hello_text: "Hello, world!".to_string(),
        }
    }
}
