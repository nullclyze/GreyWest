package app

import (
	"context"
	"time"

	"github.com/google/gopacket"
	"github.com/google/gopacket/pcap"
	"github.com/wailsapp/wails/v2/pkg/runtime"
)

type NetworkInterface struct {
	Index       int      `json:"index"`
	Name        string   `json:"name"`
	Description string   `json:"description"`
	Addresses   []string `json:"addresses"`
}

// Функция обновления доступных сетевых интерфейсов
func (app *App) RefreshAvailableInterfaces() {
	app.Interfaces = []NetworkInterface{}

	devices, err := pcap.FindAllDevs()

	if err != nil {
		return
	}

	for i, device := range devices {
		iface := NetworkInterface{
			Index:       i,
			Name:        device.Name,
			Description: device.Description,
			Addresses:   []string{},
		}

		for _, addr := range device.Addresses {
			iface.Addresses = append(iface.Addresses, addr.IP.String())
		}

		app.Interfaces = append(app.Interfaces, iface)
	}

	runtime.EventsEmit(app.Ctx, "refresh-interfaces", app.Interfaces)
}

// Функция запуска сниффинга сетевых пакетов
func (app *App) StartPacketSniffing(index int) {
	app.sniffingMu.Lock()
	defer app.sniffingMu.Unlock()

	if app.sniffingCancel != nil {
		return
	}

	app.sniffingCtx, app.sniffingCancel = context.WithCancel(context.Background())

	app.sniffingWg.Add(1)

	go func() {
		defer app.sniffingWg.Done()

		iface := app.Interfaces[index]

		handle, err := pcap.OpenLive(iface.Name, 1600, false, 5000*time.Millisecond)
		if err != nil {
			return
		}

		defer handle.Close()

		packetSource := gopacket.NewPacketSource(handle, handle.LinkType())
		packets := packetSource.Packets()

		for {
			select {
			case <-app.sniffingCtx.Done():
				return
			case packet, ok := <-packets:
				if !ok {
					return
				}

				packetInfo := processPacket(packet)

				if packetInfo.Identified && app.CheckPacket(packetInfo) {
					app.TotalPacketCount++
					runtime.EventsEmit(app.Ctx, "network-packet", packetInfo)
					app.SavePacket(packetInfo)
				}
			}
		}
	}()
}

// Функция остановки сниффинга пакетов
func (app *App) StopPacketSniffing() {
	app.sniffingMu.Lock()
	defer app.sniffingMu.Unlock()

	if app.sniffingCancel != nil {
		app.sniffingCancel()
		app.sniffingCancel = nil
	}
}
