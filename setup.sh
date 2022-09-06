cwd=$(pwd)
cd
curl -L -O "https://github.com/BlackBirdTV/flash-shell/releases/latest/download/flash-shell" --output flash-shell
sudo cp flash-shell /usr/bin/flash-shell
sudo chmod 777 /usr/bin/flash-shell
cd $cwd