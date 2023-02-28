.. _configuration:

=============
Configuration
=============

Description
-----------
Define the behavior of Oxide.
The config file provides the possibility to customize e. g. keybindings, layout, style.
If the home config file is not existing, default values will be used but commands like `exec` and `exec_always` will not be working.
The config file is written in YAML.

Files
-----

During launch, Oxide searches for a config file in the following locations:

**Home config file:**

.. code-block:: bash
    
    ~/.config/Oxide/config.yml

**System config file:**

.. code-block:: bash

    /etc/Oxide/config.yml


Keybindings
-----------

Keys
^^^^

A keybinding has to consist of at least one or more MODIFIERS and exactly one normal key such as 't' for example.

Modifier
^^^^^^^^

| **M**
|  Meta key

| **A**
|  ALT key

| **C**
|  CONTROL key

| **S**
|  SHIFT key

Commands
--------

Commands consist of a command and optional arguments.

Commands (COMMAND)
^^^^^^^^^^^^^^^^^^

| **Move [MOVEMENT]**
|  move window

| **Focus [MOVEMENT]**
|  move focus

| **Quit**
|  quit the window manager

| **Kill**
|  kill the currently focused window

| **Restart**
|  reloads the config and restarts components

| **Layout [LAYOUT]**
|  change the current layout

| **GoToWorkspace [WORKSPACE_ARGS]**
|  change the current workspace

| **MoveToWorkspace [WORKSPACE_ARGS]**
|  move the used window to a different workspace

| **MoveToWorkspaceAndFollow [WORKSPACE_ARGS]**
|  move the focused window to and select a different workspace

| **Exec  COMMAND**
|  execute a given command

| **Fullscreen**
|  toggle fullscreen mode for the focused window

Arguments (ARGS)
^^^^^^^^^^^^^^^^

Command arguments are necessary for the movement, the layout or to control workspaces.

Movement (MOVEMENT)
^^^^^^^^^^^^^^^^^^^

| **Left**
|  moves to the left

| **Right**
|  moves to the right

Layout (LAYOUT)
^^^^^^^^^^^^^^^

| **VerticalStriped**
|  windows vertically next to each other

| **HorizontalStriped**
|  windows horizontally underneath each other

| **None**
|  if no argument is provided, the next layout is chosen

Workspace arguments (WORKSPACE_ARGS)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

| **Next**
|  Next initialized workspace with a higher index than the current workspace. If the workspace with the highest index is selected, the index with the lowest index will be selected.

| **Previous**
|  Next initialized workspace with a lower index than the current workspace. If the workspace with the lowest index is selected, the index with the highest index will be selected.

| **Next_free**
|  Next available workspace with which is not initialized. Gaps in the workspace indices are filled first.

| **Index**
|  workspace with the given index

Iterations
----------

The iteration commands provide the possibility to change between workspaces when given an iteration number as shown in the example down below.

| **iter**
|  iterates over given number in order to change

Default keybindings
-------------------

Here is a short overview of the default keybindings.

| **Meta+Shift+e**
|  quits the window manager

| **Meta+Shift+r**
|  restarts the window manager

| **Meta+Shift+q**
|  kills the current window

| **h/l**
|  direction keys (left/right)

| **Meta+[DIRECTION]**
|  changes the focus to the direction window

| **Meta+Shift+[DIRECTION]**
|  moves the window to the direction

| **Meta+f**
|  changes the current window to fullscreen

| **Meta+u**
|  switches to the next layout

| **Meta+i**
|  changes the layout to vertical

| **Meta+Shift+i**
|  changes to layout to horizontal

| **Right/Left**
|  workspace navigation keys (next/previous)

| **Meta+[WORKSPACE_DIRECTION]**
|  changes to the workspace direction

| **Meta+n**
|  opens a new workspace

| **Control+Meta+[WORKSPACE_DIRECTION]**
|  moves a window to the workspace direction

| **Control+Meta+n**
|  opens a new workspace and moves the window to it

| **Meta+Shift+[WORKSPACE_DIRECTION]**
|  moves the window to the workspace direction and follows it

| **Meta+Shift+n**
|  creates a new workspace, moves the window to it and follows

| **Control+Meta+Down**
|  quits the workspace

| **Meta+t**
|  opens dmenu

| **1/2/3/4/5/6/7/8/9**
|  workspace numbers

| **Meta+[WORKSPACE_NUMBER]**
|  switches to workspace number

| **Control+Meta+[WORKSPACE_NUMBER]**
|  moves window to workspace number

| **Meta+Shift+[WORKSPACE_NUMBER]**
|  moves window to workspace number and follows it

Borders
-------

| **border_width**
|  sets the border width of windows in pixels

| **border_color**
|  sets the border color and has to be entered in hexadecimal

| **border_focus_color**
|  sets the border color for focused windows and has to be entered in hexadecimal

| **gap**
|  gap between windows in pixels

Execute
-------

| **exec**
|  one time execution when the window manager starts

| **exec_always**
|  is executed during start of the window manager and also at each restart

Examples
--------
Keybindings
^^^^^^^^^^^

.. code-block:: bash

    cmds:
     - keys: ["M", "t"]
     commands:
         - command: Exec
          args: "dmenu"

In this example pressing the meta key and 't', a new dmenu window is opened.

Iterations
^^^^^^^^^^

.. code-block:: bash

    iter_cmds:
     - iter: [1, 2, 3, 4, 5, 6, 7, 8, 9]
     command:
       keys: ["M", "C", "$VAR"]
       commands:
            - command: GoToWorkspace
             args: "$VAR"


In this example using the **ALT** and **CONTROL** key paired with a number from one to nine, the user can go to the desired workspace.
``$VAR`` is a reference for the entered iterator.

Bugs
----

Please open an issue https://github.com/DHBW-FN/OxideWM/issues .
