#!/bin/bash

# adduser node audio
# adduser node video

cd app

npm install
npx electron-rebuild

sudo chown root node_modules/electron/dist/chrome-sandbox
sudo chmod 4755 node_modules/electron/dist/chrome-sandbox

# if [ ! -d "/var/run/dbus" ]; then
#     sudo mkdir -p /var/run/dbus 
#     sudo dbus-daemon --config-file=/usr/share/dbus-1/system.conf --print-address
# fi