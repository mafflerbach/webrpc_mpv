#!/bin/bash
# poor mans tear up and teardown
/usr/bin/mpv --idle=yes --input-ipc-server=/tmp/mpvsocketTesting --fs=yes --vo=gpu --msg-level=ipc=v &
sqlite3 db/restmpv_test.db < db/drop.sql
sqlite3 db/restmpv_test.db < db/create.sql
TEST=true cargo test $1 -- --nocapture
kill $(ps aux | grep mpv | grep Testing | cut -d' ' -f 5)

