FROM rustembedded/cross:aarch64-unknown-linux-gnu

RUN dpkg --add-architecture arm64 &&  apt-get update && \
    apt-get install --assume-yes wget

RUN cd /tmp && wget https://www.sqlite.org/2020/sqlite-autoconf-3320300.tar.gz && tar xvzf sqlite-autoconf-3320300.tar.gz && cd sqlite-autoconf-3320300 && ./configure --host=arm-linux --prefix=/usr CC=/usr/bin/aarch64-linux-gnu-gcc && make && make install

RUN find /usr/lib | grep sqlite3
