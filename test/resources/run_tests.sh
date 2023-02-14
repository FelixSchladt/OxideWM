#!/bin/bash
# ┃ ━ ┏ ┓ ┗ ┛

XEPHYR=$(whereis -b Xephyr | cut -f2 -d ' ')

echo    "┏━━━━━━━━━━━━━━━━━━━┓"
echo -e "┃ \x1b[32m\x1b[1mINTEGRATION TESTS\x1b[0m ┃"
echo    "┗━━━━━━━━━━━━━━━━━━━┛"
xinit ./test/resources/unittestrc -- $XEPHYR :100 -ac -screen 511x50 -host-cursor
echo -e "\x1b[A\x1b[K\x1b[A\x1b[K\x1b[A\x1b[K\t\t\t\x1b[A"

echo    "┏━━━━━━━━━━━━━━━━━┓"
echo -e "┃ \x1b[32m\x1b[1mAUTOMATED TESTS\x1b[0m ┃"
echo    "┗━━━━━━━━━━━━━━━━━┛"
xinit ./test/resources/autotestrc -- $XEPHYR :100 -ac -screen 1200x1000 -host-cursor
echo -e "\x1b[A\x1b[K\x1b[A\x1b[K\x1b[A\x1b[K\t\t\t\x1b[A"
