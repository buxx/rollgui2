use macroquad::prelude::*;

use crate::BIN_LOCAL_STORAGE;
#[cfg(target_arch = "wasm32")]
use crate::VERSION;

pub fn convert_to_local(pixel_pos: Vec2) -> Vec2 {
    Vec2::new(pixel_pos.x / screen_width(), pixel_pos.y / screen_height()) * 2.0
        - Vec2::new(1.0, 1.0)
}

pub fn mouse_clicked() -> bool {
    is_mouse_button_released(MouseButton::Left)
}

pub fn mouse_pressed() -> bool {
    is_mouse_button_down(MouseButton::Left)
}

// TODO : use to_digit (failed when tried ...)
pub fn char_to_key_code(char_: &char) -> Option<KeyCode> {
    match char_ {
        'A' => Some(KeyCode::A),
        'B' => Some(KeyCode::B),
        'C' => Some(KeyCode::C),
        'D' => Some(KeyCode::D),
        'E' => Some(KeyCode::E),
        'F' => Some(KeyCode::F),
        'G' => Some(KeyCode::G),
        'H' => Some(KeyCode::H),
        'I' => Some(KeyCode::I),
        'J' => Some(KeyCode::J),
        'K' => Some(KeyCode::K),
        'L' => Some(KeyCode::L),
        'M' => Some(KeyCode::M),
        'N' => Some(KeyCode::N),
        'O' => Some(KeyCode::O),
        'P' => Some(KeyCode::P),
        'Q' => Some(KeyCode::Q),
        'R' => Some(KeyCode::R),
        'S' => Some(KeyCode::S),
        'T' => Some(KeyCode::T),
        'U' => Some(KeyCode::U),
        'V' => Some(KeyCode::V),
        'W' => Some(KeyCode::W),
        'X' => Some(KeyCode::X),
        'Y' => Some(KeyCode::Y),
        'Z' => Some(KeyCode::Z),
        _ => None,
    }
}

pub async fn texture_from_cache_or_from_file(file_path: &str) -> Result<Texture2D, String> {
    let storage = match quad_storage::STORAGE.lock() {
        Ok(storage_) => storage_,
        Err(error) => return Err(format!("Storage error : '{}'", error)),
    };
    if let Some(file_as_b64) = storage.get(file_path) {
        let file_as_bytes = match base64::decode(file_as_b64) {
            Ok(file_as_bytes_) => file_as_bytes_,
            Err(error) => return Err(format!("Unable to decode cached file : '{}'", error)),
        };
        Ok(Texture2D::from_file_with_format(&file_as_bytes[..], None))
    } else {
        match load_texture(file_path).await {
            Ok(texture_) => Ok(texture_),
            Err(error) => return Err(format!("Unable to load texture : '{}'", error)),
        }
    }
}

pub async fn bytes_from_cache_or_file(
    file_path: &str,
    cache_if_not_in: bool,
) -> Result<Vec<u8>, String> {
    let storage = &mut quad_storage::STORAGE.lock();
    let storage = match storage {
        Ok(storage_) => storage_,
        Err(error) => return Err(format!("Storage error : '{}'", error)),
    };
    if let Some(file_as_b64) = storage.get(file_path) {
        debug!("Found file in cache : '{}'", file_path);
        let file_as_bytes = match base64::decode(file_as_b64) {
            Ok(file_as_bytes_) => file_as_bytes_,
            Err(error) => return Err(format!("Unable to decode cached file : '{}'", error)),
        };
        Ok(file_as_bytes)
    } else {
        debug!("Load file : '{}'", file_path);
        match load_file(file_path).await {
            Ok(bytes) => {
                if BIN_LOCAL_STORAGE && cache_if_not_in {
                    storage.set(file_path, &base64::encode(bytes.clone()));
                }
                Ok(bytes)
            }
            Err(error) => return Err(format!("Unable to load file : '{}'", error)),
        }
    }
}

pub fn get_remember_me() -> bool {
    let storage = &mut quad_storage::STORAGE.lock();
    let storage = match storage {
        Ok(storage_) => storage_,
        Err(error) => {
            error!("Storage error : '{}'", error);
            return false;
        }
    };

    if let Some(value) = storage.get("__REMEMBER_ME__") {
        if value == "YES" {
            return true;
        }
    }

    return false;
}

pub fn set_remember_me(value: bool) {
    let storage = &mut quad_storage::STORAGE.lock();
    let storage = match storage {
        Ok(storage_) => storage_,
        Err(error) => {
            error!("Storage error : '{}'", error);
            return ();
        }
    };

    let value_ = if value { "YES" } else { "NO" };
    storage.set("__REMEMBER_ME__", value_);
}

pub fn get_auth_token() -> Option<String> {
    let storage = &mut quad_storage::STORAGE.lock();
    let storage = match storage {
        Ok(storage_) => storage_,
        Err(error) => {
            error!("Storage error : '{}'", error);
            return None;
        }
    };

    if let Some(value) = storage.get("__AUTH_TOKEN__") {
        return Some(value);
    }

    return None;
}

pub fn set_auth_token(auth_token: Option<&str>) {
    let storage = &mut quad_storage::STORAGE.lock();
    let storage = match storage {
        Ok(storage_) => storage_,
        Err(error) => {
            error!("Storage error : '{}'", error);
            return ();
        }
    };

    if let Some(auth_token_) = auth_token {
        storage.set("__AUTH_TOKEN__", auth_token_);
    } else {
        storage.remove("__AUTH_TOKEN__")
    }
}

pub fn vname(file_name: &str) -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        file_name.to_string()
    }
    #[cfg(target_arch = "wasm32")]
    {
        format!("{}?v={}", file_name, VERSION)
    }
}

pub fn get_text_center(
    text: &str,
    font: Option<Font>,
    font_size: u16,
    font_scale: f32,
    rotation: f32,
) -> crate::Vec2 {
    let measure = measure_text(text, font, font_size, font_scale);

    let x_center = measure.width / 2.0 * rotation.cos() + measure.height / 2.0 * rotation.sin();
    let y_center = measure.width / 2.0 * rotation.sin() - measure.height / 2.0 * rotation.cos();

    crate::Vec2::new(x_center, y_center)
}
