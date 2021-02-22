#!/bin/sh

title='Media Mate On Screen Display'

wait_for_osd() {
	while ! wmctrl -a "$title"; do
		sleep 0.3
	done

	wmctrl -r "$title" -b add,above
}

wait_for_osd &

dir=$(dirname $(readlink -f $0))/../osd

export DISPLAY=:0
TITLE="$title" "$dir/electron/electron" "$dir/osd.js"
