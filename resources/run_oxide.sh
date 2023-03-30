#!/bin/bash

function start_application() {
    echo -e "[\x1b[36m~\x1b[0m] Running OxideWM..."
    XEPHYR=$(whereis -b Xephyr | cut -f2 -d ' ')
    xinit ./xinitrc -- $XEPHYR :100 -ac -screen 1200x1000 -host-cursor
}

echo -e "[\x1b[36m~\x1b[0m] Building Oxide-bar..."
cargo build -p oxide-bar

echo -e "[\x1b[36m~\x1b[0m] Building Oxide-msg..."
cargo build -p oxide-msg

echo -e "[\x1b[36m~\x1b[0m] Building OxideWM..."
cargo build && start_application

echo -e "[\x1b[36m~\x1b[0m] Goodbye :)"
