// SSE event broadcast system
// Replaces Tauri's app.emit() with server-sent events over HTTP

use std::sync::OnceLock;
use tokio::sync::broadcast;

/// A single SSE event
#[derive(Debug, Clone, serde::Serialize)]
pub struct SseEvent {
    pub event: String,
    pub data: String,
}

static EVENT_TX: OnceLock<broadcast::Sender<SseEvent>> = OnceLock::new();

/// Initialize the broadcast channel. Call once at startup.
pub fn init_events() -> broadcast::Sender<SseEvent> {
    let (tx, _) = broadcast::channel(256);
    EVENT_TX.set(tx.clone()).ok();
    tx
}

/// Broadcast an event to all SSE listeners.
/// Non-blocking — if no receivers, event is dropped.
pub fn broadcast(event: impl Into<String>, data: impl serde::Serialize) {
    let event = event.into();
    let data_str = match serde_json::to_string(&data) {
        Ok(s) => s,
        Err(e) => {
            tracing::warn!("Failed to serialize SSE event '{}': {}", event, e);
            return;
        }
    };

    if let Some(tx) = EVENT_TX.get() {
        let _ = tx.send(SseEvent { event, data: data_str });
    }
}

/// Convenience: broadcast a string message without JSON wrapping
pub fn broadcast_str(event: impl Into<String>, msg: impl Into<String>) {
    broadcast(event, msg.into());
}
