# config(1) -- configuration for OxideWM

Inside config file, you can configure OxideWM as you like.
Please keep in mind that you have to configure every value properly for the WM to run, otherwise the programm will crash.

Please note that you have to set values for exec and exec_always.
If you do not configure the other fields, a default value will be used.

## DEFAULT VALUES

- cmds:
  - A + t, for opening a new application window
- exec:
- exec_always:
- border_witdh:
- border_color: 0xFFFFFF (white)
- border_focus_color: 0x000000 (black)
- gap: 10

## KEY SHORTCUS

OxideWM uses the 'Alt' key as the meta key

- A -> ALT
- C -> CONTROL
- S -> SHIFT

## MOVE COMMANDS:

- left
- right
- up
- down

## WMCOMMANDS:

Exec
: Lets you atart an application

Focus
: Makes you switch between applications

GoToWorkspace
: Makes you switch to another workspace

Kill
: Kill the focused window

Layout
: Changes the layout of the currently focused application

- args:

  - horizontal
  - vertical

- Move:

  - Moves the window to one of the following directions
  - args:
    - left,
    - right,
    - up,
    - down:

- MoveToWorkspace:

  - Moves the selected application to the selected workspace
  - args:
    - Number of your workspace

MoveToWorkspaceAndFollow
: Moves your focused application to the selected workspace and changes the focus to the according one

Quit
: Quit the window manager:

Resize
: Lets you resize your window

Restart
: Restarts the window manager

In the 'cmds' field of the configuration all keybinding for the WM are defined.
A Command consists of an array of keys, the command, some optional arguments and should be configured like the following:
In this example, the command opens a kitty window.

## EXAMPLES

- keys: ["A", "t"]
  command: Exec
  args: "kitty"
