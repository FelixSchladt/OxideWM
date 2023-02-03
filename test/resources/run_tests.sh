#!/bin/bash

XEPHYR=$(whereis -b Xephyr | cut -f2 -d ' ')
xinit ./test/resources/xinitrc -- $XEPHYR :100 -ac -screen 511x50 -host-cursor
