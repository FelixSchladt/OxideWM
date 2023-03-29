# Manual Tests

## The Windowmanager is run with the same privileges as the currently logged in user

Yes, no root or sudo is required when starting the application.
For installation, these privilges may still be necessary.

## Window Manager reads keyboard input

Yes, keybinds defined in the config work fine.
There is an issue with pressing keys in the wrong order, see `findings`.

## Window Focus can be switched by mouse

Yes, the window that the mouse hovers over will be focused.

## Window Focus can be switched with keyboard

No - Bug report submitted
This only does not work in some specific cases dependant on the environment.
The bug was found in a `Xephyr` instance while testing.
On a normal installation, this should not happen.

## Windows open

Yes, individual windows like the browser or terminal can be opened with either registered keyboard shortcuts, commands or dmenu.

## Windows close

Yes, windows can be closed with the registered keyboard shortcut as long as no external issue causes them to freeze.

## Can switch Workspace

Yes - Range with keyboard buttons from 1 - 9
Higher workspace numbers are supported, validated by integration tests, however there are no shortcut keys for them anymore they can be reached with cycling through spaces one by one.

## Workspaces automatically close when empty

Yes, when there is no window in a workspace and the user is corrently looking at another one, the obsolete workspace is closed.

## Can move to new workspace

Yes, the registered keybinds work fine.

## Workspaces can close

Yes.

## Workspace 0 is not allowed

Yes.

## Application can autostart

Yes, applications registered for autostart in the config to start as soon as the window manager is being run.

## Drun can be used

Yes, the drun can be accessed via terminal command and with the registered keybind.
