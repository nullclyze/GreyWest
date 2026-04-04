package app

import "strings"

type PacketFilter struct {
	Protocol string
	SrcIp    string
	DstIp    string
}

// Функция применения фильтра сетевых пакетов
func (app *App) ApplyPacketFilter(protocol string, srcIp string, dstIp string) {
	app.Filter = PacketFilter{
		Protocol: protocol,
		SrcIp:    srcIp,
		DstIp:    dstIp,
	}
}

// Функция проверки пакета
func (app *App) CheckPacket(packet PacketInfo) bool {
	filter := app.Filter

	if filter.Protocol != "" && !strings.Contains(strings.ToLower(packet.Protocol), strings.ToLower(filter.Protocol)) {
		return false
	}

	if filter.SrcIp != "" && !strings.Contains(packet.SrcIP, filter.SrcIp) {
		return false
	}

	if filter.DstIp != "" && !strings.Contains(packet.DstIP, filter.DstIp) {
		return false
	}

	return true
}
