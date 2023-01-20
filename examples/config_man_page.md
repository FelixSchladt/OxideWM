# Config

In our config file, you can configure OxideWM as you like.
Please keep in mind that you have to configure every value properly for the WM to run, otherwise the programm will crash.

OxideWM uses the 'Alt' key as the meta key
Key shortcuts:

- A -> ALT
- C -> CONTROL
- S -> SHIFT

Move commands:

- left
- right
- up
- down

WmCommands:

- Exec
- Focus
- GoToWorkspace
- Kill, kill the focused window
- Layout, args: horizontal, Vertical
- Move, args: left, right, up, down
- MoveToWorkspace
- MoveToWorkspaceAndFollow
- Quit; quit the window manager
- Resize
- Restart, restart the window manager

In 'cmds' all keybinding for the WM are defined.
A Command consists of an array of keys, the command, some optional arguments and should be configured like the following:

- keys: ["A", "t"]
  command: Exec
  args: "kitty"
  In this example, the command opens a kitty window.
