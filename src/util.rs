use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use macroquad::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub fn convert_to_local(pixel_pos: Vec2) -> Vec2 {
    Vec2::new(pixel_pos.x / screen_width(), pixel_pos.y / screen_height()) * 2.0
        - Vec2::new(1.0, 1.0)
}

pub fn mouse_clicked() -> bool {
    is_mouse_button_released(MouseButton::Left)
}

#[wasm_bindgen(module = "/static/utils.js")]
extern "C" {
    fn js_sleep(millis: u32) -> js_sys::Array;
    fn clear_timeout(id: i32);
}

pub async fn sleep(milliseconds: u32) {
    let tuple = js_sleep(milliseconds);
    let promise = js_sys::Promise::from(tuple.get(0));
    let id = tuple.get(1).as_f64().unwrap() as i32;
    struct Sleep {
        future: JsFuture,
        id: i32,
    }
    impl Future for Sleep {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            Pin::new(&mut self.future).poll(cx).map(|res| {
                res.unwrap();
            })
        }
    }
    impl Drop for Sleep {
        fn drop(&mut self) {
            clear_timeout(self.id);
        }
    }
    Sleep {
        future: JsFuture::from(promise),
        id,
    }
    .await
}
