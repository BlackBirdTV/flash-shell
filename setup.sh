#!/bin/bash

cwd=$(pwd)
cd ~
sudo curl -L --output - "https://github.com/BlackBirdTV/flash-shell/releases/latest/download/flash-shell" > flash-shell
sudo cp flash-shell /usr/bin/flash-shell
sudo chmod 777 /usr/bin/flash-shell
cd $cwd
