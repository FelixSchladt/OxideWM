use log::{error, info};
use std::process::exit;
use x11rb::protocol::xproto::{AtomEnum, ChangeWindowAttributesAux, ConnectionExt, EventMask, Window};
use x11rb::rust_connection::RustConnection;
use serde::Serialize;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Serialize)]
pub struct WindowState {
    #[serde(skip_serializing)]
    pub connection: Rc<RefCell<RustConnection>>,
    pub window: Window,
    pub title: String,
    pub visible: bool,
    pub urgent: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub titlebar_height: u32,
}

impl WindowState {
    pub fn new(connection: Rc<RefCell<RustConnection>>, window: Window) -> WindowState {
        let title = WindowState::get_title(&connection, window);
        let visible = true;
        let urgent = false;
        let x = 0;
        let y = 0;
        let width = 0;
        let height = 0;
        let titlebar_height = 0;

        let mask = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::ENTER_WINDOW | EventMask::LEAVE_WINDOW );

        match connection.borrow().change_window_attributes(window, &mask) {
            Ok(_) => {
                info!("Updated attributes of window {} ({})", title, window);
            }
            Err(reason) => {
                error!("Failed to change window attributes because {}", reason);
                //TODO refactor method to return a result instead, to cover this error instead of
                //exiting
                exit(-1);
            },
        };

        WindowState {
            connection,
            window,
            title,
            visible,
            urgent,
            x,
            y,
            width,
            height,
            titlebar_height,
        }
    }

    fn get_title(connection: &Rc<RefCell<RustConnection>>, window: Window) -> String {
        match connection.borrow().get_property(false, window, AtomEnum::WM_NAME, AtomEnum::STRING, 0, 1024) {
            Err(_) => {
                "Unknown".to_string()
            },
            Ok(property) => {
                String::from_utf8(property.reply().unwrap().value).expect("Unknown")
            }
        }
    }
}
