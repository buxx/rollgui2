use crate::{engine, graphics};
use macroquad::prelude::*;

const TOP_PANEL_X: f32 = 960.;
const TOP_PANEL_Y: f32 = 256.;
const TOP_PANEL_WIDTH: f32 = 250.;
const TOP_PANEL_HEIGHT: f32 = 42.;
const MIDDLE_PANEL_X: f32 = 960.;
const MIDDLE_PANEL_Y: f32 = 298.;
const MIDDLE_PANEL_WIDTH: f32 = 250.;
const MIDDLE_PANEL_HEIGHT: f32 = 165.;
const BOTTOM_PANEL_X: f32 = 960.;
const BOTTOM_PANEL_Y: f32 = 464.;
const BOTTOM_PANEL_WIDTH: f32 = 250.;
const BOTTOM_PANEL_HEIGHT: f32 = 42.;

const START_DRAWING_BUTTONS_X: f32 = 26.5;
const START_DRAWING_BUTTONS_y: f32 = 175.;

const BUTTON_BACKGROUND_X: f32 = 0.;
const BUTTON_BACKGROUND_Y: f32 = 864.;
const BUTTON_WIDTH: f32 = 96.;
const BUTTON_HEIGHT: f32 = 96.;
const BUTTON_MARGIN: f32 = 5.;

const LOADING_X: f32 = 96.;
const LOADING_Y: f32 = 864.;
const ACTION_BUTTON_X: f32 = 0.;
const ACTION_BUTTON_Y: f32 = 960.;
const WORLD_BUTTON_X: f32 = 0.;
const WORLD_BUTTON_Y: f32 = 1056.;

#[derive(Debug, Clone, PartialEq)]
pub enum Button {
    Actions,
    World,
}

pub enum ButtonAction {
    OpenDescription(String),
    // SetEngine(Box<dyn engine::Engine>),
}

impl Button {
    pub fn action(&self, state: &engine::zone::state::ZoneState) -> ButtonAction {
        match self {
            Button::Actions => ButtonAction::OpenDescription(format!(
                "/_describe/character/{}/main_actions",
                state.player.id
            )),
            Button::World => todo!(),
        }
    }
}

pub fn draw_panel_background(graphics: &graphics::Graphics) {
    draw_texture_ex(
        graphics.tileset_texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                TOP_PANEL_X,
                TOP_PANEL_Y,
                TOP_PANEL_WIDTH,
                TOP_PANEL_HEIGHT,
            )),
            ..Default::default()
        },
    );
    let height_count = (screen_height() / MIDDLE_PANEL_HEIGHT as f32) as i32 + 1;
    for i in 0..height_count {
        draw_texture_ex(
            graphics.tileset_texture,
            0.,
            TOP_PANEL_HEIGHT + (MIDDLE_PANEL_HEIGHT as f32 * i as f32) as f32,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    MIDDLE_PANEL_X,
                    MIDDLE_PANEL_Y,
                    MIDDLE_PANEL_WIDTH,
                    MIDDLE_PANEL_HEIGHT,
                )),
                ..Default::default()
            },
        );
    }
    draw_texture_ex(
        graphics.tileset_texture,
        0.,
        screen_height() - BOTTOM_PANEL_HEIGHT as f32,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                BOTTOM_PANEL_X,
                BOTTOM_PANEL_Y,
                BOTTOM_PANEL_WIDTH,
                BOTTOM_PANEL_HEIGHT,
            )),
            ..Default::default()
        },
    );
}

pub fn draw_buttons(graphics: &graphics::Graphics, loading: &Option<Button>) -> Option<Button> {
    let buttons = [
        (
            Button::Actions,
            Rect::new(
                ACTION_BUTTON_X,
                ACTION_BUTTON_Y,
                BUTTON_WIDTH,
                BUTTON_HEIGHT,
            ),
        ),
        (
            Button::World,
            Rect::new(WORLD_BUTTON_X, WORLD_BUTTON_Y, BUTTON_WIDTH, BUTTON_HEIGHT),
        ),
    ];
    let mut hover_button = None;

    for (i, (button, button_source_rect)) in buttons.iter().enumerate() {
        let row_i = i / 2;
        let col_i = i % 2;

        let draw_to_x =
            START_DRAWING_BUTTONS_X + ((BUTTON_WIDTH as f32 + BUTTON_MARGIN) * col_i as f32);
        let draw_to_y =
            START_DRAWING_BUTTONS_y + ((BUTTON_HEIGHT as f32 + BUTTON_MARGIN) * row_i as f32);

        // Draw background
        draw_texture_ex(
            graphics.tileset_texture,
            draw_to_x,
            draw_to_y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    BUTTON_BACKGROUND_X,
                    BUTTON_BACKGROUND_Y,
                    BUTTON_WIDTH,
                    BUTTON_HEIGHT,
                )),
                ..Default::default()
            },
        );

        let is_loading = if let Some(loading_button) = loading {
            loading_button == button
        } else {
            false
        };

        // Draw button icon
        if !is_loading {
            draw_texture_ex(
                graphics.tileset_texture,
                draw_to_x,
                draw_to_y,
                WHITE,
                DrawTextureParams {
                    source: Some(*button_source_rect),
                    ..Default::default()
                },
            );
        } else {
            draw_texture_ex(
                graphics.tileset_texture,
                draw_to_x,
                draw_to_y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(LOADING_X, LOADING_Y, BUTTON_WIDTH, BUTTON_HEIGHT)),
                    ..Default::default()
                },
            );
        }

        let (mouse_x, mouse_y) = mouse_position();
        if mouse_x > draw_to_x
            && mouse_x < draw_to_x + BUTTON_WIDTH as f32
            && mouse_y > draw_to_y
            && mouse_y < draw_to_y + BUTTON_HEIGHT as f32
        {
            hover_button = Some(button.clone());
        }
    }

    hover_button
}