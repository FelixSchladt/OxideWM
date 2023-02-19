% OXIDE-MSG(1) oxide-msg 0.1.0
% Felix Schladt <schladt.felix-it21@it.dhbw-ravensburg.de>
% February 2023

# NAME
oxide-msg - send messages to Oxide

# SYNOPSIS
**oxide-msg** \[**-h**]| \[**-v**] | \[**-c** command] \[**-a** argument] 

# DESCRIPTION
The `oxide-msg` is an ipc command tool allowing querying and messaging to Oxide via the commandline.

# OPTIONS
**-a**, **--argument** [ARGUMENT]
: arguments to specify command behavior

**-c**, **--command** [WM_COMMAND]
: window manager commands

**-h**, **--help**
: output help message and exit

**-v**, **--version**
: output version information and exit

## WM_COMMAND
Move **-a** [MOVEMENT]
: move window

Focus **-a** [MOVEMENT]
: move focus

Quit
: quit the window manager

Kill
: kill the currently focused window

Restart
: reloads the config and restarts components

Layout **-a** [LAYOUT]
: change the current layout

GoToWorkspace **-a** [WORKSPACE_ARGS]
: change the current workspace

MoveToWorkspace **-a** [WORKSPACE_ARGS]
: move the focused window to a different workspace

MoveToWorkspaceAndFollow **-a** [WORKSPACE_ARGS]
: move the focused window to and select a different workspace

Exec **-a** <COMMAND>
: execute a given command

Fullscreen
: toggle fullscreen mode for the focused window

## MOVEMENT
Left
: moves to the left

Right
: moves to the right

## LAYOUT
Vertical
: windows vertically next to each other

Horizontal
: windows horizontally underneath each other

None
: if no argument is provided, the next layout is chosen

## WORKSPACE_ARGS
Next
: Next initialized workspace with a higher index than the current workspace. If the workspace with the highest index is selected, the index with the lowest index will be selected.

Previous
: Next initialized workspace with a lower index than the current workspace. If the workspace with the lowest index is selected, the index with the highest index will be selected.

Next_free
: Next available workspace with which is not initialized. Gaps in the workspace indices are filled first.

Index
: workspace with the given index

# EXAMPLES
```sh
cargo run -p oxide-msg -- -c "exec" -a "kitty"
cargo run -p oxide-msg -- --command "kill"
```

# BUGS
Please open an issue <https://github.com/DHBW-FN/OxideWM/issues>

# COPYRIGHT
Copyright © 2023 Felix Schladt GPLv3+\: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it. There is NO WARRANTY, to the extent permitted by law.

# SEE ALSO
**oxide(1)**, **oxide-config(1)**, **oxide-bar(1)**
