use axum::{
	extract::{
		ws::{Message, WebSocket},
		State, WebSocketUpgrade,
	},
	response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::state::AppState;

#[derive(Clone)]
pub struct WebSocketManager {
	connections: Arc<RwLock<HashMap<String, broadcast::Sender<String>>>>,
}

impl WebSocketManager {
	pub fn new() -> Self {
		Self {
			connections: Arc::new(RwLock::new(HashMap::new())),
		}
	}

	pub async fn add_connection(&self, id: String) -> broadcast::Receiver<String> {
		let (tx, rx) = broadcast::channel(100);
		self.connections.write().await.insert(id, tx);
		rx
	}

	pub async fn remove_connection(&self, id: &str) {
		self.connections.write().await.remove(id);
	}

	pub async fn broadcast(&self, message: String) {
		let connections = self.connections.read().await;
		for tx in connections.values() {
			let _ = tx.send(message.clone());
		}
	}

	pub async fn send_to(&self, id: &str, message: String) {
		let connections = self.connections.read().await;
		if let Some(tx) = connections.get(id) {
			let _ = tx.send(message);
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsMessage {
	pub msg_type: String,
	pub data: serde_json::Value,
}

pub async fn ws_handler(
	ws: WebSocketUpgrade,
	State(state): State<AppState>,
) -> Response {
	ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
	let (mut sender, mut receiver) = socket.split();
	let connection_id = Uuid::new_v4().to_string();

	tracing::info!("WebSocket connected: {}", connection_id);

	let mut rx = state.ws_manager.add_connection(connection_id.clone()).await;

	let mut send_task = tokio::spawn(async move {
		while let Ok(msg) = rx.recv().await {
			if sender.send(Message::Text(msg)).await.is_err() {
				break;
			}
		}
	});

	let connection_id_clone = connection_id.clone();
	let ws_manager = state.ws_manager.clone();
	let mut recv_task = tokio::spawn(async move {
		while let Some(Ok(msg)) = receiver.next().await {
			if let Message::Text(text) = msg {
				// Echo back or process the message
				let response = format!("Echo: {}", text);
				ws_manager.send_to(&connection_id_clone, response).await;
			}
		}
	});

	tokio::select! {
		_ = (&mut send_task) => recv_task.abort(),
		_ = (&mut recv_task) => send_task.abort(),
	};

	state.ws_manager.remove_connection(&connection_id).await;
	tracing::info!("WebSocket disconnected: {}", connection_id);
}
