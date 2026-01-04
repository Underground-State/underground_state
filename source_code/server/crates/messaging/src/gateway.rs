//! WebSocket gateway for real-time messaging

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use ug_auth::JwtService;
use ug_core::UserId;

use crate::routes::MessageResponse;

/// Gateway event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum GatewayEvent {
    #[serde(rename = "READY")]
    Ready { user_id: String, session_id: String },

    #[serde(rename = "MESSAGE_CREATE")]
    MessageCreate(MessageResponse),

    #[serde(rename = "MESSAGE_UPDATE")]
    MessageUpdate(MessageResponse),

    #[serde(rename = "MESSAGE_DELETE")]
    MessageDelete { id: String, channel_id: String },

    #[serde(rename = "PRESENCE_UPDATE")]
    PresenceUpdate { user_id: String, status: String },

    #[serde(rename = "TYPING_START")]
    TypingStart { channel_id: String, user_id: String },
}

/// Client-to-server messages
#[derive(Debug, Deserialize)]
#[serde(tag = "op")]
pub enum ClientMessage {
    #[serde(rename = "0")]
    Identify { token: String },

    #[serde(rename = "1")]
    Heartbeat,

    #[serde(rename = "3")]
    Subscribe { channel_id: String },

    #[serde(rename = "4")]
    Unsubscribe { channel_id: String },

    #[serde(rename = "5")]
    Typing { channel_id: String },
}

/// Server-to-client wrapper
#[derive(Debug, Serialize)]
pub struct ServerMessage {
    pub op: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
    pub d: serde_json::Value,
}

/// Connected client session
struct ClientSession {
    user_id: UserId,
    subscribed_channels: Vec<i64>,
    tx: broadcast::Sender<String>,
}

/// Gateway manages all WebSocket connections
pub struct Gateway {
    sessions: DashMap<String, ClientSession>,
    channel_subscribers: DashMap<i64, Vec<String>>, // channel_id -> session_ids
    jwt_service: Arc<JwtService>,
    broadcast_tx: broadcast::Sender<(i64, String)>, // channel_id, message
}

impl Gateway {
    pub fn new(jwt_service: Arc<JwtService>) -> Self {
        let (broadcast_tx, _) = broadcast::channel(10000);

        Self {
            sessions: DashMap::new(),
            channel_subscribers: DashMap::new(),
            jwt_service,
            broadcast_tx,
        }
    }

    /// Broadcast message to all subscribers of a channel
    pub async fn broadcast_message_create(&self, channel_id: i64, message: &MessageResponse) {
        let event = GatewayEvent::MessageCreate(message.clone());
        let payload = serde_json::to_string(&ServerMessage {
            op: 0,
            t: Some("MESSAGE_CREATE".to_string()),
            d: serde_json::to_value(&event).unwrap(),
        })
        .unwrap();

        let _ = self.broadcast_tx.send((channel_id, payload));
    }

    /// Handle WebSocket upgrade
    pub async fn handle_upgrade(
        State(gateway): State<Arc<Gateway>>,
        ws: WebSocketUpgrade,
    ) -> Response {
        ws.on_upgrade(move |socket| gateway.handle_connection(socket))
    }

    /// Handle individual WebSocket connection
    async fn handle_connection(self: Arc<Self>, socket: WebSocket) {
        let (mut sender, mut receiver) = socket.split();
        let session_id = uuid::Uuid::new_v4().to_string();
        let (tx, mut rx) = broadcast::channel::<String>(100);
        let mut broadcast_rx = self.broadcast_tx.subscribe();

        let mut user_id: Option<UserId> = None;
        let mut subscribed_channels: Vec<i64> = Vec::new();

        // Spawn task to forward messages to client
        let forward_task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    msg = rx.recv() => {
                        if let Ok(msg) = msg {
                            if sender.send(Message::Text(msg)).await.is_err() {
                                break;
                            }
                        }
                    }
                }
            }
        });

        // Handle incoming messages
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                        match client_msg {
                            ClientMessage::Identify { token } => {
                                match self.jwt_service.validate_access_token(&token) {
                                    Ok(claims) => {
                                        user_id = Some(claims.user_id());

                                        // Send READY event
                                        let ready = ServerMessage {
                                            op: 0,
                                            t: Some("READY".to_string()),
                                            d: serde_json::json!({
                                                "user_id": claims.sub,
                                                "session_id": session_id,
                                            }),
                                        };
                                        let _ = tx.send(serde_json::to_string(&ready).unwrap());

                                        // Store session
                                        self.sessions.insert(
                                            session_id.clone(),
                                            ClientSession {
                                                user_id: claims.user_id(),
                                                subscribed_channels: vec![],
                                                tx: tx.clone(),
                                            },
                                        );
                                    }
                                    Err(_) => {
                                        // Send error and close
                                        break;
                                    }
                                }
                            }
                            ClientMessage::Heartbeat => {
                                let ack = ServerMessage {
                                    op: 11,
                                    t: None,
                                    d: serde_json::Value::Null,
                                };
                                let _ = tx.send(serde_json::to_string(&ack).unwrap());
                            }
                            ClientMessage::Subscribe { channel_id } => {
                                if let Ok(cid) = channel_id.parse::<i64>() {
                                    subscribed_channels.push(cid);
                                    self.channel_subscribers
                                        .entry(cid)
                                        .or_insert_with(Vec::new)
                                        .push(session_id.clone());
                                }
                            }
                            ClientMessage::Unsubscribe { channel_id } => {
                                if let Ok(cid) = channel_id.parse::<i64>() {
                                    subscribed_channels.retain(|&c| c != cid);
                                    if let Some(mut subs) = self.channel_subscribers.get_mut(&cid) {
                                        subs.retain(|s| s != &session_id);
                                    }
                                }
                            }
                            ClientMessage::Typing { channel_id } => {
                                if let (Some(uid), Ok(cid)) = (&user_id, channel_id.parse::<i64>()) {
                                    let event = ServerMessage {
                                        op: 0,
                                        t: Some("TYPING_START".to_string()),
                                        d: serde_json::json!({
                                            "channel_id": channel_id,
                                            "user_id": uid.as_i64().to_string(),
                                        }),
                                    };
                                    let _ = self.broadcast_tx.send((cid, serde_json::to_string(&event).unwrap()));
                                }
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => break,
                Err(_) => break,
                _ => {}
            }
        }

        // Cleanup
        self.sessions.remove(&session_id);
        for cid in subscribed_channels {
            if let Some(mut subs) = self.channel_subscribers.get_mut(&cid) {
                subs.retain(|s| s != &session_id);
            }
        }

        forward_task.abort();
    }
}
