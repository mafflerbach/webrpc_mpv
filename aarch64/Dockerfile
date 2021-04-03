FROM rustembedded/cross:aarch64-unknown-linux-gnu

RUN dpkg --add-architecture arm64 && \
	apt-get update && \
	apt-get install --assume-yes libsqlite3-dev:arm64

RUN find /usr/lib | grep sqlite3
