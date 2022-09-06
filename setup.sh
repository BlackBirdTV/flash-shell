cwd=$(pwd)
cd
curl -L -O "https://github.com/BlackBirdTV/flash-shell/releases/latest/download/flash-shell" --output flash-shell
chmod 555 flash-shell
sudo cp flash-shell /usr/bin/flash-shell
rm flash-shell
cd $cwd