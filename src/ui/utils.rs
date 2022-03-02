#[cfg(target_arch = "wasm32")]
extern "C" {
    fn _is_mobile() -> bool;
}

#[cfg(target_arch = "wasm32")]
pub fn is_mobile() -> bool {
    return unsafe { _is_mobile() };
}

#[cfg(not(target_arch = "wasm32"))]
pub fn is_mobile() -> bool {
    false
}
