use macroquad::prelude::*;
use quad_net::http_request::{Method, Request, RequestBuilder};

use crate::SERVER_ADDRESS;

#[derive(Clone)]
pub struct Client {
    login: String,
    password: String,
}

impl Client {
    pub fn new(login: String, password: String) -> Self {
        Self { login, password }
    }

    pub fn get_current_character_id_request(login: &str, password: &str) -> Request {
        let basic_auth_value = format!(
            "Basic {}",
            base64::encode(format!("{}:{}", login, password))
        );
        let url = format!("{}/account/current_character_id", SERVER_ADDRESS);
        info!("Check current character id on '{}'", &url);
        RequestBuilder::new(&url)
            .header("Authorization", &basic_auth_value)
            .send()
    }

    fn basic_auth_value(&self) -> String {
        format!(
            "Basic {}",
            base64::encode(format!("{}:{}", &self.login, &self.password))
        )
    }
    fn url_with_query(
        &self,
        url: String,
        query: serde_json::Map<String, serde_json::Value>,
    ) -> String {
        let mut params: Vec<(String, String)> = Vec::new();
        for (key, value) in query.iter() {
            match value {
                serde_json::Value::Number(number) => {
                    params.push((key.to_string(), number.to_string()));
                }
                serde_json::Value::String(str_) => {
                    params.push((key.to_string(), str_.to_string()));
                }
                serde_json::Value::Bool(bool_) => {
                    params.push((key.to_string(), bool_.to_string()));
                }
                serde_json::Value::Null => {}
                _ => {}
            }
        }

        let url = url::Url::parse_with_params(url.as_str(), &params).unwrap();
        String::from(url)
    }

    pub fn get_character_request(&self, id: &str) -> Request {
        let url = format!("{}/character/{}", SERVER_ADDRESS, id);
        info!("Retrieve character from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.basic_auth_value())
            .send()
    }

    pub fn get_zone_request(&self, world_row_i: i32, world_col_i: i32) -> Request {
        let url = format!("{}/zones/{}/{}", SERVER_ADDRESS, world_row_i, world_col_i);
        info!("Retrieve zone from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.basic_auth_value())
            .send()
    }

    pub fn get_characters_request(&self, world_row_i: i32, world_col_i: i32) -> Request {
        let url = format!(
            "{}/zones/{}/{}/characters",
            SERVER_ADDRESS, world_row_i, world_col_i
        );
        info!("Retrieve characters from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.basic_auth_value())
            .send()
    }

    pub fn get_resources_request(&self, world_row_i: i32, world_col_i: i32) -> Request {
        let url = format!(
            "{}/zones/{}/{}/resources",
            SERVER_ADDRESS, world_row_i, world_col_i
        );
        info!("Retrieve resources from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.basic_auth_value())
            .send()
    }

    pub fn get_stuffs_request(&self, world_row_i: i32, world_col_i: i32) -> Request {
        let url = format!(
            "{}/zones/{}/{}/stuff",
            SERVER_ADDRESS, world_row_i, world_col_i
        );
        info!("Retrieve stuffs from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.basic_auth_value())
            .send()
    }

    pub fn get_builds_request(&self, world_row_i: i32, world_col_i: i32) -> Request {
        let url = format!(
            "{}/zones/{}/{}/builds",
            SERVER_ADDRESS, world_row_i, world_col_i
        );
        info!("Retrieve builds from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.basic_auth_value())
            .send()
    }

    pub fn get_quick_action_request(
        &self,
        uuid: &str,
        post_url: &str,
        zone_row_i: Option<i32>,
        zone_col_i: Option<i32>,
    ) -> Request {
        let url = if let (Some(zone_row_i), Some(zone_col_i)) = (zone_row_i, zone_col_i) {
            format!(
                "{}{}&zone_row_i={}&zone_col_i={}&action_uuid={}&quick_action=1",
                SERVER_ADDRESS, post_url, zone_row_i, zone_col_i, uuid,
            )
        } else {
            format!(
                "{}{}&action_uuid={}&quick_action=1",
                SERVER_ADDRESS, post_url, uuid,
            )
        };

        info!("Post quick action with {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.basic_auth_value())
            .method(Method::Post)
            .send()
    }

    pub fn get_description_request(&self, url: String) -> Request {
        let url = format!("{}{}", SERVER_ADDRESS, url);

        info!("Request description on {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.basic_auth_value())
            .method(Method::Post)
            .send()
    }

    pub fn get_description_request_with_data(
        &self,
        url: String,
        data: serde_json::Map<String, serde_json::Value>,
    ) -> Request {
        let url = format!("{}{}", SERVER_ADDRESS, url);

        info!("Request description with data on {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.basic_auth_value())
            .method(Method::Post)
            .body(&serde_json::json!(data).to_string())
            .send()
    }

    pub fn get_description_request_with_query(
        &self,
        url: String,
        data: serde_json::Map<String, serde_json::Value>,
    ) -> Request {
        let url = format!("{}{}", SERVER_ADDRESS, url);
        let url = self.url_with_query(url, data);

        info!("Request description with data on {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.basic_auth_value())
            .method(Method::Post)
            .send()
    }

    pub fn get_inventory_request(&self, id: &str) -> Request {
        let url = format!("{}/character/{}/inventory-data", SERVER_ADDRESS, id);
        info!("Retrieve inventory from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.basic_auth_value())
            .send()
    }
}
