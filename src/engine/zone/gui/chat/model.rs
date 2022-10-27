pub struct Message {
    author_name: String,
    message: String,
}

impl Message {
    pub fn new(author_name: String, message: String) -> Self {
        Self {
            author_name,
            message,
        }
    }

    pub fn author_name(&self) -> &str {
        &self.author_name
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
