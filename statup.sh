#!/bin/bash
killall mpv_webrpc 
export DISPLAY=:0
tmux new -d './mpv_webrpc settings2.json'

