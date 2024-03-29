use crate::{engine::zone::inventory::INVENTORY_BOX_MARGIN, graphics};
use macroquad::prelude::*;

const BACK_X: f32 = 1100.;
const BACK_Y: f32 = 0.;
const BACK_WIDTH: f32 = 120.;
const BACK_HEIGHT: f32 = 223.;
pub const HELP_TEXT_HEIGHT: f32 = 25.;

const BUTTON_BACKGROUND_X: f32 = 0.;
const BUTTON_BACKGROUND_Y: f32 = 864.;
pub const BUTTON_WIDTH: f32 = 96.;
pub const BUTTON_HEIGHT: f32 = 96.;
pub const BUTTON_MARGIN: f32 = 5.;
const LOADING_X: f32 = 96.;
const LOADING_Y: f32 = 864.;

const HEAVY_ICON_X: f32 = 288.;
const HEAVY_ICON_Y: f32 = 192.;
const HEAVY_ICON_WIDTH: f32 = 32.;
const HEAVY_ICON_HEIGHT: f32 = 32.;

const EQUIP_ICON_X: f32 = 320.;
const EQUIP_ICON_Y: f32 = 192.;
const EQUIP_ICON_WIDTH: f32 = 32.;
const EQUIP_ICON_HEIGHT: f32 = 32.;

const CLOSE_ICON_X: f32 = 608.;
const CLOSE_ICON_Y: f32 = 64.;
const CLOSE_ICON_WIDTH: f32 = 32.;
const CLOSE_ICON_HEIGHT: f32 = 32.;

pub fn draw_close(graphics: &graphics::Graphics) {
    let dest_x = (screen_width() / 2.0) - CLOSE_ICON_WIDTH;
    let dest_y = INVENTORY_BOX_MARGIN - (CLOSE_ICON_HEIGHT * 1.7);
    draw_texture_ex(
        graphics.tileset_texture,
        dest_x,
        dest_y,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                CLOSE_ICON_X + 1.,
                CLOSE_ICON_Y + 1.,
                CLOSE_ICON_WIDTH,
                CLOSE_ICON_HEIGHT,
            )),
            dest_size: Some(Vec2::new(CLOSE_ICON_WIDTH * 2.0, CLOSE_ICON_HEIGHT * 2.0)),
            ..Default::default()
        },
    );
}

pub fn draw_back(
    graphics: &graphics::Graphics,
    dest_x: f32,
    dest_y: f32,
    width: f32,
    height: f32,
) -> bool {
    let height = height + HELP_TEXT_HEIGHT;
    draw_texture_ex(
        graphics.tileset_texture,
        dest_x,
        dest_y,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(BACK_X, BACK_Y, BACK_WIDTH, BACK_HEIGHT)),
            dest_size: Some(Vec2::new(width, height)),
            ..Default::default()
        },
    );

    let (mouse_x, mouse_y) = mouse_position();
    mouse_x > dest_x
        && mouse_x < dest_x + width as f32
        && mouse_y > dest_y
        && mouse_y < dest_y + height as f32
}

pub fn draw_item(
    graphics: &graphics::Graphics,
    tile_id: &str,
    dest_x: f32,
    dest_y: f32,
    quantity: Option<String>,
    draw_heavy_icon: bool,
    draw_is_equip: bool,
) -> bool {
    // Background
    draw_texture_ex(
        graphics.tileset_texture,
        dest_x,
        dest_y,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                BUTTON_BACKGROUND_X,
                BUTTON_BACKGROUND_Y,
                BUTTON_WIDTH,
                BUTTON_HEIGHT,
            )),
            dest_size: Some(Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT)),
            ..Default::default()
        },
    );

    // Tile
    let source = graphics
        .tiles_mapping
        .get(tile_id)
        .expect(&format!("Tile id {} is unknown", tile_id));
    let source_rect = source.to_rect(0);
    draw_texture_ex(
        graphics.tileset_texture,
        dest_x,
        dest_y,
        WHITE,
        DrawTextureParams {
            source: Some(source_rect),
            dest_size: Some(Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT)),
            ..Default::default()
        },
    );

    if let Some(quantity) = quantity {
        draw_circle(dest_x + 15., dest_y + 15., 10., WHITE);
        draw_text(&quantity, dest_x + 10., dest_y + 20., 20., BLACK);
    }

    if draw_heavy_icon {
        draw_texture_ex(
            graphics.tileset_texture,
            dest_x + BUTTON_WIDTH - (HEAVY_ICON_WIDTH) + 10.,
            dest_y + BUTTON_HEIGHT - (HEAVY_ICON_HEIGHT) + 10.,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    HEAVY_ICON_X + 1.,
                    HEAVY_ICON_Y + 1.,
                    HEAVY_ICON_WIDTH,
                    HEAVY_ICON_HEIGHT,
                )),
                ..Default::default()
            },
        );
    }

    if draw_is_equip {
        draw_texture_ex(
            graphics.tileset_texture,
            dest_x - 10.,
            dest_y + BUTTON_HEIGHT - (EQUIP_ICON_HEIGHT) + 10.,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    EQUIP_ICON_X + 1.,
                    EQUIP_ICON_Y + 1.,
                    EQUIP_ICON_WIDTH,
                    EQUIP_ICON_HEIGHT,
                )),
                ..Default::default()
            },
        );
    }

    let (mouse_x, mouse_y) = mouse_position();
    let mouse_hover = mouse_x > dest_x
        && mouse_x < dest_x + BUTTON_WIDTH as f32
        && mouse_y > dest_y
        && mouse_y < dest_y + BUTTON_HEIGHT as f32;

    if mouse_hover {
        draw_rectangle_lines(dest_x, dest_y, BUTTON_WIDTH, BUTTON_HEIGHT, 4.0, BLUE)
    }

    mouse_hover
}

pub fn draw_more(graphics: &graphics::Graphics, dest_x: f32, dest_y: f32) -> bool {
    draw_texture_ex(
        graphics.tileset_texture,
        dest_x,
        dest_y,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(LOADING_X, LOADING_Y, BUTTON_WIDTH, BUTTON_HEIGHT)),
            ..Default::default()
        },
    );

    let (mouse_x, mouse_y) = mouse_position();
    let mouse_hover = mouse_x > dest_x
        && mouse_x < dest_x + BUTTON_WIDTH as f32
        && mouse_y > dest_y
        && mouse_y < dest_y + BUTTON_HEIGHT as f32;

    if mouse_hover {
        draw_rectangle_lines(dest_x, dest_y, BUTTON_WIDTH, BUTTON_HEIGHT, 4.0, BLUE)
    }

    mouse_hover
}
