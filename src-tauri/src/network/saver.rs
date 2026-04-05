use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::network::packet::NetworkPacket;

pub static AUTO_SAVER: Lazy<Arc<RwLock<AutoSaver>>> =
  Lazy::new(|| Arc::new(RwLock::new(AutoSaver::default())));

#[derive(Clone, Default)]
pub struct AutoSaver {
  pub directory: String,
  pub filename: String,
}

impl AutoSaver {
  /// Функция сохранения данных пакета в файл
  pub fn save_packet(&self, packet: &NetworkPacket) {
    if self.directory.is_empty() || self.filename.is_empty() {
      return;
    }

    let dir_path = PathBuf::from(&self.directory);

    if !dir_path.exists() {
      if let Err(_) = fs::create_dir_all(&dir_path) {
        return;
      }
    }

    let file_path = dir_path.join(format!("{}.txt", self.filename));

    let mut file = match OpenOptions::new()
      .create(true)
      .append(true)
      .open(&file_path)
    {
      Ok(f) => f,
      Err(_) => return,
    };

    let timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();

    let data = format!(
      "\n[{}] {} | {} -> {} | {}",
      timestamp, packet.protocol, packet.src_ip, packet.dst_ip, packet.length_str
    );

    let _ = file.write_all(data.as_bytes());
  }
}
