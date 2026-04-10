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
  pub filetype: i32
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

    if self.filetype == 0 || self.filetype == -1 {
      self.save_as_txt(dir_path.join(format!("{}.txt", self.filename)), packet);
    } else if self.filetype == 1 {
      self.save_as_json(dir_path.join(format!("{}.json", self.filename)), packet);
    }
  }

  /// Метод сохранения данных пакета в TXT файл
  fn save_as_txt(&self, path: PathBuf, packet: &NetworkPacket) {
    let mut file = match OpenOptions::new()
      .create(true)
      .append(true)
      .open(&path)
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

  /// Метод сохранения данных пакета в JSON файл
  fn save_as_json(&self, path: PathBuf, packet: &NetworkPacket) {
    let old_data = fs::read_to_string(&path).unwrap_or_else(|_| "[]".to_string());

    let mut v: Vec<NetworkPacket> = serde_json::from_str(&old_data).unwrap_or_default();

    v.push(packet.clone());

    let pretty = serde_json::to_string_pretty(&v).expect("Failed to serialize string");

    let mut file = OpenOptions::new()
      .create(true)
      .write(true)
      .truncate(true)
      .open(&path)
      .expect("Failed to open file");

    file.write_all(pretty.as_bytes()).expect("Failed to write data to file");
  }
}
