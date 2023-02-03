# Software Engineering Project


![workflow](https://github.com/DHBW-FN/OxideWM/actions/workflows/rust.yml/badge.svg)
[![Rust-Tests](https://github.com/DHBW-FN/OxideWM/actions/workflows/rust_test.yml/badge.svg)](https://github.com/DHBW-FN/OxideWM/actions/workflows/rust_test.yml)
<!--![release](/github/v/release/DHBW-FN/OxideWM?display_name=tag) -->

## Project Status

### [Zenhub Board](https://app.zenhub.com/workspaces/oxidewm-635665ffcecdb867786ebd04/board)
### [ReadTheDocs] (https://oxide.readthedocs.io/en/latest/)

## Group Members
1. [Antonia Pawlik](https://github.com/gungula)
2. [Thomas Gingele](https://github.com/B1TC0R3)
3. [Jan Schaible](https://github.com/janschaible)
4. [Felix Schladt](https://github.com/FelixSchladt)
5. [Philipp Kalinowski](https://github.com/Philipp6802)

## Brief Project Description

The goal is to write an X11 dynamic tiling window manager in the rust programming language.
This project idea is inspired by the DWM, leftWM and i3WM. 

## Projected Features

### Fundamental
* starting and quiting applications
* dynamic tiling
* movement of applications
* floating/static applications
* keyboard input handling
* window focusing

### Basic
* multiple workspaces
* multiple monitors
* configuration
* autostarting of applications
* taskbar support
* Drun/Rofi/eww support

### Optional
* ipc
* taskbar
* config file
* power management (screenlocking after timeout)
* animations

## Logging
To change log level, set environment variable `RUST_LOG=[error, info, debug, trace]`.
Logs will always be written to `/var/log/syslog`.
When project is build without `--release` flag, the logs are additionally written to `stdout` and to `log/*.log`.
