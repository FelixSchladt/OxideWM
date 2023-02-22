#!/bin/bash

function install_xephyr () {
    echo -e  "[\x1b[36m~\x1b[0m] Installing Xephyr..."
    echo -e  "[\x1b[36m~\x1b[0m] Detecting package manager..."
    echo -ne "[\x1b[32m+\x1b[0m] "
    which apt    && sudo apt install xserver-xephyr
    which pacman && sudo pacman -S xorg-server-xephyr

    echo -e " [\x1b[32m+\x1b[0m] Installation complete"
}

function install_cargo () {
    echo -e "[\x1b[36m~\x1b[0m] Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    which cargo || . export PATH=$PATH:~/.cargo/bin

    echo -e "[\x1b[32m+\x1b[0m] Installation complete"
}

function install_dmenu () {
    echo -e  "[\x1b[36m~\x1b[0m] Installing DMenu..."
    echo -e  "[\x1b[36m~\x1b[0m] Detecting package manager..."
    echo -ne "[\x1b[32m+\x1b[0m] "
    which apt    && sudo apt install dmenu
    which pacman && sudo pacman -S dmenu

    echo -e " [\x1b[32m+\x1b[0m] Installation complete"
}

function install_graphviz () {
    echo -e "[\x1b[36m~\x1b[0m] Installing Graphviz renderer..."

    which apt    && sudo apt install graphviz
    which pacman && sudo pacman -S graphviz

    echo -e "[\x1b[32m+\x1b[0m] Installation complete"
}

echo -e  "[\x1b[36m~\x1b[0m] Checking whether Xephyr is installed..."
echo -ne "[\x1b[32m+\x1b[0m] "
which Xephyr || install_xephyr

echo -e  "[\x1b[36m~\x1b[0m] Checking whether Rust is installed..."
echo -ne "[\x1b[32m+\x1b[0m] "
which cargo || install_cargo

echo -e  "[\x1b[36m~\x1b[0m] Checking whether DMenu is installed..."
echo -ne "[\x1b[32m+\x1b[0m] "
which dmenu || install_dmenu

echo -e  "[\x1b[36m~\x1b[0m] Checking whether Graphviz is installed..."
echo -ne "[\x1b[32m+\x1b[0m] "
which dot || install_graphviz
