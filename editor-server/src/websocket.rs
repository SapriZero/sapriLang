//! Gestione WebSocket per console live

use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::StreamExt;
use std::sync::Arc;
use tracing::info;

use crate::AppState;

pub async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx = state.output_tx.subscribe();

    loop {
        tokio::select! {
            Some(Ok(msg)) = socket.next() => {
                if let Message::Text(text) = msg {
                    match text.as_str() {
                        "ping" => {
                            let _ = socket.send(Message::Text("pong".to_string())).await;
                        }
                        _ => {
                            info!("Comando non riconosciuto: {}", text);
                        }
                    }
                }
            }

            Ok(msg) = rx.recv() => {
                if socket.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }

            else => break,
        }
    }
}
