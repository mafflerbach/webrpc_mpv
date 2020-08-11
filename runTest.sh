#!/bin/bash

/usr/bin/mpv --idle=yes --input-ipc-server=/tmp/mpvsocketTesting --fs=yes --vo=gpu --msg-level=ipc=v &
cargo test
kill $(ps aux | grep mpv | grep Testing | cut -d' ' -f 5)
