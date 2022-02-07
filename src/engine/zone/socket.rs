use macroquad::prelude::*;
use quad_net::web_socket::WebSocket;

use crate::SERVER_ADDRESS;

pub fn get_socket(state: &super::state::ZoneState) -> Result<WebSocket, String> {
    let ws_url = get_url(state);
    info!("Connect web socket at {}", ws_url);

    #[cfg(not(target_arch = "wasm32"))]
    match WebSocket::connect(&ws_url) {
        Ok(socket_) => Ok(socket_),
        Err(error) => Err(format!("Erreur de connexion web socket : {:?}", error)),
    }

    #[cfg(target_arch = "wasm32")]
    match WebSocket::connect(&ws_url) {
        Ok(socket_) => Ok(socket_),
        Err(error) => Err(format!("Erreur de connexion web socket : {:?}", error)),
    }
}

fn get_url(state: &super::state::ZoneState) -> String {
    format!(
        "{}/ws/zones/{}/{}/events?character_id={}",
        SERVER_ADDRESS
            .replace("http://", "ws://")
            .replace("https://", "wss://"),
        state.player.world_row_i,
        state.player.world_col_i,
        state.player.id,
    )
}
