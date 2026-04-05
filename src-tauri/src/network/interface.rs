use pcap::Device;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct NetworkInterface {
  pub index: usize,
  pub name: String,
  pub description: String,
  pub addresses: Vec<String>,
}

/// Функция поиска доступных интерфейсов
pub fn find_interfaces() -> Vec<NetworkInterface> {
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

      interfaces
    }
    Err(_) => Vec::new(),
  }
}
