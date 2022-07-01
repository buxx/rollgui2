use crate::event::model::ItemModel;

#[derive(Debug, Clone)]
pub enum ProgressBarColor {
    Green,
    Yellow,
    Red,
}

#[derive(Debug, Clone)]
pub struct ProgressBar {
    percent: i32,
    color: ProgressBarColor,
    inverted: bool,
}

impl ProgressBar {
    pub fn from_item(item: &ItemModel) -> Result<Self, String> {
        let percent = if let Some(value) = item.value_float {
            if value >= 0. && value <= 100. {
                value
            } else {
                return Err(format!("value not between 0 and 100 : '{value}'"));
            }
        } else {
            return Err("no value".to_string());
        };

        let mut inverted = false;
        let mut color: Option<ProgressBarColor> = None;

        if item.classes.contains(&"inverted_percent".to_string()) {
            inverted = true;
        }

        if item.classes.contains(&"green".to_string()) {
            color = Some(ProgressBarColor::Green);
        }

        if item.classes.contains(&"yellow".to_string()) {
            color = Some(ProgressBarColor::Yellow);
        }

        if item.classes.contains(&"red".to_string()) {
            color = Some(ProgressBarColor::Red);
        }

        if let Some(color_) = color {
            Ok(Self {
                percent: percent as i32,
                color: color_,
                inverted,
            })
        } else {
            Err("no color".to_string())
        }
    }
}
