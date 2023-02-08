# Specification

## Product goal

Getting to know how window managers and Xorg work.
Development of a working window manager.

## Target group

Target group constains power users with advanced Linux knowledge..

## Product functions

### 1. Fundamental

#### 1.1 starting and quitting apps
The *Oxide* window manager should give the user the ability to start and quit applications easily through its interface.

#### 1.2 tiling functionality
The software itself must support dynamic tiling, allowing the user to arrange multiple applications in a grid-like arrangement optimizing screen space utilization.Along with this it should support both floating and static applications, giving the user flexibility in his window management.

#### 1.3 moving windows
Therefore applications are expected to be moved around the screen by the user to different tiled positions or to float as a separate window.

#### 1.4 controllable via keyboard
The user must be able, to control all aspects of the applications by using keyboard shortcuts.

#### 1.5 controllable via ipc
The user must be able, to control all aspects of the applications by using the ipc interface.

#### 1.6 focusing windows
The software must support focusing on different windows, allowing the user to easily switch between applications.

#### 1.7 key-forwarding
When a window is stated to be focused, the keybord inputs must be directed to the focused application.

### 2. Basic

#### 2.1 multiple workspaces
The software must support multiple workspaces, allowing the user to create, quit and switch between different virtual desktops.

##### 2.1.1 move window to workspace
The software must support moving a window to another workspace. When this functionality is executed, the windowmanager must:
- remove the widow from the old workspace
- add the window to the new workspace

##### 2.1.2 switching between workspaces
When this functionallity of the window manager is executed, the window manager must:
- display all windows that were opened or moved to this screen (if fullscreen is not active).

##### 2.1.3 closing workspaces
When this functionallity of the window manager is executed, the window manager must:
- close all windows that are currently in this workspace
- switch to another open one, so that the user is never on "no" workspace
    - when the last workspace is closed, a new workspace must be created. The windowmanager must then switch to the newly created workspace.


#### 2.2 config
The window manager must provide an interface for configuring various settings and options.<br>
This configuration must be human readable or must provide another interface so that an linux averse person can change the settings.<br>
There must be default values for the configuration elements, so that when a users configuration is incorrect, the windowmanager still starts.<br>
Furthermore, the configuration must be applied to the windowmanager, every time it is started.

#### 2.3 autostart
Autostarting of applications must be supported, allowing the user to specify which applications should start automatically. 

#### 2.4 utilities
The window manager should integrate a taskbar providing an intuitive and streamlined way to switch between open applications and workspaces quickly and easily.<br>
For this it is necessary to support popular utilities like Drun or Rofi.

### 3. Desired

#### 3.1 multiple screens
The windowmanager must empower the user to use multiple screens connected to his computer.

##### 3.1.1 multiple screens workspaces
To take full advantage of the multiple screens, the windowmanager must allow workspaces on every stream.

##### 3.1.1 multiple screens moving windows
The windowmanager must provide a way, to move windows between workspaces accross screens.

#### 3.3 screen locking
Also power management features should be included, such as screen locking after a specified timeout to help conserve energy and improve security.

#### 3.4 statusbar
The windowmanager should provide a statusbar of some sort, to keep track of which workspaces exist, and on which workspace the user currently operates.

### Documentation

Keeping track of tickets with timestamps.
