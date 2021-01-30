#!/bin/sh

# Exit with Ctrl-Q
#
trap cleanup EXIT

cleanup() {
	reset
	killall mpv_webrpc mpv 2> /dev/null
}

wait_for_player_window() {
	while ! wmctrl -a 'MediaMate Player'; do
		sleep 0.1
	done
}

killall mpv_webrpc mpv 2> /dev/null
export DISPLAY=:0

TITLE='MediaMate Player' \
	SETTINGS=settings/settings2.json \
	DATABASE_URL=db/restmpv.db \
	bin/mpv_webrpc &

wait_for_player_window

bin/osd.sh
