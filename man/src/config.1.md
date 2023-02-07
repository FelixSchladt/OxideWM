% OXIDE-CONFIG(1) oxide-config 0.1.0
% Philipp Kalinowski
% February 2023

# NAME

oxide-config - config

# FILES

**~/.config/oxide/config.yml**
: Home config file

**/etc/oxide/config.yml**
: System config file

# Description

Define the behaviour of oxidewm. You can change style, layout, keybindings and more.
The config file is written using YAML syntax.
If the home config file is not existing, default values will be used but commands like `exec` and `exec_always` will not be working.

# KEYBINDING

## KEYS

The keys need at least one MODIFIER and one normal key such as 't'

### MODIFIER

**M**
: Meta key

**A**
: ALT key

**C**
: CONTROL key

**S**
: SHIFT key

### COMMANDS

Commands consist of a command and optional arguments.

### COMMAND

Move **args** [MOVEMENT]
: Move window

Focus **args** [MOVEMENT]
: Move Focus

Quit
: Quit the window manager

Kill
: Kill the currently focused window

Restart
: Reloads the config and restarts components

Layout **args** [LAYOUT]
: Change the current layout

GoToWorkspace **args** [WORKSPACE_ARGS]
: Change the current workspace

MoveToWorkspace **args** [WORKSPACE_ARGS]
: Move the used window to a different workspace

MoveToWorkspaceAndFollow **args** [WORKSPACE_ARGS]
: Move the focused window to and select a different workspace

Exec **args** <COMMAND>
: Execute a given command

Fullscreen
: Toggle fullscreen mode for the focused window

## ARGS

### MOVEMENT:

Left
: Moves to the left

right
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

Next_free
: next available workspace with a higher index than the current workspace which is not initialized

Index
: workspace with the given index

## ITERATIONS

iter
: iteraates over given number in order to change

## EXAMPLES

TODO: DEFAULT BINDINGS AND SAMPLE CONFIG

### KEYBINDINGS

```yaml
cmds:
  - keys: ["M", "t"]
    commands:
      - command: Exec
        args: "firefox"
```

### ITERATIONS

```yaml
iter_cmds:
  - iter: [1, 2, 3, 4, 5, 6, 7, 8, 9]
    command:
      keys: ["A", "C", "$VAR"]
      commands:
        - command: GoToWorkspace
          args: "$VAR"
```

In this example using the ALT and CONTROLL key paired with a number from one to nine, the user can go to the desired workspace.
`$VAR` is a reference for the entered iterator.

## BORDERS

border_witdh
: sets the border witdh of windows in

border_color
: sets the border color and has to be entered in hexadecimal

border_focus_color
: sets the border color for focused nbdows and has to be entered in hexadecimal

gap
: gap between windows

## EXECTUE

exec
: onetime execution when the window manager starts

exec_always
: executes when the windosw manager starts or restarts

# SEE ALSO

TODO: READ THE DOCS EINBINDEN
TODO: OTHER MAN PAES REFERENZIEREN

# COPYRIGHT

EINFÜGEN
