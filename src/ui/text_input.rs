#[cfg(target_arch = "wasm32")]
use sapp_jsutils::JsObject;

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn present_singleline_text_input(title: JsObject, value: JsObject);
    fn try_recv_text_input() -> JsObject;
}

#[derive(Clone)]
pub struct TextInputRequest {
    name: String,
    #[allow(dead_code)]
    value: String,
}

impl TextInputRequest {
    #[cfg(target_arch = "wasm32")]
    pub fn new(title: String, name: String, value: String) -> Self {
        unsafe {
            present_singleline_text_input(JsObject::string(&title), JsObject::string(&value))
        };
        Self { name, value }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(_title: String, name: String, value: String) -> Self {
        Self { name, value }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn try_recv(&mut self) -> Option<String> {
        let value_js_obj = unsafe { try_recv_text_input() };

        if value_js_obj.is_nil() == false {
            let mut value = String::new();
            value_js_obj.to_string(&mut value);
            return Some(value);
        }

        None
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn try_recv(&mut self) -> Option<String> {
        return Some(self.value.to_string());
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
