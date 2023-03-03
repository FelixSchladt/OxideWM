#!/bin/bash

function start_application() {
    echo -e "[\x1b[36m~\x1b[0m] Running window with frame example..."
    XEPHYR=$(whereis -b Xephyr | cut -f2 -d ' ')
    xinit ./xinitrc_window_with_frame -- $XEPHYR :100 -ac -screen 1200x1000 -host-cursor
}

echo -e "[\x1b[36m~\x1b[0m] Building window with frame example..."
cargo build --example window_with_frame && start_application

echo -e "[\x1b[36m~\x1b[0m] Goodbye :)"
