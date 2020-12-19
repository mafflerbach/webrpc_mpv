const { app, BrowserWindow } = require('electron');

// app.disableHardwareAcceleration()
// app.commandLine.appendSwitch('enable-transparent-visuals');

const createWindow = () => {
	let window = new BrowserWindow({
		transparent: true,
	})

	window.setFullScreen(true)
	window.on('closed', () => {
		window = null
	});
// 	window.loadURL('chrome://gpu')
	window.loadFile('osd.html')
}

app.on('ready', () => setTimeout(createWindow, 100));
