# Pflichtenheft

## 1. Objectives

### 1.1. Mandatory criteria

## Fundamental

- starting and quiting applications
- dynamic tiling
- movement of applications
- floating/Static applications
- keyboard input handling
- window focusing

## Basic

- multiple workspaces
- multiple monitors
- configuration
- autostarting of Applications
- taskbar support
- Drun/Rofi/eww support

### 1.2. Desired criteria

- ipc
- taskbar
- config file
- power management (screenlocking after timeout)
- animations
- multi monitor support

### 1.3. Demarcation criteria

- no compositing
- no floating functions
- no got plugging on monitor devices
- no automatic monitor configuration

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
