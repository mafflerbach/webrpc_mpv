const { app, BrowserWindow } = require('electron');

// app.disableHardwareAcceleration()
// app.commandLine.appendSwitch('enable-transparent-visuals');

const createWindow = () => {
	let window = new BrowserWindow({
		title: process.env.TITLE || "Media Mate On Screen Display",
		transparent: true,
		show: false
	})

	window.webContents.on("dom-ready", () => {
		window.webContents.executeJavaScript('Locale.default = "' + app.getLocale() + '"; toggleTime()')
	})

	window.once("ready-to-show", () => {
		window.setFullScreen(true) // this automatically shows window
		//window.show()
	})

	window.loadFile('osd.html')

	window.on('closed', () => {
		window = null
	})
}

app.on('ready', () => setTimeout(createWindow, 500));
