// src/lib.rs

pub mod handler;
pub mod message;
pub mod server;
pub mod state;

use crate::server::run_server;
use crate::state::AudioState;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Starts the WebSocket server on the given address.
///
/// # Arguments
///
/// * `addr` - The address to bind the server to, e.g., "127.0.0.1:9001".
///
/// # Example
///
/// ```rust
/// use audio_websocket::start_server;
///
/// #[tokio::main]
/// async fn main() {
///     start_server("127.0.0.1:9001").await;
/// }
/// ```
pub async fn start_server(addr: &str) {
    // Initialize logging
    env_logger::init();

    // Initialize shared audio state
    let audio_state = Arc::new(Mutex::new(AudioState::new()));

    // Run the server
    if let Err(e) = run_server(addr, audio_state).await {
        log::error!("Server encountered an error: {}", e);
    }
}

