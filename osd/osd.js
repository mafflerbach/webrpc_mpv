const { app, BrowserWindow } = require('electron');

// app.disableHardwareAcceleration()
// app.commandLine.appendSwitch('enable-transparent-visuals');

// https://github.com/electron/electron/issues/25153#issuecomment-843688494
app.commandLine.appendSwitch('use-gl', 'desktop')

const createWindow = () => {
	let window = new BrowserWindow({
		title: process.env.TITLE || "Media Mate On Screen Display",
		transparent: true,
		show: false
	})

	window.webContents.on("dom-ready", () => {
		window.webContents.executeJavaScript('OSD.init("' + app.getLocale() + '");')
	})

	window.once("ready-to-show", () => {
		setTimeout(() => {
			window.setFullScreen(true) // this automatically shows window
		}, 500);
	})

	window.loadFile('osd.html')

	window.on('closed', () => {
		window = null
	})
}

app.on('ready', () => setTimeout(createWindow, 500));
