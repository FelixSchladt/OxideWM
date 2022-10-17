# Dependencies

## Brief introduction
This document will collect all depencies required for the project and will briefely state why this crate is required.
If not yet decided, the category will be marked with **[Dicussion]**.
If decided the category is marked with **[Certain]**.


## IPC
**[Certain]**
D-Bus has been chosen as ipc mechanism.
[Zbus project repository](https://gitlab.freedesktop.org/dbus/zbus/-/tree/main)
[Zbus crate](https://crates.io/crates/zbus)
[Zbus documentation](https://dbus.pages.freedesktop.org/zbus/)
Official D-Bus library from the freedesktop.org foundation with good documentation.

## X11
**[Discussion]**
There seem to be two main libraries offering X11 bingings for rust:
[XCB wikipedia](https://en.wikipedia.org/wiki/XCB)

Rust libraries:
[x11-rs](https://github.com/AltF02/x11-rs)
[x11rb](https://github.com/psychon/x11rb)

[Comparison between different X11 libraries](https://github.com/psychon/x11rb/blob/master/doc/comparison.md)

x11rb seems to offer some benefits:
* Purely written in rust -> less _unsafe_ calls
* Aims to be similar to the modern xcb library instead of the old xlib
* Better documentation

> I tend to use x11rb because of these reasons

