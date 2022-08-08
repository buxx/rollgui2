use macroquad::prelude::*;
use quad_net::http_request::Request;

use crate::{client::Client, message::MainMessage};

use super::Engine;

pub struct CheckCharacterIsDeadEngine {
    character_id: String,
    client: Client,
    check_character_request: Request,
}

impl CheckCharacterIsDeadEngine {
    pub fn new(character_id: String, client: Client) -> Self {
        let check_character_request = client.get_character_is_dead_request(&character_id);
        Self {
            character_id,
            client,
            check_character_request,
        }
    }
}

impl Engine for CheckCharacterIsDeadEngine {
    fn tick(&mut self) -> Vec<MainMessage> {
        if let Some(data) = self.check_character_request.try_recv() {
            info!("Check character is dead response received");
            match data {
                Ok(response_text) => {
                    if response_text == "1" {
                        info!("Character is dead, go to post mortem page");
                        return vec![MainMessage::SetLoadDescriptionEngine(
                            format!("/character/{}/post_mortem", self.character_id),
                            None,
                            None,
                            None,
                            None,
                            Some(self.client.clone()),
                        )];
                    } else {
                        return vec![MainMessage::SetErrorEngine(
                            "Internat error (fail to retrieve character)".to_string(),
                        )];
                    }
                }
                Err(error) => {
                    return vec![MainMessage::SetErrorEngine(error.to_string())];
                }
            }
        };

        egui_macroquad::draw();
        vec![]
    }
}
