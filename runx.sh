cargo build

XEPHYR=$(whereis -b Xephyr | cut -f2 -d ' ')
xinit ./xinitrc -- $XEPHYR :100 -ac -screen 750x750+0+0 -screen 750x750+900+0 -host-cursor
