import { app, BrowserWindow, Menu } from 'electron';
import { getAssetURL } from 'electron-snowpack';
import path from 'path';

const isDev = process.env.MODE !== 'production';
const isMac = process.platform === 'darwin';

//Create the main window
const createMainWindow = () => {
    const mainWindow = new BrowserWindow({
        title: 'Yet Another Map Parser',
        width: isDev ? 1000 : 800,
        height: 500,
        webPreferences: {
            // nodeIntegration: true,
            // contextIsolation: true,
            enableRemoteModule: true,
            preload: path.join(__dirname, 'preload.js')
        }
    })

    // open devtools if in dev env
    if (isDev) {
        mainWindow.webContents.openDevTools();
    }

    mainWindow.loadURL(getAssetURL('index.html'))
}

// Create about windows
function createAboutWindow() {
    const aboutWindow = new BrowserWindow({
        title: 'About YAMP',
        width: 300,
        height: 300,
        resizable: isDev,
    })

    aboutWindow.loadURL(getAssetURL('about.html'))
}

// App is rady
app.whenReady().then(() => {
    createMainWindow()

    // Implement menu
    const mainMenu = Menu.buildFromTemplate(menu);
    Menu.setApplicationMenu(mainMenu);

    app.on('activate', () => {
        if (BrowserWindow.getAllWindows().length === 0) {
            createMainWindow()
        }
    })
})

// Custom menu
const menu = [
    ...(isMac ? [{
        label: app.name,
        submenu: [{
            label: 'About',
            click: createAboutWindow
        }]
    }] : []),
    {
        role: 'fileMenu',
    },
    ...(!isMac ? [{
        label: 'Help',
        submenu: [{
            label: 'About',
            click: createAboutWindow
        }]
    }] : [])
];

// Check for mac
app.on('window-all-closed', () => {
    if (isMac) {
        app.quit()
    }
})