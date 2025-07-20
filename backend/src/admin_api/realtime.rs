use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::Response,
    Extension,
};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use std::sync::Arc;
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    Mutex,
};

use super::client_events::ClientEvent;

#[derive(Debug, Clone)]
pub struct RealtimeState {
    broadcast_tx: Arc<Mutex<Sender<ClientEvent>>>,
}
impl RealtimeState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel::<ClientEvent>(32);
        Self {
            broadcast_tx: Arc::new(Mutex::new(tx)),
        }
    }

    pub async fn broadcast_app_events(&self, events: Vec<ClientEvent>) {
        for event in events {
            self.broadcast_app_event(event).await;
        }
    }

    pub async fn broadcast_app_event(&self, event: ClientEvent) {
        match self.broadcast_tx.lock().await.send(event.clone()) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Failed to send event: {:?}", event);
            }
        }
    }
}

pub async fn handler(
    ws: WebSocketUpgrade,
    Extension(realtime_state): Extension<RealtimeState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, realtime_state))
}

async fn handle_socket(ws: WebSocket, realtime_state: RealtimeState) {
    let (ws_tx, ws_rx) = ws.split();
    let ws_tx = Arc::new(Mutex::new(ws_tx));

    {
        let broadcast_rx = realtime_state.broadcast_tx.lock().await.subscribe();
        tokio::spawn(async move {
            recv_broadcast(ws_tx, broadcast_rx).await;
        });
    }

    recv_from_client(ws_rx).await;
}

async fn recv_from_client(mut client_rx: SplitStream<WebSocket>) {
    while let Some(Ok(msg)) = client_rx.next().await {
        if matches!(msg, Message::Close(_)) {
            return;
        }

        println!("Received message from client: {:?}", msg);

        // if broadcast_tx.lock().await.send(msg).is_err() {
        //     println!("Failed to broadcast a message");
        // }
    }
}

async fn recv_broadcast(
    client_tx_mutex: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    mut broadcast_rx: Receiver<ClientEvent>,
) {
    while let Ok(msg) = broadcast_rx.recv().await {
        let mut client_tx = client_tx_mutex.lock().await;

        if client_tx.send(message_from_event(&msg)).await.is_err() {
            return; // disconnected.
        }
    }
}

fn message_from_event(event: &ClientEvent) -> Message {
    let json = serde_json::to_string(event).expect("Failed to serialize event");
    Message::Text(json.into())
}
