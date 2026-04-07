use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use once_cell::sync::Lazy;
use pcap::Capture;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use crate::emit::emit::{EmitEvent, EMITTER};
use crate::network::filter::PACKET_FILTER;
use crate::network::interface::INTERFACES;
use crate::network::parser::process_packet;
use crate::network::saver::AUTO_SAVER;

pub static SNIFFING_ACTIVE: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
pub static SNIFFING_HANDLE: Lazy<Arc<RwLock<Option<JoinHandle<()>>>>> =
  Lazy::new(|| Arc::new(RwLock::new(None)));
pub static TOTAL_PACKET_COUNT: Lazy<Arc<RwLock<usize>>> = Lazy::new(|| Arc::new(RwLock::new(0)));

/// Функция запуска сниффинга сетевых пакетов
pub async fn start_packet_sniffing(selected_interface: usize) {
  stop_packet_sniffing().await;

  let handle = tokio::spawn(async move {
    let guard = INTERFACES.read().await;

    if selected_interface >= guard.len() {
      return;
    }

    let interface_name = guard[selected_interface].name.clone();

    drop(guard);

    SNIFFING_ACTIVE.store(true, Ordering::Relaxed);

    let capture = Capture::from_device(interface_name.as_str())
      .unwrap()
      .promisc(false)
      .timeout(5000)
      .open();

    let mut capture = match capture {
      Ok(c) => c,
      Err(_) => {
        SNIFFING_ACTIVE.store(false, Ordering::Relaxed);
        return;
      }
    };

    while SNIFFING_ACTIVE.load(Ordering::Relaxed) {
      match capture.next_packet() {
        Ok(packet) => {
          let packet_info = process_packet(packet.data);

          if !packet_info.identified {
            continue;
          }

          let filter = PACKET_FILTER.read().await;
          if !filter.check_packet(&packet_info) {
            continue;
          }
          drop(filter);

          let mut count = TOTAL_PACKET_COUNT.write().await;
          *count += 1;
          drop(count);

          let saver = AUTO_SAVER.read().await;
          saver.save_packet(&packet_info);
          drop(saver);

          EMITTER.emit(EmitEvent::NetworkPacket(packet_info));
        }
        Err(_) => {
          tokio::time::sleep(Duration::from_millis(50)).await;
        }
      }
    }
  });

  let mut guard = SNIFFING_HANDLE.write().await;
  *guard = Some(handle);
}

/// Функция остановки сниффинга сетевых пакетов
pub async fn stop_packet_sniffing() {
  let mut guard = SNIFFING_HANDLE.write().await;

  if let Some(handle) = guard.as_mut() {
    handle.abort();
    *guard = None;
  }

  if SNIFFING_ACTIVE.load(Ordering::Relaxed) {
    SNIFFING_ACTIVE.store(false, Ordering::Relaxed);
  }
}
