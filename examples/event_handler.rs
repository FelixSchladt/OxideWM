use std::error::Error;
use std::collections::HashMap;

use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::protocol::ErrorKind;
use x11rb::connection::Connection;

use crate::keybindings::KeyEvent;

pub fn keypress<C: Connection>(
    manager: &C,
    event: &KeyReleaseEvent,
    keyevent: HashMap<u8, Vec<KeyEvent>>,
    ) {
    //println!("Key pressed: {:?}", event);
    let keys = keyevent.get(&event.detail).expect("Registered key not found");
    //println!("Key: {:?}", keys);
    for key in keys {
        let state = u16::from(event.state);
        if state == key.keycode.mask || state == key.keycode.mask | u16::from(ModMask::M2) {
            println!("Key: {:?}", key);
            (key.event)(key.args.clone());
        }
    }
    
}
