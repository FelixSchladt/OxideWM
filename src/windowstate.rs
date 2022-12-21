use x11rb::COPY_DEPTH_FROM_PARENT;
use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;
use x11rb::errors::ReplyError;
use x11rb::protocol::Event;
use x11rb::protocol::ErrorKind;
use x11rb::protocol::xproto::*;
use std::process::Command;
use std::collections::HashMap;



#[derive(Debug)]
pub struct WindowState {
    pub window: Window,
    pub title: String,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub titlebar_height: u16,
}

impl WindowState {
    pub fn new(connection: &RustConnection, window: Window) -> WindowState {
        let title = connection.get_property(false, window, AtomEnum::WM_NAME, AtomEnum::STRING, 0, 1024).unwrap().reply().unwrap().value;
        let title = String::from_utf8(title).unwrap();
        let visible = true;
        let focused = false;
        let urgent = false;
        let x = 0;
        let y = 0;
        let width = 0;
        let height = 0;
        let titlebar_height = 0;
        WindowState {
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
