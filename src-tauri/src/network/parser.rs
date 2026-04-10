use pnet::packet::{
  ethernet::{EtherTypes, EthernetPacket},
  ip::IpNextHeaderProtocols,
  ipv4::Ipv4Packet,
  ipv6::Ipv6Packet,
  udp::UdpPacket,
  Packet,
};

use crate::network::packet::{get_length_str, NetworkPacket};

/// Функция обработки сетевого пакета
pub fn process_packet(data: &[u8]) -> (bool, NetworkPacket) {
  let length = data.len();

  let mut packet_info = NetworkPacket {
    length,
    length_str: get_length_str(length),
    protocol: "Unknown".to_string(),
    src_ip: "-".to_string(),
    dst_ip: "-".to_string(),
  };

  let mut identified = false;

  if let Some(ethernet) = EthernetPacket::new(data) {
    match ethernet.get_ethertype() {
      EtherTypes::Ipv4 => {
        if let Some(ipv4) = Ipv4Packet::new(ethernet.payload()) {
          process_ipv4(&ipv4, &mut packet_info);
          identified = true;
        }
      }
      EtherTypes::Ipv6 => {
        if let Some(ipv6) = Ipv6Packet::new(ethernet.payload()) {
          process_ipv6(&ipv6, &mut packet_info);
          identified = true;
        }
      }
      _ => {}
    }
  }

  (identified, packet_info)
}

/// Функция извлечения данных из IPv4 пакета
fn process_ipv4(ipv4: &Ipv4Packet, packet_info: &mut NetworkPacket) {
  packet_info.src_ip = ipv4.get_source().to_string();
  packet_info.dst_ip = ipv4.get_destination().to_string();

  match ipv4.get_next_level_protocol() {
    IpNextHeaderProtocols::Tcp => {
      packet_info.protocol = "TCP / IPv4".to_string();
    }
    IpNextHeaderProtocols::Udp => {
      if let Some(udp) = UdpPacket::new(ipv4.payload()) {
        if udp.get_source() == 67
          || udp.get_source() == 68
          || udp.get_destination() == 67
          || udp.get_destination() == 68
        {
          packet_info.protocol = "DHCPv4".to_string();
        } else {
          packet_info.protocol = "UDP / IPv4".to_string();
        }
      } else {
        packet_info.protocol = "UDP / IPv4".to_string();
      }
    }
    IpNextHeaderProtocols::Icmp => {
      packet_info.protocol = "ICMPv4".to_string();
    }
    IpNextHeaderProtocols::Igmp => {
      packet_info.protocol = "IGMPv4".to_string();
    }
    _ => {
      packet_info.protocol = format!("{} / IPv4", ipv4.get_next_level_protocol());
    }
  }
}

/// Функция извлечения данных из IPv6 пакета
fn process_ipv6(ipv6: &Ipv6Packet, packet_info: &mut NetworkPacket) {
  packet_info.src_ip = ipv6.get_source().to_string();
  packet_info.dst_ip = ipv6.get_destination().to_string();

  match ipv6.get_next_header() {
    IpNextHeaderProtocols::Tcp => {
      packet_info.protocol = "TCP / IPv6".to_string();
    }
    IpNextHeaderProtocols::Udp => {
      packet_info.protocol = "UDP / IPv6".to_string();
    }
    IpNextHeaderProtocols::Icmp | IpNextHeaderProtocols::Icmpv6  => {
      packet_info.protocol = "ICMPv6".to_string();
    }
    IpNextHeaderProtocols::Igmp => {
      packet_info.protocol = "IGMPv6".to_string();
    }
    _ => {
      packet_info.protocol = format!("{} / IPv6", ipv6.get_next_header());
    }
  }
}
