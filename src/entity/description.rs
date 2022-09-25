use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestClicks {
    pub action_type: String,
    pub action_description_id: String,
    pub cursor_classes: Vec<String>,
    pub many: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Part {
    pub text: Option<String>,
    pub is_form: bool,
    pub form_action: Option<String>,
    pub form_values_in_query: bool,
    pub submit_label: Option<String>,
    pub items: Vec<Part>,
    pub type_: Option<String>,
    pub expect_integer: bool,
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
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
    pub cost: Option<f32>,
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
    pub request_clicks: Option<RequestClicks>,
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
    pub reload_zone: bool,
    pub reload_inventory: bool,
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
            None => "".to_string(),
        }
    }

    pub fn is_text(&self) -> bool {
        let is_choice = if let Some(choices) = &self.choices {
            choices.len() != 0
        } else {
            false
        };
        (self.text.is_some() || self.label.is_some())
            && !self.is_link
            && self.type_.is_none()
            && !self.is_checkbox
            && !is_choice
    }

    pub fn is_form(&self) -> bool {
        self.is_form
    }

    pub fn is_input(&self) -> bool {
        self.name.is_some() && self.type_.is_some()
    }

    pub fn is_link(&self) -> bool {
        self.is_link && self.form_action.is_some()
    }

    pub fn is_checkbox(&self) -> bool {
        self.is_checkbox
    }

    pub fn is_choices(&self) -> bool {
        self.choices.is_some() && !self.search_by_str
    }

    pub fn is_search_by_str(&self) -> bool {
        self.choices.is_some() && self.search_by_str
    }

    pub fn analyze_default_value(&self) -> Option<(String, Option<String>)> {
        match &self.default_value {
            Some(default_value_) => {
                let suffix = if default_value_.ends_with("l") {
                    Some("l".to_string())
                } else if default_value_.ends_with("kg") {
                    Some("kg".to_string())
                } else if default_value_.ends_with("g") {
                    Some("g".to_string())
                } else if default_value_.ends_with("m³") {
                    Some("m³".to_string())
                } else if default_value_.ends_with("u") {
                    Some("u".to_string())
                } else {
                    None
                };
                Some((default_value_.clone(), suffix))
            }
            None => None,
        }
    }
}

pub enum InputType {
    String,
    Numeric,
}
