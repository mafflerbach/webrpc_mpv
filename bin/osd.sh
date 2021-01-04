#!/bin/sh

title='MediaMate On Screen Display'

wait_for_osd() {
	while ! wmctrl -a "$title"; do
		sleep 0.3
	done

	wmctrl -r "$title" -b add,above
}

wait_for_osd &

dir="$(dirname "$0")"/../osd
"$dir/node_modules/.bin/electron" "$dir/osd.js"
