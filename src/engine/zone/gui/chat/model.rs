pub struct Message {
    author_name: Option<String>,
    message: String,
    system: bool,
}

impl Message {
    pub fn character(author_name: String, message: String) -> Self {
        Self {
            author_name: Some(author_name),
            message,
            system: false,
        }
    }
    pub fn system(message: String) -> Self {
        Self {
            author_name: None,
            message,
            system: true,
        }
    }

    pub fn message(&self) -> String {
        if let Some(author_name) = &self.author_name {
            format!("{}: {}", author_name.clone(), self.message.clone())
        } else {
            self.message.to_string()
        }
    }
}
