use quad_net::http_request::HttpError;

#[cfg(not(target_arch = "wasm32"))]
pub fn auth_failed(result: &Result<String, HttpError>) -> Result<bool, String> {
    match result {
        Ok(_) => Ok(false),
        Err(error) => match error {
            HttpError::UreqError(ureq::Error::Status(status_code, _)) => {
                if *status_code == 401 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Err(format!("{}", error)),
        },
    }
}

#[cfg(target_arch = "wasm32")]
pub fn auth_failed(result: &Result<String, HttpError>) -> Result<bool, String> {
    match result {
        Ok(data) => {
            if data == "__AUTH_REQUIRED__" {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(error) => Err(format!("{}", error)),
    }
}
