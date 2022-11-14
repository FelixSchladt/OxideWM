# Pflichtenheft

1. Zielbestimmung
1.1. Musskriterien

## Fundamental
* Starting and quiting applications
* Dynamic tiling
* Movement of applications
* Floating/Static applications
* Keyboard input handling
* Window focusing

## Basic
* Multiple workspaces
* Multiple monitors
* Configuration
* Autostarting of Applications
* Taskbar support
* Drun/Rofi/eww support

### 1.2. Wunschkriterien
* ipc
* Taskbar
* Config file
* Power managment (Screenlocking after timeout)
* Animations
* Multi Monitor Support

### 1.3. Abrenzungskriterien
* Kein Compositing
* Keine Floating-Funktion
* No got plugging on monitor devices
* No automatic monitor configuration

## 2. Produkteinsatz
### 2.1. Anwendungsbereiche
* Daily use on desktop computers

### 2.2. Zielgruppe
* power user with advanced Linux knowlede

### 2.3 Betriebsbedinungen
* personal computer
* posix system with Xorg
* usage is not limited
* monitoring the running system is not necessary^

## 3. Product overview

#TODO: Overview von Felix

## 4. Product functionality

#TODO

## 5. Data relevant for the user
* Application will be running locally thus has to be downloaded by the user
* Configuration files will be sotred locally

## 6. Product Performance - Requirements
* no dely between key inputs
* if possible: visible taks should be performed in under a 24th of a second
* this is not possible for opening application windows

## 7. Quality Requirements
* should not crash randomly
* default override if configurations are invalid
* config file should be formated as JSON

## 8. User Interface
* Keyboard will be used to controll the window manager
* Mouse can be used to focus on individal frames and interact with application interfaces like webbrowsers

## 9. Non-functional Requirements
* installer with package manager cargo

## 10. Project Enviroment
### 10.1. Software
* Unix based operating systems with X11
* running X11 instance
* no other running window manager

### 10.2. Hardware
* monitor
* keyboard working with your operating system
* there are no hardware limitations

### 10.3 Organisatorische Randbedingungen
* since our code is licesed with GPL v3 there are not conflicts with GPL licensed libraries

### 10.4 Produktschnittstelle
* behavior can be changed using config files
* programm actions will be stored in log files

## 11. Spezielle Anforderungen
### 11.1 Software
* [x11rb](https://github.com/psychon/x11rb)
* buildin crate `log` for logging
* Zbus for ipc

### 11.4 Entwicklungsschnittstellen
* X11 API
* Debus

## 12. Gliederung in Teilprodukte
