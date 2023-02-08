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

#### 1.3 moveing windows
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
The software must support moveing a window to another workspace. When this functionality is executed, the windowmanager must:
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


It is also supposed to provide an interface for configuring various settings and options, such as the number of workspaces, monitor arrangement, and more. 

Autostarting of applications is supposed to be another feature, allowing the user to specify which applications should start automatically when the software is launched. 

The window manager should integrate a taskbar providing an intuitive and streamlined way to switch between open applications and workspaces quickly and easily. For this it is necessary to support popular utilities like Drun or Rofi.

### 3. Desired

#### 3.1 multiple screens


Inter process communication (IPC) should be used for interacting between different applications and services, allowing for a seamless integration with the users workflow.
The window manager is supposed to use a config file in which the user can easily manage his preferences and settings. Also power management features should be included, such as screen locking after a specified timeout to help conserve energy and improve security. 
For improving the overall user experience the software is to include visually appealing animations.

### Documentation

Keeping track of tickets with timestamps.
