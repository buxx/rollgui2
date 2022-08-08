use macroquad::prelude::*;

use crate::{engine::zone::resume::CharacterResume, graphics::Graphics};

const START_X: f32 = 200.;
const START_Y: f32 = 1100.;
const WIDTH: f32 = 20.;
const HEIGHT: f32 = 20.;
const DRAW_START_X: f32 = 150.;
const DRAW_START_Y: f32 = 24.;
const MARGIN_RIGHT: f32 = 5.;
const MARGIN_BOTTOM: f32 = 0.;

pub enum ResumeItem {
    Heart,
    Water,
    Food,
    HaveWater,
    HaveFood,
    Shield,
    Follow,
    Follower,
    Ok,
    Ko,
    Clock,
    GoodSmiley,
    NeutralSmiley,
    BadSmiley,
    CriticalSmiley,
    Warning,
    Sleep,
}

impl ResumeItem {
    pub fn source(&self) -> Rect {
        match self {
            ResumeItem::Heart => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 0.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::Water => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 1.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::Food => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 2.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::HaveWater => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 3.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::HaveFood => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 4.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::Shield => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 5.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::Follow => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 6.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::Follower => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 7.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::Ok => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 8.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::Ko => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 9.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::Clock => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 10.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::GoodSmiley => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 11.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::NeutralSmiley => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 12.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::BadSmiley => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 13.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::CriticalSmiley => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 14.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::Warning => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 15.),
                w: WIDTH,
                h: HEIGHT,
            },
            ResumeItem::Sleep => Rect {
                x: START_X,
                y: START_Y + (HEIGHT * 16.),
                w: WIDTH,
                h: HEIGHT,
            },
        }
    }

    pub fn draw_param(&self) -> DrawTextureParams {
        let source = self.source();

        DrawTextureParams {
            source: Some(source),
            ..Default::default()
        }
    }
}

pub fn draw_resume_items(graphics: &Graphics, resume: &CharacterResume) {
    // Health
    draw_item(
        graphics,
        ResumeItem::Heart.draw_param(),
        vec![resume.health.item().draw_param()],
        DRAW_START_X,
        DRAW_START_Y + (HEIGHT * 0.),
    );
    // Tiredness
    draw_item(
        graphics,
        ResumeItem::Sleep.draw_param(),
        resume.tiredness.draw_params(50., 15.),
        DRAW_START_X,
        DRAW_START_Y + MARGIN_BOTTOM + (HEIGHT * 1.),
    );
    // Thirsty
    draw_item(
        graphics,
        ResumeItem::Water.draw_param(),
        resume.thirsty.draw_params(50., 15.),
        DRAW_START_X,
        DRAW_START_Y + MARGIN_BOTTOM + (HEIGHT * 2.),
    );
    // Can eat
    draw_item(
        graphics,
        ResumeItem::Food.draw_param(),
        resume.hungry.draw_params(50., 15.),
        DRAW_START_X,
        DRAW_START_Y + MARGIN_BOTTOM + (HEIGHT * 3.),
    );
    // Can drink
    draw_item(
        graphics,
        ResumeItem::HaveWater.draw_param(),
        vec![resume.can_drink.item().draw_param()],
        DRAW_START_X,
        DRAW_START_Y + MARGIN_BOTTOM + (HEIGHT * 4.),
    );
    // Can eat
    draw_item(
        graphics,
        ResumeItem::HaveFood.draw_param(),
        vec![resume.can_eat.item().draw_param()],
        DRAW_START_X,
        DRAW_START_Y + MARGIN_BOTTOM + (HEIGHT * 5.),
    );
    // Action points
    draw_item(
        graphics,
        ResumeItem::Clock.draw_param(),
        vec![],
        DRAW_START_X,
        DRAW_START_Y + MARGIN_BOTTOM + (HEIGHT * 6.),
    );
    draw_text(
        &format!("{:.2}", resume.action_points),
        DRAW_START_X + WIDTH + MARGIN_RIGHT,
        DRAW_START_Y + MARGIN_BOTTOM + (HEIGHT * 6.) + (HEIGHT - 5.0),
        HEIGHT,
        BLACK,
    );
    // Follow
    draw_item(
        graphics,
        ResumeItem::Follow.draw_param(),
        vec![],
        DRAW_START_X - WIDTH * 3. - MARGIN_RIGHT,
        DRAW_START_Y + MARGIN_BOTTOM + (HEIGHT * 6.),
    );
    draw_text(
        &format!("{}", resume.follow),
        DRAW_START_X - WIDTH * 2. - MARGIN_RIGHT + 3.,
        DRAW_START_Y + MARGIN_BOTTOM + (HEIGHT * 6.) + HEIGHT / 1.3,
        HEIGHT,
        BLACK,
    );
    // Follower
    draw_item(
        graphics,
        ResumeItem::Follower.draw_param(),
        vec![],
        DRAW_START_X - WIDTH * 6. - MARGIN_RIGHT,
        DRAW_START_Y + MARGIN_BOTTOM + (HEIGHT * 6.),
    );
    draw_text(
        &format!("{}", resume.follower),
        DRAW_START_X - WIDTH * 5. - MARGIN_RIGHT + 3.,
        DRAW_START_Y + MARGIN_BOTTOM + (HEIGHT * 6.) + HEIGHT / 1.3,
        HEIGHT,
        BLACK,
    );
}

fn draw_item(
    graphics: &Graphics,
    type_draw_param: DrawTextureParams,
    value_draw_params: Vec<DrawTextureParams>,
    dest_x: f32,
    dest_y: f32,
) {
    draw_texture_ex(
        graphics.tileset_texture,
        dest_x,
        dest_y,
        WHITE,
        type_draw_param,
    );
    for draw_param in value_draw_params {
        draw_texture_ex(
            graphics.tileset_texture,
            dest_x + WIDTH + MARGIN_RIGHT,
            dest_y,
            WHITE,
            draw_param,
        );
    }
}
