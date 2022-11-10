# Keyboard input
This documents how to receive keyboard input and utilize it.

## Get Mapping
Penrose offers to ways of getting the keyboard map.

1. Using xmodmap -pke

The xmodmap utility is part of the standard X utilities and should be pre installed. 
This can be called via the command module of rust and the output can be converted into a hashmap.

Has been already implemented with [commit](https://stackoverflow.com/questions/71131688/how-can-i-get-all-events-on-the-root-window-with-xcb)

2. Using the penrose-keysyms module
The penrose keysyms module seems to contain a full list of all possible keysyms.
Offers the benefit of being static.

3. (Maybe) x11rb GetKeyboardMapping
X11rb has a GetKeyboardMapping function
https://docs.rs/x11rb/0.8.1/x11rb/protocol/xproto/struct.GetKeyboardMappingReply.html

**Not yet investigates if this is a possible way to get the keysyms**


## Getting Keypress Events
At the current state it has been possible with [feature/ISSUE8](https://github.com/DHBW-FN/sweng_dhbWM/tree/feature/ISSUE8-full-screen)
to receive keypress events with the corresponding keysyms.
This though has only been achieved using an newly created window.
According to this [stackoverflow post](https://stackoverflow.com/questions/71131688/how-can-i-get-all-events-on-the-root-window-with-xcb) it is not possible to generally receive the keypress events from the root window. 
ButtonPress mask can be only be registered by one client at a time on a particular window.

Further investigation is needed.

### Grab key

This seems to be the metho utilized by penrose

https://docs.rs/x11rb/0.4.1/x11rb/generated/xproto/fn.grab_key.html
https://github.com/sminez/penrose/blob/154b29ff4c3c931ff28f99afd7ae4dd6654ec1ea/src/x11rb/mod.rs#L279
https://tronche.com/gui/x/xlib/input/XGrabKey.html

### Modifier key
the keyboard seems to differentiate upon modifiers
https://unix.stackexchange.com/questions/328400/how-to-remap-super-keys

I do not know currently how this works


### Keycodes
https://renenyffenegger.ch/notes/hardware/keyboard

####Keycode
* Device dependent
* 

=======
