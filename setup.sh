cwd=$(pwd)
cd
curl "http://github.com/BlackBirdTV/flash-shell/releases/latest/download/flash-shell" --output flash-shell
chmod 777 flash-shell
sudo cp flash-shell /usr/bin/flash-shell
rm flash-shell
cd $cwd