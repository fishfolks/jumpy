//! This module holds the networking core, used

use hecs::World;

pub use network_core::{Backend, Lobby, LobbyId, Player, PlayerId, DEFAULT_PORT};

#[cfg(feature = "ultimate")]
pub use ultimate::UltimateBackend as Api;

#[cfg(not(feature = "ultimate"))]
pub use network_core::MockBackend as Api;

#[allow(dead_code)]
pub struct NetworkClient {
    port: u16,
    host_id: PlayerId,
}

impl NetworkClient {
    pub fn new<P: Into<Option<u16>>>(port: P, host_id: &PlayerId) -> Self {
        let port = port.into().unwrap_or(DEFAULT_PORT);

        NetworkClient {
            port,
            host_id: host_id.clone(),
        }
    }
}

pub fn update_network_client(_world: &mut World) {}

pub fn fixed_update_network_client(_world: &mut World) {}

#[allow(dead_code)]
pub struct NetworkHost {
    port: u16,
}

impl NetworkHost {
    pub fn new<P: Into<Option<u16>>>(port: P) -> Self {
        let port = port.into().unwrap_or(DEFAULT_PORT);

        NetworkHost { port }
    }
}

pub fn update_network_host(_world: &mut World) {}

pub fn fixed_update_network_host(_world: &mut World) {}
