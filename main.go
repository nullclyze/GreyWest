package main

import (
	"GreyWest/app"
	"embed"

	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/assetserver"
)

//go:embed all:frontend/dist
var assets embed.FS

func main() {
	app := app.New()

	err := wails.Run(&options.App{
		Title:         "GreyWest",
		Frameless:     false,
		Width:         1100,
		Height:        600,
		DisableResize: true,
		Debug: options.Debug{
			OpenInspectorOnStartup: false,
		},
		AssetServer: &assetserver.Options{
			Assets: assets,
		},
		BackgroundColour: &options.RGBA{R: 20, G: 20, B: 20, A: 1},
		OnStartup:        app.Startup,
		Bind: []interface{}{
			app,
		},
	})

	if err != nil {
		println("Error: ", err.Error())
	}
}
