use std::sync::Arc;

use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::network::packet::NetworkPacket;

pub static PACKET_FILTER: Lazy<Arc<RwLock<PacketFilter>>> =
  Lazy::new(|| Arc::new(RwLock::new(PacketFilter::default())));

#[derive(Clone, Default)]
pub struct PacketFilter {
  pub protocol: String,
  pub src_ip: String,
  pub dst_ip: String,
}

impl PacketFilter {
  /// Метод проверки пакета по фильтру
  pub fn check_packet(&self, packet: &NetworkPacket) -> bool {
    if !self.protocol.is_empty()
      && !packet
        .protocol
        .to_lowercase()
        .contains(&self.protocol.to_lowercase())
    {
      return false;
    }

    if !self.src_ip.is_empty() && !packet.src_ip.contains(&self.src_ip) {
      return false;
    }

    if !self.dst_ip.is_empty() && !packet.dst_ip.contains(&self.dst_ip) {
      return false;
    }

    true
  }
}
