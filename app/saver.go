package app

import (
	"fmt"
	"os"
	"path"
	"time"
)

type AutoSaver struct {
	Directory string
	Filename  string
}

// Функция применения авто сохранения
func (app *App) ApplyAutoSaver(directory string, filename string) {
	app.Saver = AutoSaver{
		Directory: directory,
		Filename:  filename,
	}
}

// Функция сохранения пакета в файл
func (app *App) SavePacket(packet PacketInfo) {
	if app.Saver.Directory == "" || app.Saver.Filename == "" {
		return
	}

	_, err := os.Stat(app.Saver.Directory)
	if os.IsNotExist(err) {
		if err := os.MkdirAll(app.Saver.Directory, 0755); err != nil {
			return
		}
	}

	filePath := path.Join(app.Saver.Directory, app.Saver.Filename+".txt")

	file, err := os.OpenFile(filePath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)

	if err != nil {
		return
	}

	defer file.Close()

	data := fmt.Sprintf("\n[%d] %s | %s -> %s | %s", time.Now().Unix(), packet.Protocol, packet.SrcIP, packet.DstIP, packet.LengthStr)

	file.WriteString(data)
}
