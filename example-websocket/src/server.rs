use crate::handler::{handle_command, handle_parameter};
use crate::message::{ClientMessage, ServerMessage, AckData, ErrorData};
use crate::state::AudioState;
use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Error as WsError;

/// Runs the WebSocket server.
///
/// # Arguments
///
/// * `addr` - The address to bind the server to.
/// * `audio_state` - Shared audio state.
pub async fn run_server(
    addr: &str,
    audio_state: Arc<Mutex<AudioState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(&addr).await?;
    info!("WebSocket server is listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let audio_state = audio_state.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, audio_state).await {
                error!("Error handling connection: {}", e);
            }
        });
    }
}

/// Handles an individual WebSocket connection.
///
/// # Arguments
///
/// * `stream` - The incoming TCP stream.
/// * `audio_state` - Shared audio state.
async fn handle_connection(
    stream: tokio::net::TcpStream,
    audio_state: Arc<Mutex<AudioState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = stream.peer_addr()?;
    info!("Incoming TCP connection from: {}", addr);

    let ws_stream = accept_async(stream).await?;
    info!("WebSocket connection established: {}", addr);

    let (mut write, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        let message = match message {
            Ok(msg) => msg,
            Err(e) => {
                if matches!(e, WsError::ConnectionClosed) {
                    info!("Connection closed");
                } else {
                    error!("Error receiving message from {}: {}", addr, e);
                }
                break;
            }
        };

        if message.is_text() {
            let text = message.into_text()?;
            info!("Received a message from {}: {}", addr, text);

            match serde_json::from_str::<ClientMessage>(&text) {
                Ok(client_msg) => {
                    match client_msg {
                        ClientMessage::Command(cmd) => {
                            handle_command(cmd, audio_state.clone()).await;
                        }
                        ClientMessage::Parameter(param) => {
                            handle_parameter(param, audio_state.clone()).await;
                        }
                    }

                    let ack = ServerMessage::Ack(AckData {
                        status: "Received".to_string(),
                    });
                    let ack_text = serde_json::to_string(&ack)?;
                    write
                        .send(tokio_tungstenite::tungstenite::Message::Text(ack_text))
                        .await?;
                }
                Err(e) => {
                    error!("Failed to deserialize message from {}: {}", addr, e);

                    // Send error message back to the client
                    let error_msg = ServerMessage::Error(ErrorData {
                        message: "Invalid message format".to_string(),
                    });
                    let error_text = serde_json::to_string(&error_msg)?;
                    write
                        .send(tokio_tungstenite::tungstenite::Message::Text(error_text))
                        .await?;
                }
            }
        } else {
            info!("Received non-text message from {}", addr);
        }
    }

    info!("Connection closed: {}", addr);
    Ok(())
}
