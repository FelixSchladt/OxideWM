cargo build --example keybindings_window

XEPHYR=$(whereis -b Xephyr | cut -f2 -d ' ')
xinit ./xinitrc -- $XEPHYR :100 -ac -screen 1000x1000 -host-cursor 
