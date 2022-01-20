use crate::scene;

pub enum MainMessage {
    Quit,
    ChangeScene(Box<dyn scene::Scene>),
}
