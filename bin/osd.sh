#!/bin/sh

title='MediaMate On Screen Display'

wait_for_osd() {
	while ! wmctrl -a "$title"; do
		sleep 0.3
	done

	wmctrl -r "$title" -b add,above
}

wait_for_osd &

dir=$(dirname $(readlink -f $0))

export DISPLAY=:0
TITLE="$title" "$dir/../osd/electron/electron" "$dir/../osd/osd.js"
