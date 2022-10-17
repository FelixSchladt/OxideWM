# Concept for Reactiveness of the WM
As in my current understanding at least some kind of async or loop is required.

The WM needs to react to ipc and to keyboard(maybe this is ipc) inputs.

**Not researched**
As of my shallow understanding:
D-Bus offers queing and notification:

-> WM waits for an dbus event and then handles each event. 
Alternatively for a dbus event or keyboard input
