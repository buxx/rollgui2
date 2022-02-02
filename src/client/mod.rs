use macroquad::prelude::*;
use quad_net::http_request::{Request, RequestBuilder};

use crate::SERVER_ADDRESS;

pub struct Client {
    server_address: String,
    login: String,
    password: String,
}

impl Client {
    pub fn new(server_address: String, login: String, password: String) -> Self {
        Self {
            server_address,
            login,
            password,
        }
    }

    pub fn get_current_character_id_request(login: &str, password: &str) -> Request {
        let basic_auth_value = format!(
            "Basic {}",
            base64::encode(format!("{}:{}", login, password))
        );
        let url = &format!("{}/account/current_character_id", SERVER_ADDRESS);
        info!("Check current character id on '{}'", url);
        RequestBuilder::new(url)
            .header("Authorization", &basic_auth_value)
            .send()
    }
}
