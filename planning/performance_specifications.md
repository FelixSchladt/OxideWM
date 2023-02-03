# Performance specification

## 1. Objectives

### 1.1. Mandatory criteria

## Fundamental

The Oxide Windowmanager should give the user the ability to start and quit applications easily through its interface. The software itself is supposed to support dynamic tiling, allowing the user to arrange multiple applications in a grid-like arrangement optimizing screen space utilization. Along with this it should support both floating and static applications, giving the user flexibility in his window management.

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

- daily use on desktop computers

### 2.2. Target group

- power user with advanced Linux knowledge

### 2.3 Service conditions

- personal computer
- posix system with Xorg
- usage is not limited
- monitoring the running system is not necessary

## 3. Product overview

#TODO: Overview von Felix

## 4. Product functionality

#TODO

## 5. Data relevant for the user

- application will be running locally, needs to be downloaded by the user
- configuration files will be stored locally

## 6. Product performance - requirements

- no dely between key inputs
- if possible: visible taks should be performed in under a 24th of a second
- this is not possible for opening application windows

## 7. Quality requirements

- should not crash randomly
- default override if configurations are invalid
- config file should be formated as JSON

## 8. User Interface

- keyboard will be used to control the window manager
- mouse can be used to focus on individal frames and interact with application interfaces like webbrowsers

## 9. Non-functional Requirements

- installer with package manager cargo

## 10. Project Enviroment

### 10.1. Software

- Unix based operating systems with X11
- running X11 instance
- no other running window manager

### 10.2. Hardware

- monitor
- keyboard working with your operating system
- there are no hardware limitations

### 10.3 Organisational framework

- since our code is licensed with GPL v3 there are no conflicts with GPL licensed libraries

### 10.4 Product Interface

- behavior can be changed using config files
- programm actions will be stored in log files

## 11. Special requirements

### 11.1 Software

- [x11rb](https://github.com/psychon/x11rb)
- buildin crate `log` for logging
- Zbus for ipc

### 11.4 Development interfaces

- X11 API19
- Debus

## 12. Breakdown into sub-products
