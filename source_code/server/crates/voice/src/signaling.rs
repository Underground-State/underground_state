//! Voice signaling for WebRTC

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::Response,
};
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

/// Voice channel participant
#[derive(Debug, Clone)]
pub struct Participant {
    pub user_id: i64,
    pub session_id: String,
    pub muted: bool,
    pub deafened: bool,
}

/// Voice channel state
struct VoiceChannel {
    participants: Vec<Participant>,
    tx: broadcast::Sender<String>,
}

/// Voice signaling server
pub struct VoiceSignaling {
    channels: DashMap<i64, VoiceChannel>,
}

impl VoiceSignaling {
    pub fn new() -> Self {
        Self {
            channels: DashMap::new(),
        }
    }

    /// Handle WebSocket upgrade for voice
    pub async fn handle_upgrade(
        State(signaling): State<Arc<VoiceSignaling>>,
        Path((guild_id, channel_id)): Path<(i64, i64)>,
        ws: WebSocketUpgrade,
    ) -> Response {
        ws.on_upgrade(move |socket| signaling.handle_connection(guild_id, channel_id, socket))
    }

    async fn handle_connection(self: Arc<Self>, _guild_id: i64, channel_id: i64, socket: WebSocket) {
        let (mut sender, mut receiver) = socket.split();
        let session_id = uuid::Uuid::new_v4().to_string();

        // Get or create channel
        let channel = self.channels.entry(channel_id).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            VoiceChannel {
                participants: Vec::new(),
                tx,
            }
        });

        let mut rx = channel.tx.subscribe();
        drop(channel);

        // Forward messages task
        let forward_task = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        });

        // Handle incoming signaling messages
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(signal) = serde_json::from_str::<SignalingMessage>(&text) {
                        self.handle_signal(channel_id, &session_id, signal).await;
                    }
                }
                Ok(Message::Close(_)) => break,
                Err(_) => break,
                _ => {}
            }
        }

        // Cleanup
        if let Some(mut channel) = self.channels.get_mut(&channel_id) {
            channel.participants.retain(|p| p.session_id != session_id);
        }

        forward_task.abort();
    }

    async fn handle_signal(&self, channel_id: i64, session_id: &str, signal: SignalingMessage) {
        if let Some(channel) = self.channels.get(&channel_id) {
            match signal {
                SignalingMessage::Offer { sdp, target } => {
                    let response = serde_json::json!({
                        "type": "offer",
                        "from": session_id,
                        "sdp": sdp,
                    });
                    let _ = channel.tx.send(serde_json::to_string(&response).unwrap());
                }
                SignalingMessage::Answer { sdp, target } => {
                    let response = serde_json::json!({
                        "type": "answer",
                        "from": session_id,
                        "sdp": sdp,
                    });
                    let _ = channel.tx.send(serde_json::to_string(&response).unwrap());
                }
                SignalingMessage::IceCandidate { candidate, target } => {
                    let response = serde_json::json!({
                        "type": "ice",
                        "from": session_id,
                        "candidate": candidate,
                    });
                    let _ = channel.tx.send(serde_json::to_string(&response).unwrap());
                }
                SignalingMessage::Join { user_id } => {
                    // Notify others
                    let response = serde_json::json!({
                        "type": "join",
                        "user_id": user_id,
                        "session_id": session_id,
                    });
                    let _ = channel.tx.send(serde_json::to_string(&response).unwrap());
                }
                SignalingMessage::Leave => {
                    let response = serde_json::json!({
                        "type": "leave",
                        "session_id": session_id,
                    });
                    let _ = channel.tx.send(serde_json::to_string(&response).unwrap());
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum SignalingMessage {
    #[serde(rename = "offer")]
    Offer { sdp: String, target: Option<String> },

    #[serde(rename = "answer")]
    Answer { sdp: String, target: Option<String> },

    #[serde(rename = "ice")]
    IceCandidate {
        candidate: serde_json::Value,
        target: Option<String>,
    },

    #[serde(rename = "join")]
    Join { user_id: i64 },

    #[serde(rename = "leave")]
    Leave,
}
