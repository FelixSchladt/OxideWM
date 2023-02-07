#!/bin/bash
# ┃ ━ ┏ ┓ ┗ ┛

XEPHYR=$(whereis -b Xephyr | cut -f2 -d ' ')

echo    "┏━━━━━━━━━━━┓"
echo -e "┃ \x1b[32m\x1b[1mUNITTESTS\x1b[0m ┃"
echo    "┗━━━━━━━━━━━┛"
xinit ./test/resources/unittestrc -- $XEPHYR :100 -ac -screen 511x50 -host-cursor 2>/dev/null

echo    "┏━━━━━━━━━━━━━━━━━┓"
echo -e "┃ \x1b[32m\x1b[1mAUTOMATED TESTS\x1b[0m ┃"
echo    "┗━━━━━━━━━━━━━━━━━┛"
xinit ./test/resources/oxidemsgrc -- $XEPHYR :100 -ac -screen 511x50 -host-cursor 2>/dev/null
