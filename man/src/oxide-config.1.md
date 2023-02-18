% OXIDE-CONFIG(1) oxide-config 0.1.0
% Philipp Kalinowski
% February 2023

# NAME

oxide-config - config of Oxide

# DESCRIPTION

Define the behavior of Oxide.
The config file provides the possibility to customize e. g. keybindings, layout, style.
If the home config file is not existing, default values will be used but commands like `exec` and `exec_always` will not be working.
The config file is written in YAML.

# FILES

During launch, Oxide searches for a config file in the following locations:

**~/.config/oxide/config.yml**
: home config file

**/etc/oxide/config.yml**
: system config file

# KEYBINDING

## KEYS

A keybinding has to consist of at least one or more MODIFIERS and exactly one normal key such as 't' for example.

## MODIFIER

**M**
: Meta key

**A**
: ALT key

**C**
: CONTROL key

**S**
: SHIFT key

# COMMANDS

Commands consist of a command and optional arguments.

## COMMAND

Move **args** [MOVEMENT]
: move window

Focus **args** [MOVEMENT]
: move focus

Quit
: quit the window manager

Kill
: kill the currently focused window

Restart
: reloads the config and restarts components

Layout **args** [LAYOUT]
: change the current layout

GoToWorkspace **args** [WORKSPACE_ARGS]
: change the current workspace

MoveToWorkspace **args** [WORKSPACE_ARGS]
: move the used window to a different workspace

MoveToWorkspaceAndFollow **args** [WORKSPACE_ARGS]
: move the focused window to and select a different workspace

Exec **args** COMMAND
: execute a given command

Fullscreen
: toggle fullscreen mode for the focused window

## ARGS

Command arguments are necessary for the movement, the layout or to control workspaces.

## MOVEMENT

Left
: moves to the left

Right
: moves to the right

## LAYOUT

VerticalStriped
: windows vertically next to each other

HorizontalStriped
: windows horizontally underneath each other

None
: if no argument is provided, the next layout is chosen

## WORKSPACE_ARGS

Next
: next initialized workspace with a higher index than the current workspace

Previous
: next initialized workspace with a lower index than the current workspace

Next_free
: next available workspace with a higher index than the current workspace which is not initialized

Index
: workspace with the given index

# ITERATIONS

The iteration commands provide the possibility to change between workspaces when given an iteration number as shown in the example down below.

iter
: iterates over given number in order to change

# DEFAULT KEYBINDINGS

Here is a short overview of the default keybindings.

Meta+Shift+e
: quits the window manager

Meta+Shift+r
: restarts the window manager

Meta+Shift+q
: kills the current window

h/l
: direction keys (left/right)

Meta+[DIRECTION]
: changes the focus to the direction window

Meta+Shift+[DIRECTION]
: moves the window to the direction

Meta+f
: changes the current window to fullscreen

Meta+u
: switches to the next layout

Meta+i
: changes the layout to vertical

Meta+Shift+i
: changes to layout to horizontal

Right/Left
: workspace navigation keys (next/previous)

Meta+[WORKSPACE_DIRECTION]
: changes to the workspace direction

Meta+n
: opens a new workspace

Control+Meta+[WORKSPACE_DIRECTION]
: moves a window to the workspace direction

Control+Meta+n
: opens a new workspace and moves the window to it

Meta+Shift+[WORKSPACE_DIRECTION]
: moves the window to the workspace direction and follows it

Meta+Shift+n
: creates a new workspace, moves the window to it and follows

Control+Meta+Down
: quits the workspace

Meta+t
: opens dmenu

1/2/3/4/5/6/7/8/9
: workspace numbers

Meta+[WORKSPACE_NUMBER]
: switches to workspace number

Control+Meta+[WORKSPACE_NUMBER]
: moves window to workspace number

Meta+Shift+[WORKSPACE_NUMBER]
: moves window to workspace number and follows it

# BORDERS

border_width
: sets the border width of windows in pixels

border_color
: sets the border color and has to be entered in hexadecimal

border_focus_color
: sets the border color for focused windows and has to be entered in hexadecimal

gap
: gap between windows in pixels

# EXECUTE

exec
: one time execution when the window manager starts

exec_always
: is executed during start of the window manager and also at each restart

# EXAMPLES

## KEYBINDINGS

```yaml
cmds:
  - keys: ["M", "t"]
    commands:
      - command: Exec
        args: "dmenu"
```

In this example pressing the meta key and 't', a new dmenu window is opened.

## ITERATIONS

```yaml
iter_cmds:
  - iter: [1, 2, 3, 4, 5, 6, 7, 8, 9]
    command:
      keys: ["M", "C", "$VAR"]
      commands:
        - command: GoToWorkspace
          args: "$VAR"
```

In this example using the ALT and CONTROL key paired with a number from one to nine, the user can go to the desired workspace.
`$VAR` is a reference for the entered iterator.

# BUGS

Please open an issue <https://github.com/DHBW-FN/OxideWM/issues> .

# COPYRIGHT

Copyright © 2023 Philipp Kalinowski GPLv3+\: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: You are free to change and redistribute it. There is NO WARRANTY to the extent permitted by law.

# FURTHER DOCUMENTATION

Access the full Oxide documentation under **https://oxide.readthedocs.io/**.

# SEE ALSO

**oxide(1)**, **oxide-msg(1)**, **oxide-bar(1)**
