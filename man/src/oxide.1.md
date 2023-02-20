% OXIDE(1) oxide 0.1.0
% Felix Schladt <schladt.felix-it21@it.dhbw-ravensburg.de>
% February 2023

# NAME
Oxide - a dynamic tiling window manager written in rust

# SYNOPSIS
**Oxide** 

# DESCRIPTION
## INTRODUCTION
Oxide windowmanager is a dynamic tiling windowmanager for X11.
Windows are automatically arranged in a grid-like fashion.
The user can then move and resize windows by using keyboard shortcuts.
Defining custom keyboard shortcuts to launch applications is possible, too.
Oxide tries to maximize the screensize by removing unnecessary borders
and decorations as well as to be as keyboard friendly as possible.
Everything can be done via the keyboard.

## TERMINOLIGY 
**Window**
: An X11 application window such as a browser or terminal.

**Workspace**
: A workspace contains multiple windows. The user can switch between several workspaces.

**Layout**
: Layouts are different algorithms placing windows.

## CONFIG FILE
Oxide can be configured via its config file. This includes keybindings, appearance and more.
Before editing the global config file located under **/etc/oxide/config.yml** should be copied into the users home directory under **~/.config/oxide/config.yml**.
For a more detailed description of the config see **oxide-config(1)**.

## LOGGING
Oxide log messages are written to **/var/log/syslog**.

## FURTHER DOCUMENTATION
Access the full Oxide documentation under **https://oxide.readthedocs.io/**.

# FILES
*~/.config/oxide/config.yml*
:   per-user config file

*/etc/oxide/config.yml*
:   global config file

*/usr/share/xsessions/oxide.desktop*
: 	Oxide desktop file

# BUGS
Please open an issue <https://github.com/DHBW-FN/OxideWM/issues> .

# COPYRIGHT
Copyright © 2023 Felix Schladt GPLv3+\: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: You are free to change and redistribute it. There is NO WARRANTY to the extent permitted by law.

# SEE ALSO
**oxide-config(1)**, **oxide-msg**, **oxide-bar(1)**
