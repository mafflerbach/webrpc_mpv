#!/bin/bash
killall mpv_webrpc 
export DISPLAY=:0
SETTINGS=settings/settings2.json bin/mpv_webrpc &

