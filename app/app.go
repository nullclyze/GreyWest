package app

import (
	"context"
	"fmt"
	"strconv"
	"sync"
)

// Структура приложения
type App struct {
	Ctx               context.Context
	Interfaces        []NetworkInterface
	Filter            PacketFilter
	Saver             AutoSaver
	SelectedInterface int
	TotalPacketCount  int
	sniffingCtx       context.Context
	sniffingCancel    context.CancelFunc
	sniffingWg        sync.WaitGroup
	sniffingMu        sync.Mutex
}

// Функция создания нового приложения
func New() *App {
	return &App{}
}

// Функция запуска нужных операция при запуске приложения
func (app *App) Startup(ctx context.Context) {
	app.Ctx = ctx
}

// Функция выбора целевого интерфейса
func (app *App) SelectInterface(index int) {
	app.SelectedInterface = index
}

// Функция получения строки общего счёта пакетов
func (app *App) GetTotalPacketCountStr() string {
	if app.TotalPacketCount < 1000 {
		return strconv.Itoa(app.TotalPacketCount)
	} else if app.TotalPacketCount >= 1000 && app.TotalPacketCount < 1_000_000 {
		return fmt.Sprintf("%.2f", float64(app.TotalPacketCount)/1000.0) + "k"
	} else {
		return fmt.Sprintf("%.2f", float64(app.TotalPacketCount)/1_000_000.0) + "m"
	}
}

// Функция сброса общего счёта пакетов
func (app *App) ResetTotalPacketCount() {
	app.TotalPacketCount = 0
}
