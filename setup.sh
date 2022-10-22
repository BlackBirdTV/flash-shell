cwd=$(pwd)
cd ~
curl -L -O "https://github.com/BlackBirdTV/flash-shell/releases/latest/download/flash-shell" --output flash-shell
if [ -d /usr/bin/flash-shell ]
then
    mkdir /usr/bin/flash-shell
fi
sudo cp flash-shell /usr/bin/flash-shell
sudo chmod 777 /usr/bin/flash-shell
cd $cwd
