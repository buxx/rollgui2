#[cfg(target_arch = "wasm32")]
extern "C" {
    fn _is_mobile() -> bool;
}

#[cfg(target_arch = "wasm32")]
fn __is_mobile() -> bool {
    return unsafe { _is_mobile() };
}

#[cfg(not(target_arch = "wasm32"))]
fn __is_mobile() -> bool {
    false
}

static mut IS_MOBILE: Option<bool> = None;

pub fn is_mobile() -> bool {
    unsafe {
        if let Some(is_mobile_) = &mut IS_MOBILE {
            *is_mobile_
        } else {
            IS_MOBILE = Some(__is_mobile());
            IS_MOBILE.unwrap()
        }
    }
}

pub fn egui_scale() -> f32 {
    if is_mobile() {
        2.0
    } else {
        1.3
    }
}
