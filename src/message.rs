use crate::{description, engine, entity};

pub enum MainMessage {
    Quit,
    SetRootEngine,
    SetLoadZoneEngine(String, String, String),
    SetCreateCharacterEngine(String, String),
    SetLoadDescriptionEngine(
        String,
        Option<serde_json::Map<String, serde_json::Value>>,
        Option<serde_json::Map<String, serde_json::Value>>,
        Option<description::UiDescription>,
        Option<description::UiDescriptionState>,
    ),
    SetDescriptionEngine(entity::description::Description),
    SetDescriptionEngineFrom(description::UiDescription, description::UiDescriptionState),
    SetErrorEngine(String),
    SetEngine(Box<dyn engine::Engine>),
}
