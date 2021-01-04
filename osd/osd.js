const { app, BrowserWindow } = require('electron');

// app.disableHardwareAcceleration()
// app.commandLine.appendSwitch('enable-transparent-visuals');

const createWindow = () => {
	let window = new BrowserWindow({
		title: "MediaMate On Screen Display",
		transparent: true,
		show: false // Avoid flickering (Pt. 1)
	})

	// Avoid flickering (Pt. 2): loadFile before setFullScreen
	window.loadFile('osd.html')
	window.setFullScreen(true)
	window.on('closed', () => {
		window = null
	})
}

app.on('ready', () => setTimeout(createWindow, 100));
