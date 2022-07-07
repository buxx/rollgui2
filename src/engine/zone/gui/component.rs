use crate::event::model::ItemModel;
use macroquad::prelude::*;

const START_X: f32 = 194.;
const START_Y: f32 = 1000.;
const WIDTH: f32 = 7.;
const HEIGHT: f32 = 6.;

#[derive(Debug, Clone)]
pub enum ProgressBarColor {
    Green,
    Yellow,
    Red,
}

impl ProgressBarColor {
    pub fn position(&self) -> f32 {
        match self {
            Self::Green => 1.0,
            Self::Yellow => 2.0,
            Self::Red => 3.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProgressBar {
    percent: f32,
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
                percent: percent,
                color: color_,
                inverted,
            })
        } else {
            Err("no color".to_string())
        }
    }

    pub fn draw_params(&self, width: f32, height: f32) -> Vec<DrawTextureParams> {
        let progress = if self.inverted {
            (100. - self.percent).max(5.)
        } else {
            self.percent.max(5.)
        };
        vec![
            DrawTextureParams {
                source: Some(Rect {
                    x: START_X,
                    y: START_Y,
                    w: WIDTH,
                    h: HEIGHT,
                }),
                dest_size: Some(Vec2::new(width, height)),
                ..Default::default()
            },
            DrawTextureParams {
                source: Some(Rect {
                    x: START_X,
                    y: START_Y + (HEIGHT * self.color.position()),
                    w: WIDTH,
                    h: HEIGHT,
                }),
                dest_size: Some(Vec2::new(width * (progress / 100.), height)),
                ..Default::default()
            },
        ]
    }
}
