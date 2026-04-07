use std::thread;

use crate::emit::emit::{emit_event_loop, EmitEvent, EMITTER};
use crate::network::filter::{PacketFilter, PACKET_FILTER};
use crate::network::interface::{INTERFACES, refresh_interfaces};
use crate::network::saver::{AutoSaver, AUTO_SAVER};
use crate::network::sniffer::{start_packet_sniffing, stop_packet_sniffing, TOTAL_PACKET_COUNT};

mod emit;
mod network;

/// Функция обновления доступных сетевых интерфейсов
#[tauri::command]
async fn refresh_available_interfaces() {
  refresh_interfaces().await;
  EMITTER.emit(EmitEvent::AvailableInterfaces(INTERFACES.read().await.clone()));
}

/// Команда запуска сниффинга пакетов
#[tauri::command]
async fn start_sniffing(selected_interface: usize) {
  start_packet_sniffing(selected_interface).await;
}

/// Команда остановки сниффинга пакетов
#[tauri::command]
async fn stop_sniffing() {
  stop_packet_sniffing().await;
}

/// Команда применения фильтра пакетов
#[tauri::command]
async fn apply_packet_filter(protocol: String, src_ip: String, dst_ip: String) {
  let mut guard = PACKET_FILTER.write().await;

  *guard = PacketFilter {
    protocol: protocol,
    src_ip: src_ip,
    dst_ip: dst_ip,
  };
}

/// Команда применения авто сохранения
#[tauri::command]
async fn apply_auto_saver(directory: String, filename: String) {
  let mut guard = AUTO_SAVER.write().await;

  *guard = AutoSaver {
    directory: directory,
    filename: filename,
  };
}

/// Команда получения общего счёта пакетов
#[tauri::command]
async fn get_total_packet_count() -> usize {
  let count = TOTAL_PACKET_COUNT.read().await;
  *count
}

/// Команда получения строки общего счёта пакетов
#[tauri::command]
async fn convert_packet_count_to_str(count: usize) -> String {
  if count < 1000 {
    count.to_string()
  } else if count >= 1000 && count < 1_000_000 {
    format!("{:.2}k", count as f64 / 1000.0)
  } else {
    format!("{:.2}m", count as f64 / 1_000_000.0)
  }
}

/// Команда сброса общего счёта пакетов
#[tauri::command]
async fn reset_total_packet_count() {
  let mut count = TOTAL_PACKET_COUNT.write().await;
  *count = 0;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let handle = app.handle().clone();

      thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(emit_event_loop(handle));
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      refresh_available_interfaces,
      start_sniffing,
      stop_sniffing,
      apply_packet_filter,
      apply_auto_saver,
      get_total_packet_count,
      convert_packet_count_to_str,
      reset_total_packet_count
    ])
    .run(tauri::generate_context!())
    .expect("Error while running application");
}
