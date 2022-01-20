use crate::engine;

pub enum MainMessage {
    Quit,
    ChangeScene(Box<dyn engine::Engine>),
}
