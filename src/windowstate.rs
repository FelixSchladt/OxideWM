use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use x11rb::connection::Connection;
use x11rb::COPY_DEPTH_FROM_PARENT;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct WindowState {
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
    pub titlebar_height: u32,
    pub border_width: u32,
}

impl WindowState {
    pub fn new(connection: Rc<RefCell<RustConnection>>, root_screen: &Screen, window: Window) -> WindowState {
        let title = connection.borrow()
                              .get_property(
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
        let titlebar_height = 0;
        let border_width = 0;

        let frame = connection.borrow().generate_id().unwrap();
        connection.borrow().create_window(
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
            &CreateWindowAux::new().background_pixel(root_screen.black_pixel),
        ).unwrap();

        let mask = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::ENTER_WINDOW | EventMask::LEAVE_WINDOW );
        let res = connection.borrow().change_window_attributes(window, &mask).unwrap().check();
        if let Err(e) = res {
            println!("Error couldn change mask: {:?}", e);
            panic!("Error couldnt change mask");
        }

        WindowState {
            connection,
            frame,
            window,
            title,
            visible,
            urgent,
            x,
            y,
            width,
            height,
            titlebar_height,
            border_width,
        }
    }

    pub fn set_bounds(&self, x: i32, y: i32, width: u32, height: u32) {
        let frame_aux = ConfigureWindowAux::new()
            .x(x)
            .y(y)
            .width(width)
            .height(height);

        let window_aux = ConfigureWindowAux::new()
            .x(x+(self.border_width+self.titlebar_height) as i32)
            .y(y+self.border_width as i32)
            .width(width-(self.border_width*2))
            .height(height-self.titlebar_height-(self.border_width*2));

        self.connection.borrow().configure_window(self.frame, &frame_aux).unwrap();
        self.connection.borrow().configure_window(self.window, &window_aux).unwrap();
    }

    pub fn map(&self) {
        let con_b = self.connection.borrow();
        con_b.grab_server().unwrap();
        con_b.map_window(self.frame).unwrap();
        con_b.map_window(self.window).unwrap();
        con_b.ungrab_server().unwrap();
        con_b.flush().unwrap();
    }
}
