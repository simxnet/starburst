use std::sync::Arc;
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::Mutex;
use crate::http::Http;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};
use futures_util::{StreamExt, SinkExt};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
use tokio::sync::mpsc;

type EventHandler = Arc<Mutex<dyn FnMut(Value) + Send>>;

pub struct Gateway {
    ws_url: String,
    tx: UnboundedSender<WsMessage>,
    rx: UnboundedReceiver<WsMessage>,
    http: Http,
    event_handlers: Arc<Mutex<HashMap<String, Vec<EventHandler>>>>,
}

impl Gateway {
    pub async fn connect(token: &str) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let (ws_stream, _) = connect_async("wss://gateway.discord.gg/?v=10&encoding=json").await.expect("Failed to connect");

        let (_, read) = ws_stream.split();

        tokio::spawn(read.for_each(|message| {
            let tx = tx.clone();
            async move {
                if let Ok(msg) = message {
                    tx.send(msg).unwrap();
                }
            }
        }));

        let ws_url = format!("{}?v=10&encoding=json", "wss://gateway.discord.gg");

        Gateway {
            ws_url,
            tx,
            rx,
            http: Http::new(token.to_string()),
            event_handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn send_message(&self, message: &str) {
        if let Err(e) = self.tx.send(WsMessage::Text(message.to_string())) {
            eprintln!("Failed to send message: {:?}", e);
        }
    }

    pub async fn receive_message(&mut self) -> Option<WsMessage> {
        self.rx.recv().await
    }

    pub fn http(&self) -> &Http {
        &self.http
    }

    pub async fn handle_events(&mut self) {
        while let Some(message) = self.receive_message().await {
            match message {
                WsMessage::Text(text) => {
                    if let Ok(value) = serde_json::from_str::<Value>(&text) {
                        if let Some(op) = value.get("op").and_then(|v| v.as_u64()) {
                            match op {
                                0 => {
                                    if let Some(event_type) = value.get("t").and_then(|v| v.as_str()) {
                                        let handlers = self.event_handlers.lock().await;
                                        if let Some(handlers) = handlers.get(event_type) {
                                            for handler in handlers {
                                                let mut handler = handler.lock().await;
                                                handler(value.clone());
                                            }
                                        }
                                    }
                                },
                                _ => {},
                            }
                        }
                    }
                },
                _ => {},
            }
        }
    }

    pub async fn on<F>(&self, event: &str, handler: F)
    where
        F: FnMut(Value) + Send + 'static,
    {
        let mut handlers = self.event_handlers.lock().await;
        let entry = handlers.entry(event.to_string()).or_insert_with(Vec::new);
        entry.push(Arc::new(Mutex::new(handler)));
    }
}