#[cfg(target_arch = "wasm32")]
use sapp_jsutils::JsObject;

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn _is_mobile() -> bool;
    fn _reload_page() -> bool;
    fn _open_url(url: JsObject) -> bool;
    fn _loaded() -> bool;
}

#[cfg(target_arch = "wasm32")]
fn __is_mobile() -> bool {
    return unsafe { _is_mobile() };
}

#[cfg(target_arch = "wasm32")]
pub fn reload_page() {
    return unsafe {
        _reload_page();
    };
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
        1.7
    } else {
        1.4
    }
}

pub fn open_url(url: &str) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        return webbrowser::open(&url).is_ok();
    }
    #[cfg(target_arch = "wasm32")]
    {
        unsafe { _open_url(JsObject::string(url)) }
    }
}

pub fn loaded() -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        return true;
    }
    #[cfg(target_arch = "wasm32")]
    {
        unsafe { _loaded() }
    }
}
