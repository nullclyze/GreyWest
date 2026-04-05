use once_cell::sync::Lazy;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast;

use crate::network::{interface::NetworkInterface, packet::NetworkPacket};

pub static EMITTER: Lazy<Arc<EmitManager>> = Lazy::new(|| Arc::new(EmitManager::new()));

pub struct EmitManager {
  events: broadcast::Sender<EmitEvent>,
}

#[derive(Clone)]
pub enum EmitEvent {
  AvailableInterfaces(Vec<NetworkInterface>),
  NetworkPacket(NetworkPacket),
}

impl EmitManager {
  pub fn new() -> Self {
    let (tx, _) = broadcast::channel(100);
    Self { events: tx }
  }

  pub fn emit(&self, event: EmitEvent) {
    let _ = self.events.send(event);
  }
}

/// Цикл событий эмиттера
pub async fn emit_event_loop(handle: AppHandle) {
  let mut rx = EMITTER.events.subscribe();

  while let Ok(event) = rx.recv().await {
    match event {
      EmitEvent::AvailableInterfaces(payload) => {
        let _ = handle.emit("refresh-interfaces", payload);
      }
      EmitEvent::NetworkPacket(payload) => {
        let _ = handle.emit("network-packet", payload);
      }
    }
  }
}
