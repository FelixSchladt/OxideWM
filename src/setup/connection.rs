use std::{process::exit, sync::Arc};

use log::{error, info};
use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{ChangeWindowAttributesAux, ConnectionExt, EventMask, GrabMode, ModMask, Screen},
        ErrorKind,
    },
    rust_connection::{ConnectionError, ReplyError, RustConnection},
};

use crate::keybindings::KeyBindings;

pub fn get_connection(keybindings: &KeyBindings) -> Arc<RustConnection> {
    let rc = RustConnection::connect(None).unwrap().0;
    let rust_connection = Arc::new(rc);
    grab_keys(rust_connection.clone(), keybindings).expect("failed to grab keys");
    update_root_window_event_masks(rust_connection.clone());
    rust_connection
}

pub fn grab_keys(
    connection: Arc<RustConnection>,
    keybindings: &KeyBindings,
) -> Result<(), ConnectionError> {
    info!("grabbing keys");
    for screen in connection.setup().roots.iter() {
        for modifier in [0, u16::from(ModMask::M2)] {
            for keyevent in keybindings.events_vec.iter() {
                connection.grab_key(
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
    connection.flush()
}

pub fn ungrab_keys(
    connection: Arc<RustConnection>,
    keybindings: &KeyBindings,
) -> Result<(), ConnectionError> {
    info!("ungrabbing keys");
    for screen in connection.setup().roots.iter() {
        for modifier in [0, u16::from(ModMask::M2)] {
            for keyevent in keybindings.events_vec.iter() {
                connection.ungrab_key(
                    keyevent.keycode.code,
                    screen.root,
                    (keyevent.keycode.mask | modifier).into(),
                )?;
            }
        }
    }
    connection.flush()
}

fn update_root_window_event_masks(connection: Arc<RustConnection>) {
    let mask = ChangeWindowAttributesAux::default().event_mask(
        EventMask::SUBSTRUCTURE_REDIRECT |
                    EventMask::SUBSTRUCTURE_NOTIFY |
                    EventMask::BUTTON_MOTION |
                    EventMask::FOCUS_CHANGE |
                    //EventMask::ENTER_WINDOW |
                    //EventMask::LEAVE_WINDOW | //this applies only to the rootwin
                    EventMask::PROPERTY_CHANGE,
    );

    for screen in connection.setup().roots.iter() {
        set_mask(connection.clone(), screen, mask).unwrap();
    }
}

fn set_mask(
    connection: Arc<RustConnection>,
    screen: &Screen,
    mask: ChangeWindowAttributesAux,
) -> Result<(), ReplyError> {
    let update_result = connection
        .change_window_attributes(screen.root, &mask)?
        .check();

    if let Err(ReplyError::X11Error(ref error)) = update_result {
        if error.error_kind == ErrorKind::Access {
            error!("Access to X11 Client Api denied!");
            exit(1);
        }
    }

    update_result
}
