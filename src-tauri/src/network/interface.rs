use std::sync::Arc;

use once_cell::sync::Lazy;
use pcap::Device;
use serde::Serialize;
use tokio::sync::RwLock;

pub static INTERFACES: Lazy<Arc<RwLock<Vec<NetworkInterface>>>> = Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

#[derive(Serialize, Clone, Debug)]
pub struct NetworkInterface {
  pub index: usize,
  pub name: String,
  pub description: String,
  pub addresses: Vec<String>,
}

/// Функция обновления интерфейсов
pub async fn refresh_interfaces() {
  match Device::list() {
    Ok(devices) => {
      let mut interfaces = Vec::new();

      for (idx, device) in devices.iter().enumerate() {
        let addresses: Vec<String> = device
          .addresses
          .iter()
          .map(|addr| {
            let addr_str = addr.addr.to_string();
            addr_str
          })
          .collect();

        let desc = device.desc.clone().unwrap_or(device.name.clone());

        let interface = NetworkInterface {
          index: idx,
          name: device.name.clone(),
          description: desc,
          addresses,
        };

        interfaces.push(interface);
      }

      let mut guard = INTERFACES.write().await;
      guard.extend(interfaces);
    }
    Err(_) => {},
  }
}
