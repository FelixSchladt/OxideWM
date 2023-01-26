use log::error;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use x11rb::connection::Connection;
use x11rb::COPY_DEPTH_FROM_PARENT;
use serde::Serialize;
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;

use crate::config::Config;

#[derive(Debug, Clone, Serialize)]
pub struct WindowState {
    #[serde(skip_serializing)]
    pub connection: Arc<RustConnection>,
    #[serde(skip_serializing)]
    pub config: Rc<RefCell<Config>>,
    pub frame: Window,
    pub window: Window,
    pub title: String,
    pub visible: bool,
    pub urgent: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub gap_size: u32,
}

impl WindowState {
    pub fn new(connection: Arc<RustConnection>, root_screen: &Screen, config: Rc<RefCell<Config>>, window: Window) -> WindowState {
        let title = connection.get_property(
                                  false,
                                  window,
                                  AtomEnum::WM_NAME,
                                  AtomEnum::STRING,
                                  0,
                                  1024
                              ).unwrap()
                               .reply()
                               .unwrap()
                               .value;
        let title = String::from_utf8(title).unwrap();
        let visible = true;
        let urgent = false;
        let x: i32 = 0;
        let y: i32 = 0;
        let width: u32 = 0;
        let height: u32 = 0;
        let border_width = config.borrow().border_width;
        let gap_size = config.borrow().gap;
        let border_color = config.borrow().border_color;

        let colormap = connection.generate_id().unwrap();
        connection.create_colormap(ColormapAlloc::ALL, colormap, root_screen.root, root_screen.root_visual).unwrap();
        connection.alloc_color(colormap, 255, 0, 0).unwrap();

        let frame = connection.generate_id().unwrap();
        connection.create_window(
            COPY_DEPTH_FROM_PARENT,
            frame,
            root_screen.root,
            x as i16,
            y as i16,
            width as u16,
            height as u16,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &CreateWindowAux::new().background_pixel(colormap),
        ).unwrap();

        let mask = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::ENTER_WINDOW | EventMask::LEAVE_WINDOW );
        let res = connection.change_window_attributes(window, &mask).unwrap().check();
        if let Err(e) = res {
            error!("Error couldn change mask: {:?}", e);
            panic!("Error couldnt change mask");
        }

        WindowState {
            connection,
            config,
            frame,
            window,
            title,
            visible,
            urgent,
            x,
            y,
            width,
            height,
            border_width,
            gap_size,
        }
    }

    pub fn set_bounds(&self, x: i32, y: i32, width: u32, height: u32) -> &WindowState {
        let frame_aux = ConfigureWindowAux::new()
            .x(x+self.gap_size as i32)
            .y(y+self.gap_size as i32)
            .width(width - (self.gap_size*2))
            .height(height - (self.gap_size*2));

        let window_aux = ConfigureWindowAux::new()
            .x(x+(self.border_width + self.gap_size) as i32)
            .y(y+(self.border_width + self.gap_size) as i32)
            .width(width - (self.border_width*2) - (self.gap_size*2))
            .height(height - (self.border_width*2) - (self.gap_size*2));

        self.connection.configure_window(self.frame, &frame_aux).unwrap();
        self.connection.configure_window(self.window, &window_aux).unwrap();

        return self;
    }

    pub fn draw(&self) {
        self.connection.grab_server().unwrap();

        self.connection.map_window(self.frame).unwrap();
        self.connection.map_window(self.window).unwrap();

        self.connection.ungrab_server().unwrap();
        self.connection.flush().unwrap();
    }
}
