# Concept for Reactiveness of the WM
As in my current understanding at least some kind of async or loop is required.

> Async handling of tasks would complicate developement but i will not roll it out as a potential feature.
> However I do not think that it is required for a functional WM.

The WM needs to react to ipc and to keyboard(maybe this is ipc) inputs.

**Not researched**
As of my shallow understanding:
D-Bus offers queing and notification:

-> WM waits for an dbus event and then handles each event. 
Alternatively for a dbus event or keyboard input

>> Which other wms use dbus?
> Sorry, formulted the previous statement a little wrong. AwesomeWM has a dbus **module** which performs fine.
> Since, to my understanding, we are using dbus for communication with addons like taskbars, this should be good here as well i think.
> https://awesomewm.org/doc/api/libraries/dbus.html
