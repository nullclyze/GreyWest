use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct NetworkPacket {
  pub length: usize,
  pub length_str: String,
  pub protocol: String,
  #[serde(rename = "srcIp")]
  pub src_ip: String,
  #[serde(rename = "dstIp")]
  pub dst_ip: String,
  pub identified: bool,
}

impl Default for NetworkPacket {
  fn default() -> Self {
    Self {
      length: 0,
      length_str: String::new(),
      protocol: "Unknown".to_string(),
      src_ip: "-".to_string(),
      dst_ip: "-".to_string(),
      identified: false,
    }
  }
}

/// Функция конвертации длины пакета в строку
pub fn get_length_str(length: usize) -> String {
  if length < 1000 {
    format!("{} Б", length)
  } else if length >= 1000 && length < 1_000_000 {
    format!("{:.2} КБ", length as f64 / 1024.0)
  } else {
    format!("{:.2} МБ", length as f64 / 1_048_576.0)
  }
}
