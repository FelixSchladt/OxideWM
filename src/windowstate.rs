use log::{error, info};
use std::process::exit;
use x11rb::protocol::xproto::{AtomEnum, ChangeWindowAttributesAux, ConfigureWindowAux, ConnectionExt, CreateWindowAux, EventMask, Screen, Window, WindowClass};
use x11rb::rust_connection::RustConnection;
use x11rb::connection::Connection;
use x11rb::COPY_DEPTH_FROM_PARENT;
use serde::Serialize;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Serialize)]
pub struct WindowState {
    #[serde(skip_serializing)]
    pub connection: Rc<RefCell<RustConnection>>,
    pub frame: Window,
    pub window: Window,
    pub title: String,
    pub visible: bool,
    pub urgent: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub titlebar_height: u16,
    pub border_width: u16,
}

impl WindowState {
    pub fn new(connection: Rc<RefCell<RustConnection>>, root_screen: &Screen, window: Window) -> WindowState {
        let title = WindowState::get_title(&connection, window);
        let visible = true;
        let urgent = false;
        let x: i16 = 0;
        let y: i16 = 0;
        let width: u16 = 0;
        let height: u16 = 0;
        let titlebar_height = 5;
        let border_width = 5;

        let frame = match connection.borrow().generate_id() {
            Ok(frame_id) => frame_id,
            Err(reason) => {
                error!("Failed to generate a new ID for a window frame because {}", reason);
                exit(-1);
            }
        };

        connection.borrow().create_window(
            COPY_DEPTH_FROM_PARENT,
            frame,
            root_screen.root,
            x,
            y,
            width,
            height,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &CreateWindowAux::new().background_pixel(root_screen.black_pixel),
        ).ok();

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
            frame,
            window,
            title,
            visible,
            urgent,
            x: x.into(),
            y: y.into(),
            width: width.into(),
            height: height.into(),
            titlebar_height,
            border_width,
        }
    }

    #[must_use]
    pub fn set_bounds(&self, x: i32, y: i32, width: u32, height: u32) -> &WindowState {
        let frame_aux = ConfigureWindowAux::new()
            .x(x)
            .y(y)
            .width(width)
            .height(height);

        let window_aux = ConfigureWindowAux::new()
            .x(x+i32::from(self.border_width))
            .y(y+i32::from(self.border_width+self.titlebar_height))
            .width(width-u32::from(self.border_width*2))
            .height(height-u32::from(self.titlebar_height-(self.border_width*2)));

        self.connection.borrow().configure_window(self.frame, &frame_aux).ok();
        self.connection.borrow().configure_window(self.window, &window_aux).ok();

        return self;
    }

    pub fn draw(&self) {
        let connection_borrow = self.connection.borrow();

        match connection_borrow.grab_server() {
            Ok(_) => {
                connection_borrow.map_window(self.frame).ok();
                connection_borrow.map_window(self.window).ok();
            },
            Err(reason) => {
                error!("Failed to grab server for window remapping because {}", reason);
            }
        }

        connection_borrow.ungrab_server().ok();
        connection_borrow.flush().ok();
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
