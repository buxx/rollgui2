pub struct RootState {
    pub first_frame: bool,
    pub login: String,
    pub password: String,
    pub error_message: Option<String>,
}

impl RootState {
    pub fn new(login: &str, password: &str) -> Self {
        Self {
            first_frame: true,
            login: login.to_string(),
            password: password.to_string(),
            error_message: None,
        }
    }
}
