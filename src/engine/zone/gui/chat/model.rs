pub struct Message {
    message: String,
    system: bool,
}

impl Message {
    pub fn character(message: String) -> Self {
        Self {
            message,
            system: false,
        }
    }
    pub fn system(message: String) -> Self {
        Self {
            message,
            system: true,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
