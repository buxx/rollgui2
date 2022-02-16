use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Part {
    pub text: Option<String>,
    pub is_form: bool,
    pub form_action: Option<String>,
    pub form_values_in_query: bool,
    pub submit_label: Option<String>,
    pub items: Vec<Part>,
    pub type_: Option<String>,
    pub label: Option<String>,
    pub name: Option<String>,
    pub is_link: bool,
    pub default_value: Option<String>,
    pub link_group_name: Option<String>,
    pub align: Option<String>,
    pub value: Option<String>,
    pub is_checkbox: bool,
    pub checked: bool,
    pub choices: Option<Vec<String>>,
    pub search_by_str: bool,
    pub classes: Vec<String>,
    pub classes2: Vec<String>,
    pub is_web_browser_link: bool,
    pub columns: u8,
    pub is_column: bool,
    pub colspan: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Description {
    pub type_: String,
    pub origin_url: Option<String>,
    pub title: Option<String>,
    pub items: Vec<Part>,
    pub footer_links: Vec<Part>,
    pub back_url: Option<String>,
    pub back_url_is_zone: bool,
    pub back_to_zone: bool,
    pub image: Option<String>,
    pub image_id: Option<i32>,
    pub image_extension: Option<String>,
    pub is_long_text: bool,
    pub new_character_id: Option<String>,
    pub redirect: Option<String>,
    pub force_back_url: Option<String>,
    pub can_be_back_url: bool,
    // pub request_clicks: Option<RequestClicks>,
    pub footer_with_character_id: Option<String>,
    pub footer_actions: bool,
    pub footer_inventory: bool,
    pub footer_with_build_id: Option<i32>,
    pub footer_with_affinity_id: Option<i32>,
    pub footer_with_business_id: Option<i32>,
    pub illustration_name: Option<String>,
    pub disable_illustration_row: bool,
    pub account_created: bool,
    pub character_ap: Option<String>,
    pub quick_action_response: Option<String>,
    pub action_uuid: Option<String>,
    pub not_enough_ap: bool,
    pub exploitable_success: Option<(i32, i32)>,
    pub is_quick_error: bool,
    pub deposit_success: Option<((i32, i32), Vec<String>)>,
    pub is_grid: bool,
}

impl Description {
    pub fn from_string(description_string: &str) -> Result<Self, String> {
        let description: Description =
            serde_json::from_str(description_string).map_err(|e| e.to_string())?;
        Ok(description)
    }

    pub fn title(&self) -> String {
        self.title.clone().unwrap_or_default()
    }
}

impl Part {
    pub fn label(&self) -> String {
        match &self.label {
            Some(label) => label.clone(),
            None => match &self.text {
                Some(text) => text.clone(),
                None => "__NO_LABEL__".to_string(),
            },
        }
    }
}
