use crate::engine;

pub enum MainMessage {
    Quit,
    SetRootEngine,
    SetLoadZoneEngine(String, String, String),
    SetCreateCharacterEngine(String, String),
    SetErrorEngine(String),
    SetEngine(Box<dyn engine::Engine>),
}
