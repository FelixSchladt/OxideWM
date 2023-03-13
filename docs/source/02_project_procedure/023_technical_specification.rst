.. _technical_specification:

=======================================
Technical specification (Pflichtenheft)
=======================================

Product functions
-----------------

1. Fundamental
^^^^^^^^^^^^^^

1.1 starting and quitting apps
''''''''''''''''''''''''''''''
The *Oxide* window manager should give the user the ability to start and quit applications through its interface.

1.2 tiling functionality
''''''''''''''''''''''''
The software itself must support dynamic tiling, allowing the user to arrange applications in a grid-like arrangement optimizing screen space utilization. 
Along with this it shouis supposed to support both floating and static applications, giving the user flexibility in his window management.

1.3 moving windows
''''''''''''''''''
Therefore applications are expected to be moved around the screen by the user to different tiled positions or to float as a separate window.

1.4 controllable via keyboard
''''''''''''''''''''''''''''''
The user must be able to control all aspects of the applications by using keyboard shortcuts.

1.5 controllable via IPC
''''''''''''''''''''''''
The user must be able to control all aspects of the applications by using the IPC interface.

1.6 focusing windows
''''''''''''''''''''
The software must support focusing on different windows, allowing the user to switch between applications.

1.7 key-forwarding
''''''''''''''''''
When a window is stated to be focused the keybord inputs must be directed to the focused application.

2. Basic
^^^^^^^^

2.1 multiple workspaces
''''''''''''''''''''''''
The software must support at least ten workspaces, allowing the user to create, quit and switch between different virtual desktops.

2.1.1 move window to workspace
''''''''''''''''''''''''''''''
The software must support moving a window to another workspace. When this functionality is executed, the windowmanager must:
- remove the window from the old workspace
- add the window to the new workspace

2.1.2 switching between workspaces
''''''''''''''''''''''''''''''''''
When this functionality of the window manager is executed, the window manager must:
- display all windows that were opened or moved to this screen (if fullscreen is not active).

2.1.3 closing workspaces
''''''''''''''''''''''''
When this functionality of the window manager is executed, the window manager must:
- close all windows that are currently in this workspace
- switch to another open one, so that the user is never on "no" workspace 
When the last workspace is closed, a new workspace must be created. The windowmanager must then switch to the newly created workspace.

2.2 config
'''''''''''
The window manager must provide an interface for configuring various settings and options. 
This configuration must be human readable or must provide another interface so that an linux averse person can change the settings. 
There must be default values for the configuration elements, so that when a users configuration is incorrect, the windowmanager still starts. 
Furthermore, the configuration must be applied to the windowmanager, every time it is started.

2.2.1 keybindings
''''''''''''''''''
For every command, that the window manager provides, the user must be able to configure a keybinding specified as below. 
A keybinding must contain exactly one none modifier key such as `1`, `2`, `A`, `B`, ... 
It can contain any combination of the following modifiers: `Alt`, `Meta`, `Command`, `Shift`. 
To enhance the configurability, the user must be able to assign multiple commands to a single keybinding.

2.2.2 autostart
'''''''''''''''
Autostarting of applications must be supported, allowing the user to specify which applications should start automatically. 

2.3 utilities
''''''''''''''
The window manager should integrate a taskbar providing astreamlined way to switch between open applications and workspaces. 
For this it is necessary to support popular utilities like Drun or Rofi.

3. Desired
^^^^^^^^^^

3.1 multiple screens
'''''''''''''''''''''
The windowmanager must empower the user to use multiple screens connected to his computer.

3.1.1 multiple screens workspaces
''''''''''''''''''''''''''''''''''
To take full advantage of the multiple screens, the windowmanager must allow workspaces on every stream.

3.1.1 multiple screens moving windows
''''''''''''''''''''''''''''''''''''''
The windowmanager must provide a way, to move windows between workspaces across screens.

3.2 screen locking
''''''''''''''''''
Also power management features should be included, such as screen locking after a specified timeout to help conserve energy and improve security.

3.3 statusbar
'''''''''''''
The windowmanager should provide a statusbar of some sort, to keep track of which workspaces exist, and on which workspace the user currently operates.

4. Documentation
^^^^^^^^^^^^^^^^

Keeping track of tickets with timestamps.

5. Data relevant for the user
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
The application will be running locally so it needs to be downloaded and installed by the user before using it for the first time. 
Files needed for configuration will be stored locally.

6. Product performance - requirements
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
Claim is having no delay between key inputs and the following action. 
If possible, visible tasks should be performed in under a 24th of a second. This is not possible for opening application windows.

7. Quality requirements
^^^^^^^^^^^^^^^^^^^^^^^
Randomly crashing must not happen. If configurations are invalid they should be overwritten by default values. 
The config file should be formatted as JSON.

8. User Interface
^^^^^^^^^^^^^^^^^
Controlling the window manager will only be possible by using the keyboard. 
A mouse can be used to focus on individal frames and interact with application interfaces like webbrowsers.

9. Non-functional requirements
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
An installer with package manager cargo is required.

10. Project enviroment
^^^^^^^^^^^^^^^^^^^^^^

10.1. Software
''''''''''''''
The product is supposed to be used on Unix based operating systems with an X11 instance running. 
Furthermore there is no other running window manager accepted.

10.2. Hardware
''''''''''''''
Required hardware is at least one monitor as well as a keyboard working with the operating system. 
There are no hardware limitations.

10.3 Organizational framework
''''''''''''''''''''''''''''''
Since the code is licensed with GPL v3 there are no conflicts with GPL licensed libraries.

10.4 Product interface
''''''''''''''''''''''
The behavior of the window manager can be customized by changing the config files. 
Program actions will be stored in log files located under TODO .

11. Special requirements
^^^^^^^^^^^^^^^^^^^^^^^^

11.1 Software
'''''''''''''
- `x11rb <https://github.com/psychon/x11rb>`__ 
- buildin crate **log** for logging
- **Zbus** for IPC
- **Serde** for parsing 

11.2 Development interfaces
''''''''''''''''''''''''''''
- X11 API
- D-Bus