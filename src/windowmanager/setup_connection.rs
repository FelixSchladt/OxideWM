use std::{cell::RefCell, sync::Arc, process::exit};

use log::{info, error};
use x11rb::{
    protocol::{
        xproto::{
            ChangeWindowAttributesAux,
            EventMask, 
            Screen, 
            GrabMode, 
            ModMask, ConnectionExt
        },
        ErrorKind
    },
    rust_connection::{
        RustConnection,
        ReplyError,
        ConnectionError,
    }, 
    connection::Connection,
};

use crate::keybindings::KeyBindings;

pub fn grab_keys(connection: Arc<RefCell<RustConnection>>, keybindings: &KeyBindings) -> Result<(), ConnectionError> {
    info!("grabbing keys");
    //TODO check if the the screen iterations should be merged
    for screen in connection.borrow().setup().roots.iter() {
        for modifier in [0, u16::from(ModMask::M2)] {
            for keyevent in keybindings.events_vec.iter() {
                connection.borrow().grab_key(
                    false,
                    screen.root,
                    (keyevent.keycode.mask | modifier).into(),
                    keyevent.keycode.code,
                    GrabMode::ASYNC,
                    GrabMode::ASYNC,
                )?;
            }
        }
    }
    connection.borrow().flush()
}

pub fn update_root_window_event_masks(connection: Arc<RefCell<RustConnection>>) {
    let mask = ChangeWindowAttributesAux::default()
               .event_mask(
                    EventMask::SUBSTRUCTURE_REDIRECT |
                    EventMask::SUBSTRUCTURE_NOTIFY |
                    EventMask::BUTTON_MOTION |
                    EventMask::FOCUS_CHANGE |
                    //EventMask::ENTER_WINDOW |
                    //EventMask::LEAVE_WINDOW | //this applies only to the rootwin
                    EventMask::PROPERTY_CHANGE
                );

    for screen in connection.borrow().setup().roots.iter() {
        set_mask(connection.clone(), screen, mask).unwrap();
    }
}

fn set_mask(
    connection: Arc<RefCell<RustConnection>>,
    screen: &Screen,
    mask: ChangeWindowAttributesAux
) -> Result<(), ReplyError> {
    let update_result = connection.borrow().change_window_attributes(
                            screen.root,
                            &mask
                        )?.check();

    if let Err(ReplyError::X11Error(ref error)) = update_result {
        if error.error_kind == ErrorKind::Access {
            error!("Access to X11 Client Api denied!");
            exit(1);
        }
    }

    update_result
}