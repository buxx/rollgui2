use crate::entity::build::Build;
use crate::tileset::TileId;
use macroquad::prelude::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use serde_derive::{Deserialize as DeserializeDerive, Serialize as SerializeDerive};
use serde_json;
use serde_json::Value;
use std::collections::HashMap;

pub mod model;

pub const PLAYER_MOVE: &str = "PLAYER_MOVE";
pub const CLIENT_WANT_CLOSE: &str = "CLIENT_WANT_CLOSE";
pub const SERVER_PERMIT_CLOSE: &str = "SERVER_PERMIT_CLOSE";
pub const CHARACTER_ENTER_ZONE: &str = "CHARACTER_ENTER_ZONE";
pub const CHARACTER_SPRITESHEET_CHANGE: &str = "CHARACTER_SPRITESHEET_CHANGE";
pub const CHARACTER_EXIT_ZONE: &str = "CHARACTER_EXIT_ZONE";
pub const CLIENT_REQUIRE_AROUND: &str = "CLIENT_REQUIRE_AROUND";
pub const THERE_IS_AROUND: &str = "THERE_IS_AROUND";
pub const CLICK_ACTION_EVENT: &str = "CLICK_ACTION_EVENT";
pub const CLIENT_REQUIRE_NEW_RESUME_TEXT: &str = "CLIENT_REQUIRE_NEW_RESUME_TEXT";
pub const NEW_RESUME_TEXT: &str = "NEW_RESUME_TEXT";
pub const NEW_BUILD: &str = "NEW_BUILD";
pub const REMOVE_BUILD: &str = "REMOVE_BUILD";
pub const REQUEST_CHAT: &str = "REQUEST_CHAT";
pub const NEW_CHAT_MESSAGE: &str = "NEW_CHAT_MESSAGE";
pub const ANIMATED_CORPSE_MOVE: &str = "ANIMATED_CORPSE_MOVE";
pub const TOP_BAR_MESSAGE: &str = "TOP_BAR_MESSAGE";
pub const ZONE_TILE_REPLACE: &str = "ZONE_TILE_REPLACE";
pub const ZONE_GROUND_RESOURCE_REMOVE: &str = "ZONE_GROUND_RESOURCE_REMOVE";
pub const ZONE_GROUND_STUFF_REMOVE: &str = "ZONE_GROUND_STUFF_REMOVE";
pub const ZONE_GROUND_RESOURCE_APPEAR: &str = "ZONE_GROUND_RESOURCE_APPEAR";
pub const ZONE_GROUND_STUFF_APPEAR: &str = "ZONE_GROUND_STUFF_APPEAR";

#[derive(SerializeDerive, DeserializeDerive, Debug)]
#[serde(untagged)]
pub enum TopBarMessageType {
    NORMAL,
    ERROR,
}

// TODO : Must use struct to simplify deserialization
#[derive(SerializeDerive, DeserializeDerive, Debug)]
#[serde(untagged)]
pub enum ZoneEventType {
    // FIXME rename into ClientClosing
    ClientWantClose,
    // FIXME rename into ClientClosingAcknowledge
    ServerPermitClose,
    PlayerMove {
        to_row_i: i32,
        to_col_i: i32,
        character_id: String,
    },
    CharacterEnter {
        zone_row_i: i32,
        zone_col_i: i32,
        character_id: String,
        spritesheet_filename: Option<String>,
    },
    CharacterSpritesheetChange {
        character_id: String,
        spritesheet_filename: String,
    },
    CharacterExit {
        character_id: String,
    },
    ClientRequireResumeText,
    ClientRequireAround {
        zone_row_i: i32,
        zone_col_i: i32,
        character_id: String,
    },
    ThereIsAround {
        stuff_count: i32,
        resource_count: i32,
        build_count: i32,
        character_count: i32,
        quick_actions: Vec<crate::action::quick::QuickAction>,
    },
    ClickActionEvent {
        action_type: String,
        action_description_id: String,
        row_i: i16,
        col_i: i16,
    },
    NewResumeText {
        resume: Vec<model::ItemModel>,
    },
    NewBuild {
        build: Build,
        produced_resource_id: Option<String>,
        produced_stuff_id: Option<String>,
        producer_character_id: Option<String>,
    },
    RemoveBuild {
        zone_row_i: i32,
        zone_col_i: i32,
    },
    RequestChat,
    NewChatMessage {
        character_id: Option<String>,
        message: String,
        system: bool,
        silent: bool,
    },
    AnimatedCorpseMove {
        to_row_i: i32,
        to_col_i: i32,
        animated_corpse_id: i32,
    },
    TopBarMessage {
        message: String,
        type_: TopBarMessageType,
    },
    ZoneTileReplace {
        row_i: i16,
        col_i: i16,
        new_tile_id: TileId,
    },
    ZoneGroundResourceRemoved {
        row_i: i32,
        col_i: i32,
        resource_id: String,
    },
    ZoneGroundStuffRemoved {
        stuff_id: i32,
    },
    ZoneGroundResourceAdded {
        row_i: i32,
        col_i: i32,
        resource_id: String,
    },
    ZoneGroundStuffAdded {
        id_: i32,
        stuff_id: String,
        zone_row_i: i32,
        zone_col_i: i32,
        classes: Vec<String>,
    },
}

#[derive(SerdeSerialize, SerdeDeserialize, Debug)]
pub struct NewChatMessage {
    pub message: String,
    pub character_id: Option<String>,
    pub silent: bool,
    pub system: bool,
}

#[derive(SerdeSerialize, SerdeDeserialize, Debug)]
pub struct CharacterActionLink {
    pub name: String,
    pub link: String,
    pub classes1: Vec<String>,
    pub classes2: Vec<String>,
}

#[derive(Debug)]
pub struct ZoneEvent {
    pub event_type: ZoneEventType,
    pub event_type_name: String,
}

impl ZoneEvent {
    pub fn from_u8(bytes: Vec<u8>) -> Result<ZoneEvent, String> {
        let data_json = String::from_utf8(bytes).unwrap();
        let value: Value = serde_json::from_str(&data_json).unwrap();
        Self::from_value(value)
    }

    // TODO: by hand for now ... how to do automatic ?
    pub fn from_value(value: Value) -> Result<Self, String> {
        let type_ = value["type"].as_str().unwrap();
        let data = value.get("data").unwrap();

        match &type_ {
            &PLAYER_MOVE => Ok(ZoneEvent {
                event_type_name: String::from(PLAYER_MOVE),
                event_type: ZoneEventType::PlayerMove {
                    to_row_i: data["to_row_i"].as_i64().unwrap() as i32,
                    to_col_i: data["to_col_i"].as_i64().unwrap() as i32,
                    character_id: String::from(data["character_id"].as_str().unwrap()),
                },
            }),
            &CLIENT_WANT_CLOSE => Ok(ZoneEvent {
                event_type_name: String::from(CLIENT_WANT_CLOSE),
                event_type: ZoneEventType::ClientWantClose,
            }),
            &SERVER_PERMIT_CLOSE => Ok(ZoneEvent {
                event_type_name: String::from(SERVER_PERMIT_CLOSE),
                event_type: ZoneEventType::ServerPermitClose,
            }),
            &CHARACTER_ENTER_ZONE => {
                let spritesheet_filename =
                    if let Some(spritesheet_filename) = data["spritesheet_filename"].as_str() {
                        Some(spritesheet_filename.to_string())
                    } else {
                        None
                    };
                Ok(ZoneEvent {
                    event_type_name: String::from(CHARACTER_ENTER_ZONE),
                    event_type: ZoneEventType::CharacterEnter {
                        zone_row_i: data["zone_row_i"].as_i64().unwrap() as i32,
                        zone_col_i: data["zone_col_i"].as_i64().unwrap() as i32,
                        character_id: String::from(data["character_id"].as_str().unwrap()),
                        spritesheet_filename,
                    },
                })
            }
            &CHARACTER_SPRITESHEET_CHANGE => Ok(ZoneEvent {
                event_type_name: String::from(CHARACTER_SPRITESHEET_CHANGE),
                event_type: ZoneEventType::CharacterSpritesheetChange {
                    character_id: String::from(data["character_id"].as_str().unwrap()),
                    spritesheet_filename: String::from(
                        data["spritesheet_filename"].as_str().unwrap(),
                    ),
                },
            }),
            &CHARACTER_EXIT_ZONE => Ok(ZoneEvent {
                event_type_name: String::from(CHARACTER_EXIT_ZONE),
                event_type: ZoneEventType::CharacterExit {
                    character_id: String::from(data["character_id"].as_str().unwrap()),
                },
            }),
            &THERE_IS_AROUND => {
                let stuff_count: i32 = data["stuff_count"].as_i64().unwrap() as i32;
                let resource_count: i32 = data["resource_count"].as_i64().unwrap() as i32;
                let build_count: i32 = data["build_count"].as_i64().unwrap() as i32;
                let character_count: i32 = data["character_count"].as_i64().unwrap() as i32;
                let quick_actions: Vec<crate::action::quick::QuickAction> =
                    serde_json::from_value(data["quick_actions"].clone()).unwrap();

                Ok(ZoneEvent {
                    event_type_name: String::from(THERE_IS_AROUND),
                    event_type: ZoneEventType::ThereIsAround {
                        stuff_count,
                        resource_count,
                        build_count,
                        character_count,
                        quick_actions,
                    },
                })
            }
            &NEW_RESUME_TEXT => {
                let list_of_items: model::ListOfItemModel =
                    serde_json::from_value(data.get("resume").unwrap().clone()).unwrap();
                Ok(ZoneEvent {
                    event_type_name: String::from(NEW_RESUME_TEXT),
                    // FIXME BS NOW
                    event_type: ZoneEventType::NewResumeText {
                        resume: list_of_items.items,
                    },
                })
            }
            &NEW_BUILD => {
                let build_data = data["build"].as_object().unwrap();
                let mut classes: Vec<String> = vec![];
                for value in build_data["classes"].as_array().unwrap() {
                    let class = value.as_str().unwrap();
                    classes.push(class.to_string());
                }
                let mut traversable: HashMap<String, bool> = HashMap::new();
                traversable.insert(
                    "WALKING".to_string(),
                    build_data["traversable"]
                        .as_object()
                        .unwrap()
                        .get("WALKING")
                        .unwrap()
                        .as_bool()
                        .unwrap(),
                );

                let produced_resource_id = match data["produced_resource_id"].as_str() {
                    Some(produced_resource_id) => Some(produced_resource_id.to_string()),
                    None => None,
                };
                let produced_stuff_id = match data["produced_stuff_id"].as_str() {
                    Some(produced_stuff_id) => Some(produced_stuff_id.to_string()),
                    None => None,
                };
                let producer_character_id = match data["producer_character_id"].as_str() {
                    Some(producer_character_id) => Some(producer_character_id.to_string()),
                    None => None,
                };

                Ok(ZoneEvent {
                    event_type_name: String::from(NEW_BUILD),
                    event_type: ZoneEventType::NewBuild {
                        build: Build {
                            id: build_data["id"].as_i64().unwrap() as i32,
                            build_id: build_data["build_id"].as_str().unwrap().to_string(),
                            row_i: build_data["row_i"].as_i64().unwrap() as i32,
                            col_i: build_data["col_i"].as_i64().unwrap() as i32,
                            classes,
                            traversable,
                            is_floor: build_data["is_floor"].as_bool().unwrap(),
                            under_construction: build_data["under_construction"].as_bool().unwrap(),
                        },
                        produced_resource_id,
                        produced_stuff_id,
                        producer_character_id,
                    },
                })
            }
            &REMOVE_BUILD => Ok(ZoneEvent {
                event_type_name: String::from(REMOVE_BUILD),
                event_type: ZoneEventType::RemoveBuild {
                    zone_row_i: data["zone_row_i"].as_i64().unwrap() as i32,
                    zone_col_i: data["zone_col_i"].as_i64().unwrap() as i32,
                },
            }),
            &ZONE_TILE_REPLACE => {
                let new_tile_id = data["new_tile_id"].as_str().unwrap();
                let zone_row_i = data["zone_row_i"].as_i64().unwrap() as i16;
                let zone_col_i = data["zone_col_i"].as_i64().unwrap() as i16;

                Ok(ZoneEvent {
                    event_type_name: String::from(ZONE_TILE_REPLACE),
                    event_type: ZoneEventType::ZoneTileReplace {
                        row_i: zone_row_i,
                        col_i: zone_col_i,
                        new_tile_id: new_tile_id.to_string(),
                    },
                })
            }
            &NEW_CHAT_MESSAGE => {
                let new_chat_message: NewChatMessage =
                    serde_json::from_value(data.clone()).unwrap();
                Ok(ZoneEvent {
                    event_type_name: String::from(NEW_CHAT_MESSAGE),
                    event_type: ZoneEventType::NewChatMessage {
                        message: new_chat_message.message,
                        character_id: new_chat_message.character_id,
                        system: new_chat_message.system,
                        silent: new_chat_message.silent,
                    },
                })
            }
            &ANIMATED_CORPSE_MOVE => Ok(ZoneEvent {
                event_type_name: String::from(ANIMATED_CORPSE_MOVE),
                event_type: ZoneEventType::AnimatedCorpseMove {
                    to_row_i: data["to_row_i"].as_i64().unwrap() as i32,
                    to_col_i: data["to_col_i"].as_i64().unwrap() as i32,
                    animated_corpse_id: data["animated_corpse_id"].as_i64().unwrap() as i32,
                },
            }),
            &TOP_BAR_MESSAGE => Ok(ZoneEvent {
                event_type_name: String::from(TOP_BAR_MESSAGE),
                event_type: ZoneEventType::TopBarMessage {
                    message: data["message"].as_str().unwrap().to_string(),
                    type_: match data["type_"].as_str().unwrap() {
                        "ERROR" => TopBarMessageType::ERROR,
                        _ => TopBarMessageType::NORMAL,
                    },
                },
            }),
            &ZONE_GROUND_RESOURCE_REMOVE => Ok(ZoneEvent {
                event_type_name: String::from(ZONE_GROUND_RESOURCE_REMOVE),
                event_type: ZoneEventType::ZoneGroundResourceRemoved {
                    row_i: data["zone_row_i"].as_i64().unwrap() as i32,
                    col_i: data["zone_col_i"].as_i64().unwrap() as i32,
                    resource_id: data["resource_id"].as_str().unwrap().to_string(),
                },
            }),
            &ZONE_GROUND_STUFF_REMOVE => Ok(ZoneEvent {
                event_type_name: String::from(ZONE_GROUND_STUFF_REMOVE),
                event_type: ZoneEventType::ZoneGroundStuffRemoved {
                    stuff_id: data["stuff_id"].as_i64().unwrap() as i32,
                },
            }),
            &ZONE_GROUND_RESOURCE_APPEAR => Ok(ZoneEvent {
                event_type_name: String::from(ZONE_GROUND_RESOURCE_APPEAR),
                event_type: ZoneEventType::ZoneGroundResourceAdded {
                    row_i: data["zone_row_i"].as_i64().unwrap() as i32,
                    col_i: data["zone_col_i"].as_i64().unwrap() as i32,
                    resource_id: data["resource_id"].as_str().unwrap().to_string(),
                },
            }),
            &ZONE_GROUND_STUFF_APPEAR => Ok(ZoneEvent {
                event_type_name: String::from(ZONE_GROUND_STUFF_APPEAR),
                event_type: ZoneEventType::ZoneGroundStuffAdded {
                    id_: data["id"].as_i64().unwrap() as i32,
                    stuff_id: data["stuff_id"].as_str().unwrap().to_string(),
                    zone_row_i: data["zone_row_i"].as_i64().unwrap() as i32,
                    zone_col_i: data["zone_col_i"].as_i64().unwrap() as i32,
                    classes: data["classes"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|x| x.as_str().unwrap().to_string())
                        .collect(),
                },
            }),
            _ => Err(format!("Unknown event {}", &type_)),
        }
    }
}

impl Serialize for ZoneEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ZoneEvent", 2)?;
        state.serialize_field("type", &self.event_type_name)?;
        state.serialize_field("data", &self.event_type)?;
        state.end()
    }
}
