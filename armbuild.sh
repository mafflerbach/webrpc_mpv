PI_IP=192.168.0.52 # Be sure to change this!
TARGET=aarch64-unknown-linux-gnu # Pi 2/3/4
#TARGET=arm-unknown-linux-gnueabihf # Pi 0/1

# build binary
cross build --release --target $TARGET

# upload binary
#sshpass -p 'audiocenter' ssh audiocenter@$PI_IP "killall mpv*"
#sshpass -p 'audiocenter' scp -r ./templates audiocenter@$PI_IP:
#sshpass -p 'audiocenter' scp -r ./target/$TARGET/release/mpv_webrpc audiocenter@$PI_IP:
sshpass -p 'audiocenter' scp -r ./settings/settings2.json  audiocenter@$PI_IP:
#sshpass -p 'audiocenter' scp -r ./statup.sh  audiocenter@$PI_IP:
#sshpass -p 'audiocenter' ssh audiocenter@$PI_IP "./statup.sh &"
