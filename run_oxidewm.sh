#!/bin/bash

function install_xephyr () {
    echo -ne " [\x1b[36m~\x1b[0m] Installing Xephyr...\n"\
             "[\x1b[36m~\x1b[0m] Detecting package manager...\n"\
             "[\x1b[32m+\x1b[0m] "

    whereis apt    | grep -i "/apt"    && sudo apt install xserver-xephyr
    whereis pacman | grep -i "/pacman" && sudo pacman -S xorg-server-xephyr

    echo -e " [\x1b[32m+\x1b[0m] Installation complete"
}

echo -ne "\x1b[1m\x1b[31m#- Thank you for using OxideWM -#\x1b[0m\n"\
         "[\x1b[36m~\x1b[0m] Checking whether Xephyr is installed...\n"\
         "[\x1b[32m+\x1b[0m] "

whereis Xephyr | grep "/Xephyr" && install_xephyr  # TODO: Change && to || after testing is done
