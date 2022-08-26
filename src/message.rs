use crate::{
    client::{self, Client},
    description,
    engine::{self, zone::state::ZoneState},
    entity::{self, character::Character},
};

pub enum MainMessage {
    Quit,
    SetRootEngine,
    SetLoadZoneEngine(Client, String),
    SetLoadZoneEngineWithClient(Client, String),
    SetZoneEngine(client::Client, ZoneState),
    SetWorldEngine(client::Client, Character),
    SetLoadDescriptionEngine(
        String,
        Option<serde_json::Map<String, serde_json::Value>>,
        Option<serde_json::Map<String, serde_json::Value>>,
        Option<description::UiDescription>,
        Option<description::UiDescriptionState>,
        Option<client::Client>,
    ),
    SetDescriptionEngine(entity::description::Description, Option<client::Client>),
    SetDescriptionEngineFrom(
        description::UiDescription,
        description::UiDescriptionState,
        Option<client::Client>,
    ),
    SetErrorEngine(String),
    AccountCreated,
    CharacterCreated(Client, String), // login, password, character_id
    SetEngine(Box<dyn engine::Engine>),
    LoadIllustration(String),
    Exit,
}
