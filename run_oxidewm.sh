#!/bin/bash

function install_xephyr () {
    echo -e  "[\x1b[36m~\x1b[0m] Installing Xephyr..."
    echo -e  "[\x1b[36m~\x1b[0m] Detecting package manager..."
    echo -ne "[\x1b[32m+\x1b[0m] "
    whereis apt    | grep -i "/apt"    && sudo apt install xserver-xephyr
    whereis pacman | grep -i "/pacman" && sudo pacman -S xorg-server-xephyr

    echo -e " [\x1b[32m+\x1b[0m] Installation complete"
}

function install_cargo () {
    echo -e "[\x1b[36m~\x1b[0m] Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    whereis cargo | grep "/cargo" || export PATH=$PATH:~/.cargo/bin

    echo -e "[\x1b[32m+\x1b[0m] Installation complete"
}

echo -e  "\x1b[1m\x1b[31m#- Thank you for using OxideWM -#\x1b[0m"
echo -e  "[\x1b[36m~\x1b[0m] Checking whether Xephyr is installed..."
echo -ne "[\x1b[32m+\x1b[0m] "
whereis Xephyr | grep "/Xephyr" || install_xephyr

echo -e  "[\x1b[36m~\x1b[0m] Checking whether Rust is installed..."
echo -ne "[\x1b[32m+\x1b[0m] "
whereis cargo  | grep "/cargo"  || install_cargo

echo -e "[\x1b[36m~\x1b[0m] Building OxideWM..."
cargo build

echo -e "[\x1b[36m~\x1b[0m] Running OxideWM..."
XEPHYR=$(whereis -b Xephyr | cut -f2 -d ' ')
xinit ./xinitrc -- $XEPHYR :100 -ac -screen 1000x1000 -host-cursor

echo -e "[\x1b[36m~\x1b[0m] Goodbye :)"
