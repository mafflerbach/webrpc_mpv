#!/bin/bash
killall mpv_webrpc 
export DISPLAY=:0
./mpv_webrpc settings2.json &

