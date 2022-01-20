use crate::scene;

pub enum MainMessage {
    Quit,
    ChangeScene(scene::Scene),
}
