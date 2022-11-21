use std::error::Error;
use std::collections::HashMap;

use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::protocol::ErrorKind;
use x11rb::connection::Connection;

//mod keybindings;
use crate::keybindings::KeyEvent;

pub fn keypress<C: Connection>(
    manager: &C,
    event: &KeyReleaseEvent,
    keyevent: HashMap<u8, KeyEvent>,
    ) {
    /*
    for keyevent in keyevent.iter() {
        if event.detail == keyevent.keycode.code {
            println!("Code: {:?}", keyevent.keycode.code);
            println!("mask: {:?}", keyevent.keycode.mask);
        }
    }*/
    println!("Key pressed: {:?}", event);
    let key = keyevent.get(&event.detail).expect("Registered key not found");
    println!("Key: {:?}", key);
    (key.event)(key.args.clone());
    
}
