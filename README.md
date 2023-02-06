# *Software Engineering* project


![workflow](https://github.com/DHBW-FN/OxideWM/actions/workflows/rust.yml/badge.svg)
[![Rust-Tests](https://github.com/DHBW-FN/OxideWM/actions/workflows/rust_test.yml/badge.svg)](https://github.com/DHBW-FN/OxideWM/actions/workflows/rust_test.yml)
<!--![release](/github/v/release/DHBW-FN/OxideWM?display_name=tag) -->

## Project status

### [Zenhub Board](https://app.zenhub.com/workspaces/oxidewm-635665ffcecdb867786ebd04/board)
### [ReadTheDocs](https://oxide.readthedocs.io/en/latest/)

## Group members
1. [Antonia Pawlik](https://github.com/gungula)
2. [Thomas Gingele](https://github.com/B1TC0R3)
3. [Jan Schaible](https://github.com/janschaible)
4. [Felix Schladt](https://github.com/FelixSchladt)
5. [Philipp Kalinowski](https://github.com/Philipp6802)

## Brief project description

The goal is to write an X11 dynamic tiling window manager in the rust programming language.
This project idea is inspired by the DWM, leftWM and i3WM. 

## Projected features

### Fundamental

The *Oxide* window manager should give the user the ability to start and quit applications easily through its interface. The software itself is supposed to support dynamic tiling, allowing the user to arrange multiple applications in a grid-like arrangement optimizing screen space utilization. Along with this it should support both floating and static applications, giving the user flexibility in his window management.

Therefore applications are expected to be moved around the screen by the user to different tiled positions or to float as a separate window.

Keyboard inputs are to be handled effectively, allowing the user to control all aspects of the applications by using keyboard shortcuts. The software should support focusing on different windows, allowing the user to easily switch between applications.

### Basic

The software should support multiple workspaces as well as multiple monitors, allowing the user to create and switch between different virtual desktops and to extend their workspace across multiple screens. It is also supposed to provide an interface for configuring various settings and options, such as the number of workspaces, monitor arrangement, and more. 

Autostarting of applications is supposed to be another feature, allowing the user to specify which applications should start automatically when the software is launched. 

The window manager should integrate a taskbar providing an intuitive and streamlined way to switch between open applications and workspaces quickly and easily. For this it is necessary to support popular utilities like Drun or Rofi.

### Desired

Inter process communication (IPC) should be used for interacting between different applications and services, allowing for a seamless integration with the users workflow.
The window manager is supposed to use a config file in which the user can easily manage his preferences and settings. Also power management features should be included, such as screen locking after a specified timeout to help conserve energy and improve security. 
For improving the overall user experience the software is to include visually appealing animations.

## Logging

To change log level, set environment variable `RUST_LOG=[error, info, debug, trace]`.
Logs will always be written to `/var/log/syslog`.
When project is built without `--release` flag, the logs are additionally written to `stdout` and to `log/*.log`.
