package app

import (
	"fmt"
	"strconv"

	"github.com/google/gopacket"
	"github.com/google/gopacket/layers"
)

// Структура информации сетевого пакета
type PacketInfo struct {
	Length     int    `json:"length"`
	LengthStr  string `json:"length_str"`
	Protocol   string `json:"protocol"`
	SrcIP      string `json:"src_ip"`
	DstIP      string `json:"dst_ip"`
	Identified bool   `json:"identified"`
}

// Функция извлечения данных из IPv4 пакета
func (pi *PacketInfo) fromIPv4(layer gopacket.Layer) {
	if l := layer.(*layers.IPv4); l != nil {
		pi.Protocol = l.Protocol.String() + " / IPv4"
		pi.SrcIP = l.SrcIP.String()
		pi.DstIP = l.DstIP.String()
		pi.Identified = true
	}
}

// Функция извлечения данных из IPv6 пакета
func (pi *PacketInfo) fromIPv6(layer gopacket.Layer, packet gopacket.Packet) {
	if layerIPv6 := layer.(*layers.IPv6); layerIPv6 != nil {
		if l := packet.Layer(layers.LayerTypeTCP); l != nil {
			pi.Protocol = "TCP / IPv6"
			pi.SrcIP = layerIPv6.SrcIP.String()
			pi.DstIP = layerIPv6.DstIP.String()
			pi.Identified = true

			return
		}

		if l := packet.Layer(layers.LayerTypeUDP); l != nil {
			pi.Protocol = "UDP / IPv6"
			pi.SrcIP = layerIPv6.SrcIP.String()
			pi.DstIP = layerIPv6.DstIP.String()
			pi.Identified = true

			return
		}
	}
}

// Функция извлечения данных из ICMPv4 пакета
func (pi *PacketInfo) fromICMPv4(layer gopacket.Layer) {
	if l := layer.(*layers.ICMPv4); l != nil {
		pi.Protocol = "ICMPv4"
		pi.Identified = true

		return
	}
}

// Функция извлечения данных из ICMPv6 пакета
func (pi *PacketInfo) fromICMPv6(layer gopacket.Layer) {
	if l := layer.(*layers.ICMPv6); l != nil {
		pi.Protocol = "ICMPv6"
		pi.Identified = true

		return
	}
}

// Функция извлечения данных из DHCPv4 пакета
func (pi *PacketInfo) fromDHCPv4(layer gopacket.Layer) {
	if l := layer.(*layers.DHCPv4); l != nil {
		pi.Protocol = "DHCPv4"
		pi.SrcIP = l.ClientIP.String()
		pi.DstIP = l.NextServerIP.String()
		pi.Identified = true

		return
	}
}

// Функция конвертировки цифровой (байты) длинны пакета в строковую
func getLengthStr(length int) string {
	if length < 1000 {
		return strconv.Itoa(length) + " Б"
	} else if length >= 1000 && length < 1_000_000 {
		return fmt.Sprintf("%.2f", float64(length)/1024.0) + " КБ"
	} else {
		return fmt.Sprintf("%.2f", float64(length)/1_048_576.0) + " МБ"
	}
}

// Функция обработки сетевого пакета
func processPacket(packet gopacket.Packet) PacketInfo {
	length := packet.Metadata().CaptureInfo.Length

	packetInfo := PacketInfo{
		Length:     length,
		LengthStr:  getLengthStr(length),
		Protocol:   "Unknown",
		SrcIP:      "-",
		DstIP:      "-",
		Identified: false,
	}

	for _, layer := range packet.Layers() {
		if layer == nil {
			continue
		}

		switch layer.LayerType() {
		case layers.LayerTypeIPv4:
			packetInfo.fromIPv4(layer)

		case layers.LayerTypeIPv6:
			packetInfo.fromIPv6(layer, packet)

		case layers.LayerTypeICMPv4:
			packetInfo.fromICMPv4(layer)

		case layers.LayerTypeICMPv6:
			packetInfo.fromICMPv6(layer)

		case layers.LayerTypeDHCPv4:
			packetInfo.fromDHCPv4(layer)
		}
	}

	return packetInfo
}
