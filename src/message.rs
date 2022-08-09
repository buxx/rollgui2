use crate::{
    client, description,
    engine::{self, zone::state::ZoneState},
    entity,
};

pub enum MainMessage {
    Quit,
    SetRootEngine,
    SetLoadZoneEngine(String, String, String),
    SetZoneEngine(client::Client, ZoneState),
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
    CharacterCreated(String, String, String), // login, password, character_id
    SetEngine(Box<dyn engine::Engine>),
    LoadIllustration(String),
}
