# Concept IPC

## Description

An IPC mechanism for the window manager is required.
This is neccessary for:

* Taskbar
* External libraries
* Command line utility

## Requirements
As for the aforementioned use cases it will not be required to send large amounts of data. 
Only short messages will be exchanged between the clients. Also it is not expected that the ipc performance will have a significant impact on the usability of the system.
Therefore some ipc options such as shared memory and 

## Research

### Options
There are multiple different ways of implementing ipc on posix systems.

#### FIFO
[Named Pipes Wikipedia](https://en.wikipedia.org/wiki/Named_pipe)

#### Unix Sockets
[Unix Domain Sockets Wikipedia](https://de.wikipedia.org/wiki/Unix_Domain_Socket)

#### D-Bus
[D-Bus Wikipedia](https://en.wikipedia.org/wiki/D-Bus)
[D-Bus documentation Rust](https://docs.rs/dbus/latest/dbus/)
[D-Bus create from freedesktop.org](https://dbus.pages.freedesktop.org/zbus/)
[D-Bus interface for Rust](https://github.com/diwic/dbus-rs)



[Discussion about ipc on Stackoverflow](https://stackoverflow.com/questions/1235958/ipc-performance-named-pipe-vs-socket)

In the above linked discussion, multiple interesting points where made:
* TCP Sockets are only about 16% slower compared to FIFO
* IPC performance is in most cases not the bottleneck
* Sockets allow for two way communication
* Sockets are more widely supported
* IPC interface should be abstracted, so that the ipc mechanism can be changed in a later stage

[Stackoverflow Comparison D-Bus vs Unix Sockets](https://stackoverflow.com/questions/33887063/difference-between-dbus-and-other-interprocess-communications-method)

