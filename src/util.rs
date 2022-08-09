use macroquad::prelude::*;

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
        'P' => Some(KeyCode::P),
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
                if cache_if_not_in {
                    storage.set(file_path, &base64::encode(bytes.clone()));
                }
                Ok(bytes)
            }
            Err(error) => return Err(format!("Unable to load file : '{}'", error)),
        }
    }
}
