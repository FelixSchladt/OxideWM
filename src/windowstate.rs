use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct WindowState {
    pub connection: Rc<RefCell<RustConnection>>,
    pub window: Window,
    pub title: String,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub titlebar_height: u32,
}

impl WindowState {
    pub fn new(connection: Rc<RefCell<RustConnection>>, window: Window) -> WindowState {
        let title = connection.borrow().get_property(false, window, AtomEnum::WM_NAME, AtomEnum::STRING, 0, 1024).unwrap().reply().unwrap().value;
        let title = String::from_utf8(title).unwrap();
        let visible = true;
        let focused = false;
        let urgent = false;
        let x = 0;
        let y = 0;
        let width = 0;
        let height = 0;
        let titlebar_height = 0;

        let mask = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::ENTER_WINDOW | EventMask::LEAVE_WINDOW );
        let res = connection.borrow().change_window_attributes(window, &mask).unwrap().check();
        if let Err(e) = res {
            println!("Error couldn change mask: {:?}", e);
            panic!("Error couldnt change mask");
        }

        WindowState {
            connection,
            window,
            title,
            visible,
            focused,
            urgent,
            x,
            y,
            width,
            height,
            titlebar_height,
        }
    }
}
