use macroquad::prelude::*;
use quad_net::http_request::{Method, Request, RequestBuilder};

use crate::{entity, types::AvatarUuid, SERVER_ADDRESS};

#[derive(Clone)]
pub struct Client {
    pub credentials: Option<(String, String)>,
    pub auth_token: Option<String>,
}

impl Client {
    pub fn with_credentials(login: String, password: String) -> Self {
        Self {
            credentials: Some((login, password)),
            auth_token: None,
        }
    }

    pub fn with_auth_token(auth_token: String) -> Self {
        Self {
            credentials: None,
            auth_token: Some(auth_token),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn error_message_from_http_error(
        http_error: quad_net::http_request::HttpError,
    ) -> Result<String, ()> {
        match http_error {
            quad_net::http_request::HttpError::IOError => Ok(http_error.to_string()),
            quad_net::http_request::HttpError::UreqError(ureq_error) => match ureq_error {
                ureq::Error::Status(_http_code, response) => {
                    let response_body = response.into_string().unwrap_or("".to_string());
                    let response_object = serde_json::from_str::<serde_json::Value>(&response_body)
                        .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
                    match response_object["message"].as_str() {
                        Some(message) => Ok(message.to_string()),
                        None => Ok("Erreur inconnue".to_string()),
                    }
                }
                ureq::Error::Transport(transport) => Ok(format!("Transport error : {}", transport)),
            },
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn description_from_request_data(
        request_result: Result<String, quad_net::http_request::HttpError>,
    ) -> Result<entity::description::Description, String> {
        match request_result {
            Ok(response_body) => {
                // Try to find error json structure. If not, it is not an error but description
                let response_object = serde_json::from_str::<serde_json::Value>(&response_body)
                    .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
                if response_object["message"].as_str().is_some()
                    && response_object["details"].as_object().is_some()
                {
                    // This is an error json
                    match response_object["message"].as_str() {
                        Some(message) => Err(message.to_string()),
                        None => Err("Erreur inconnue".to_string()),
                    }
                } else {
                    entity::description::Description::from_string(&response_body)
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn description_from_request_data(
        request_result: Result<String, quad_net::http_request::HttpError>,
    ) -> Result<entity::description::Description, String> {
        match request_result {
            Ok(description_string) => {
                entity::description::Description::from_string(&description_string)
            }
            Err(http_error) => Err(Self::error_message_from_http_error(http_error)
                .unwrap_or("Erreur inconnue".to_string())),
        }
    }

    // pub fn get_current_character_id_request(login: &str, password: &str) -> Request {
    //     let basic_auth_value = format!(
    //         "Basic {}",
    //         base64::encode(format!("{}:{}", login, password))
    //     );
    //     let url = format!("{}/account/current_character_id", SERVER_ADDRESS);
    //     info!("Check current character id on '{}'", &url);
    //     RequestBuilder::new(&url)
    //         .header("Authorization", &basic_auth_value)
    //         .send()
    // }

    pub fn get_current_character_id_request(&self) -> Request {
        let url = format!("{}/account/current_character_id", SERVER_ADDRESS);
        info!("Check current character id on '{}'", &url);
        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
            .send()
    }

    pub fn get_auth_token_request(login: &str, password: &str) -> Request {
        let basic_auth_value = format!(
            "Basic {}",
            base64::encode(format!("{}:{}", login, password))
        );
        let url = format!("{}/account/auth-token", SERVER_ADDRESS);
        info!("Get auth token on '{}'", &url);
        RequestBuilder::new(&url)
            .header("Authorization", &basic_auth_value)
            .send()
    }

    fn authentification_value(&self) -> String {
        if let Some((login, password)) = &self.credentials {
            return format!(
                "Basic {}",
                base64::encode(format!("{}:{}", login, password))
            );
        } else if let Some(auth_token) = &self.auth_token {
            return format!("Token {}", auth_token);
        }

        panic!("Auth token or credentials is required")
    }
    fn url_with_query(url: String, query: serde_json::Map<String, serde_json::Value>) -> String {
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

    pub fn get_tiles_request(&self) -> Request {
        let url = format!("{}/zones/tiles", SERVER_ADDRESS);
        info!("Retrieve tiles from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
            .send()
    }

    pub fn get_character_request(&self, id: &str) -> Request {
        let url = format!("{}/character/{}", SERVER_ADDRESS, id);
        info!("Retrieve character from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
            .send()
    }

    pub fn get_character_is_dead_request(&self, character_id: &str) -> Request {
        let url = format!("{}/character/{}/dead", SERVER_ADDRESS, character_id);
        info!("Retrieve character from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
            .send()
    }

    pub fn get_zone_request(&self, world_row_i: i32, world_col_i: i32) -> Request {
        let url = format!("{}/zones/{}/{}", SERVER_ADDRESS, world_row_i, world_col_i);
        info!("Retrieve zone from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
            .send()
    }

    pub fn get_characters_request(&self, world_row_i: i32, world_col_i: i32) -> Request {
        let url = format!(
            "{}/zones/{}/{}/characters",
            SERVER_ADDRESS, world_row_i, world_col_i
        );
        info!("Retrieve characters from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
            .send()
    }

    pub fn get_resources_request(&self, world_row_i: i32, world_col_i: i32) -> Request {
        let url = format!(
            "{}/zones/{}/{}/resources",
            SERVER_ADDRESS, world_row_i, world_col_i
        );
        info!("Retrieve resources from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
            .send()
    }

    pub fn get_stuffs_request(&self, world_row_i: i32, world_col_i: i32) -> Request {
        let url = format!(
            "{}/zones/{}/{}/stuff",
            SERVER_ADDRESS, world_row_i, world_col_i
        );
        info!("Retrieve stuffs from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
            .send()
    }

    pub fn get_builds_request(&self, world_row_i: i32, world_col_i: i32) -> Request {
        let url = format!(
            "{}/zones/{}/{}/builds",
            SERVER_ADDRESS, world_row_i, world_col_i
        );
        info!("Retrieve builds from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
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
            .header("Authorization", &self.authentification_value())
            .method(Method::Post)
            .send()
    }

    pub fn get_anonymous_description_request(
        url: &str,
        query: Option<serde_json::Map<String, serde_json::Value>>,
        data: Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Request {
        let url = if let Some(query_) = query {
            Self::url_with_query(format!("{}{}", SERVER_ADDRESS, url), query_)
        } else {
            format!("{}{}", SERVER_ADDRESS, url)
        };

        info!("Request anonymous description on {}", url);

        let mut request = RequestBuilder::new(&url).method(Method::Post);

        if let Some(_data_) = &data {
            request = request
                .body(&serde_json::json!(data).to_string())
                .header("Content-Type", "application/json");
        }

        request.send()
    }

    pub fn get_anonymous_illustrations_names_request() -> Request {
        let url = format!("{}/system/illustrations-names", SERVER_ADDRESS);
        info!("Request anonymous illustrations names {}", url);
        RequestBuilder::new(&url).method(Method::Get).send()
    }

    pub fn get_description_request(
        &self,
        url: String,
        query: Option<serde_json::Map<String, serde_json::Value>>,
        data: Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Request {
        let url = if let Some(query_) = query {
            Self::url_with_query(format!("{}{}", SERVER_ADDRESS, url), query_)
        } else {
            format!("{}{}", SERVER_ADDRESS, url)
        };

        info!("Request description on {}", url);

        let mut request = RequestBuilder::new(&url)
            .method(Method::Post)
            .header("Authorization", &self.authentification_value());

        if let Some(_data_) = &data {
            request = request
                .body(&serde_json::json!(data).to_string())
                .header("Content-Type", "application/json");
        }

        request.send()
    }

    pub fn get_inventory_request(&self, id: &str) -> Request {
        let url = format!("{}/character/{}/inventory-data", SERVER_ADDRESS, id);
        info!("Retrieve inventory from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
            .send()
    }

    pub fn get_look_at_inventory_stuff(&self, character_id: &str, stuff_id: i32) -> Request {
        let url = format!(
            "{}/_describe/character/{}/inventory_look/{}",
            SERVER_ADDRESS, character_id, stuff_id
        );
        info!("Retrieve look at stuff from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
            .method(Method::Post)
            .send()
    }

    pub fn get_look_at_inventory_resource(&self, character_id: &str, resource_id: &str) -> Request {
        let url = format!(
            "{}/_describe/character/{}/resource_look/{}",
            SERVER_ADDRESS, character_id, resource_id
        );
        info!("Retrieve look at resource from {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
            .method(Method::Post)
            .send()
    }

    pub fn get_avatar_request(&self, avatar_uuid: &AvatarUuid) -> Request {
        // NOTE : Should be different than zone_thumb but same used currently ?
        let media_file_name = format!("character_avatar__zone_thumb__{}.png", avatar_uuid);
        let url = format!("{}/media/{}", SERVER_ADDRESS, media_file_name);
        info!("Retrieve avatar media at {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
            .method(Method::Get)
            .send()
    }

    pub fn get_avatar_zone_thumb_request(&self, avatar_uuid: &AvatarUuid) -> Request {
        let media_file_name = format!("character_avatar__zone_thumb__{}.png", avatar_uuid);
        let url = format!("{}/media/{}", SERVER_ADDRESS, media_file_name);
        info!("Retrieve avatar zone thumb media at {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
            .method(Method::Get)
            .send()
    }

    pub fn get_world_as_character_request(&self, character_id: &str) -> Request {
        let url = format!("{}/world/as-character/{}", SERVER_ADDRESS, character_id);
        info!("Retrieve world as character at {}", url);

        RequestBuilder::new(&url)
            .header("Authorization", &self.authentification_value())
            .method(Method::Get)
            .send()
    }
}
