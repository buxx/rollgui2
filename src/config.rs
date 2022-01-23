use ini::Ini;

pub struct Config {
    pub server_url: String,
    pub server_login: String,
    pub server_password: String,
}

impl Config {
    pub fn from_config_file(
        config_file_path: String,
        server_login: String,
        server_password: String,
    ) -> Result<Self, String> {
        let config_file_ini = match Ini::load_from_file(config_file_path) {
            Ok(config_file_ini) => config_file_ini,
            Err(e) => return Err(format!("Failed to load config file: {}", e.to_string())),
        };

        let server_url = match config_file_ini.get_from(Some("server"), "url") {
            Some(server_url) => server_url.to_string(),
            None => return Err(format!("Failed to load config file: server.url not found")),
        };

        Ok(Self {
            server_url,
            server_login,
            server_password,
        })
    }
}
