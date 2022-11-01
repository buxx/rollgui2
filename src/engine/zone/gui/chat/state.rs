use super::model::Message;

pub struct State {
    messages: Vec<Message>,
    unread: bool,
    display: bool,
    input_focused: bool,
    request_focus: bool,
    input_value: String,
    mouse_hover: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            messages: vec![],
            unread: false,
            display: false,
            input_focused: false,
            request_focus: false,
            input_value: "".to_string(),
            mouse_hover: false,
        }
    }

    pub fn set_display(&mut self, display: bool) {
        self.display = display;
        if self.display {
            self.unread = false;
            self.request_focus = true;
        } else {
            self.input_focused = false;
        }
    }

    pub fn is_display(&self) -> bool {
        self.display
    }

    pub fn is_input_focused(&self) -> bool {
        self.input_focused
    }

    pub fn is_mouse_hover(&self) -> bool {
        self.mouse_hover
    }

    pub fn have_unread(&self) -> bool {
        self.unread
    }

    pub fn add_message(&mut self, message: Message, silent: bool) {
        self.messages.push(message);
        if !silent {
            self.unread = !self.display;
        }
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn input_value(&self) -> &str {
        &self.input_value
    }

    pub fn reset_input_value(&mut self) {
        self.input_value = "".to_string()
    }

    pub fn set_request_focus(&mut self) {
        self.request_focus = true;
    }

    pub fn request_focus(&self) -> bool {
        self.request_focus
    }

    pub fn update_from_display(&mut self, display_state: DisplayState) {
        self.input_value = display_state.input_value;
        self.input_focused = display_state.input_focused;
        self.mouse_hover = display_state.mouse_hover;
        if display_state.input_focused {
            self.request_focus = false;
        }
    }
}

pub struct DisplayState {
    pub input_focused: bool,
    pub input_validated: bool,
    pub input_value: String,
    pub mouse_hover: bool,
}

impl DisplayState {
    pub fn from_state(state: &State) -> Self {
        Self {
            input_focused: state.input_focused,
            input_validated: false,
            input_value: state.input_value().to_string(),
            mouse_hover: false,
        }
    }
}
