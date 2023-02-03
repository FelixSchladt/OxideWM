# Performance specification

## 1. Objectives

### 1.1. Mandatory criteria

## Fundamental

The *Oxide* window manager should give the user the ability to start and quit applications easily through its interface. The software itself is supposed to support dynamic tiling, allowing the user to arrange multiple applications in a grid-like arrangement optimizing screen space utilization. Along with this it should support both floating and static applications, giving the user flexibility in his window management.

Therefore applications are expected to be moved around the screen by the user to different tiled positions or to float as a separate window.

Keyboard inputs are to be handled effectively, allowing the user to control all aspects of the applications by using keyboard shortcuts. The software should support focusing on different windows, allowing the user to easily switch between applications.

## Basic

The software should support multiple workspaces as well as multiple monitors, allowing the user to create and switch between different virtual desktops and to extend their workspace across multiple screens. It is also supposed to provide an interface for configuring various settings and options, such as the number of workspaces, monitor arrangement, and more. 

Autostarting of applications is supposed to be another feature, allowing the user to specify which applications should start automatically when the software is launched. 

The window manager should integrate a taskbar providing an intuitive and streamlined way to switch between open applications and workspaces quickly and easily. For this it is necessary to support popular utilities like Drun or Rofi.

### 1.2. Desired criteria

Inter process communication (IPC) should be used for interacting between different applications and services, allowing for a seamless integration with the users workflow.
The window manager is supposed to use a config file in which the user can easily manage his preferences and settings. Also power management features should be included, such as screen locking after a specified timeout to help conserve energy and improve security. 
For improving the overall user experience the software is to include visually appealing animations.

### 1.3. Demarcation criteria

Compositing should be taken over by a compositor. 
Also the software will not include floating functions, meaning that applications cannot be detached from the main window and positioned independently. Hot plugging will not be supported so the user will have to manually (re-)configure his monitor setup if they change or add a monitor.

## 2. Product Usage

### 2.1. Applications

The product is to be for the daily use on desktop computers.

### 2.2. Target group

Target group constains power users with advanced Linux knowledge.

### 2.3 Service conditions

- desktop computer
- posix system with Xorg
- usage is not limited
- monitoring the running system is not necessary

## 3. Product overview

#TODO: Overview von Felix

## 4. Product functionality

#TODO

## 5. Data relevant for the user

The application will be running locally so it needs to be downloaded and installed by the user before using it for the first time.

Files needed for configuration will be stored locally.

## 6. Product performance - requirements

Claim is having no delay between key inputs and the following action. 

If possible, visible tasks should be performed in under a 24th of a second. This is not possible for opening application windows.

TODO: more input!


## 7. Quality requirements

Randomly crashing must not happen. If configurations are invalid they should be overwritten by default values. 
The config file should be formatted as JSON.

## 8. User Interface

Controlling the window manager will only be possible by using the keyboard.

A mouse can be used to focus on individal frames and interact with application interfaces like webbrowsers.

## 9. Non-functional requirements

An installer with package manager cargo is required.

## 10. Project enviroment

### 10.1. Software

The product is supposed to be used on Unix based operating systems with an X11 instance running. Furthermore there is no other running window manager accepted.

### 10.2. Hardware

Required hardware is at least one monitor as well as a keyboard working with the operating system.

There are no hardware limitations.

### 10.3 Organizational framework

Since the code is licensed with GPL v3 there are no conflicts with GPL licensed libraries.

### 10.4 Product interface

The behavior of the windowmanager can be customized by changing the config files.
Program actions will be stored in log files.

## 11. Special requirements

### 11.1 Software

- [x11rb](https://github.com/psychon/x11rb)
- buildin crate `log` for logging
- Zbus for ipc

### 11.4 Development interfaces

- X11 API19
- Debus

## 12. Breakdown into sub-products
