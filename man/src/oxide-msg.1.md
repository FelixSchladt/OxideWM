% OXIDE-MSG(1) oxide-msg 0.1.0
% Felix Schladt <schladt.felix-it21@it.dhbw-ravensburg.de>
% February 2023

# NAME
oxide-msg - send messages to oxidewm

# SYNOPSIS
**oxide-msg** \[**-h**]| \[**-v**] | \[**-c** command] \[**-a** argument] 

# DESCRIPTION
The `oxide-msg` is an ipc command tool allowing queriying and messaging to oxidewm via the commandline

# OPTIONS
**-a**, **--argument** <ARGUMENT>
: arguments to specify command behvior

**-c**, **--command** <WM_COMMAND>
: window manager commands

**-h**, **--help**
: output help message and exit

**-v**, **--version**
: output version information and exit

## WM_COMMAND
Move **-a** [MOVEMENT]
: Move Window

Focus **-a** [MOVEMENT]
: Move Focus

Quit
: Quit the window manager

Kill
: Kill the currently focused window

Restart
: Reloads the config and restarts components

Layout **-a** [LAYOUT]
: Change the current layout

GoToWorkspace **-a** [WORKSPACE_ARGS]
: Change the current workspace

MoveToWorkspace **-a** [WORKSPACE_ARGS]
: Move the focused window to a different workspace

MoveToWorkspaceAndFollow **-a** [WORKSPACE_ARGS]
: Move the focused window to and select a different workspace

Exec **-a** <COMMAND>
: Execute a given command

Fullscreen
: Toggle fullscreen mode for the focused window

## MOVEMENT
Left
: Moves to the left

Right
: Moves to the right

## LAYOUT
VerticalStriped
: windows vertically next to each other

HorizontalStriped
: windows a horizontally underneath each other

None
: if no argument is provided, the next layout is chosen

## WORKSPACE_ARGS
Next
: next initialized workspace with a higher index than the current workspace

Previous
: next initialized workspace with a lower index than the current workspace

New
: newly initialized workspace

Index
: workspace with the given index

# EXAMPLES
```sh
cargo run -p oxide-msg -- -c "exec" -a "firefox"
cargo run -p oxide-msg -- --command "kill"
```

# BUGS
Please open an issue <https://github.com/DHBW-FN/OxideWM/issues>

# COPYRIGHT
Copyright © 2023 Felix Schladt GPLv3+\: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it. There is NO WARRANTY, to the extent permitted by law.

# SEE ALSO
**oxide(1)**, **oxide-config(1)**, **oxide-bar(1)**
