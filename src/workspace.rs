use super::windowstate::WindowState;
use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::*;
use x11rb::CURRENT_TIME;
use std::collections::HashMap;

use std::{cell::RefCell, rc::Rc};


#[derive(Debug)]
pub enum Layout {
    TILED,
    VERTICAL_STRIPED,   //  |
    HORIZONTAL_STRIPED, // ---
    //Different layout modes and better names wanted C:
}

#[derive(Debug)]
pub struct Workspace {
    pub connection:  Rc<RefCell<RustConnection>>,
    pub name: String,
    pub index: u16,
    pub root_window_index: u16,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub windows: HashMap<u32, WindowState>,
    pub order: Vec<u32>,
    pub layout: Layout,
    pub x: i32,         //Used to resize the entire workspace, e.g. to make room for taskbars
    pub y: i32,
    pub height: u32,
    pub width: u32,
}


impl Workspace {
    pub fn new(index: u16, connection: Rc<RefCell<RustConnection>>, x: i32, y: i32, height: u32, width: u32) -> Workspace {
        Workspace {
            connection: connection,
            name: index.to_string(),
            index,
            root_window_index: 0,  //TODO get root window index from windowmanager
            visible: false,
            focused: false,
            urgent: false,
            windows: HashMap::new(),
            order: Vec::new(),
            layout: Layout::TILED,
            x,
            y,
            height,
            width,
        }
    }

    pub fn get_focused_window(&self) -> Option<u32> {
        for (_, window) in self.windows.iter() {
            if window.focused {
                return Some(window.window);
            }
        }
        None
    }

    pub fn rename(&mut self, name: String) {
        //TODO: Check if name is already taken
        //TODO: Check if name is valid (not too long, etc.)
        self.name = name;
    }

    pub fn add_window(&mut self, win: WindowState) {
        self.order.push(win.window);
        self.windows.insert(win.window, win);
    }

    pub fn kill_window(&mut self, winid: &u32) {
        //TODO implement soft kill via client message over x
        //(Tell window to close itself)
        //https://github.com/DHBW-FN/OxideWM/issues/46
        self.connection.borrow().kill_client(*winid).expect("Could not kill client");
        self.connection.borrow().flush().unwrap();
        self.remove_window(winid);
    }

    pub fn remove_window(&mut self, win_id: &u32) {
        self.windows.remove(&win_id);
        self.order.retain(|&x| x != *win_id);
        self.remap_windows();
    }

    pub fn new_window(&mut self, window: Window) {
        let windowstruct = WindowState::new(self.connection.clone(), window);
        self.add_window(windowstruct);
    }

    //TODO: What is supposed to happen here?
    // if an window is hidden how does the user know it exists?
    // and does this make much sense in an window manager?
    // the user could just move an not wanted window to a different workspace
    pub fn show() { panic!("Not implemented"); }
    pub fn hide() { panic!("Not implemented"); }

    pub fn focus_window(&mut self, winid: u32) {
        self.connection.borrow().set_input_focus(InputFocus::PARENT, winid, CURRENT_TIME).unwrap().check().unwrap();
        self.windows.get_mut(&winid).unwrap().focused = true;
        //TODO: Chagnge color of border to focus color
        //
    }

    pub fn unfocus_window(&mut self, winid: u32) {
        if let Some(win) = self.windows.get_mut(&winid) {
            win.focused = false;
        }
        //TODO: Change color of border to unfocus color
    }

    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
        self.remap_windows();
    }

    pub fn remap_windows(&mut self) {
        match self.layout {
            Layout::TILED => {},
            Layout::VERTICAL_STRIPED => self.map_vertical_striped(),
            Layout::HORIZONTAL_STRIPED => self.map_horizontal_striped(),

        }
        for id in self.order.iter() {
            //TODO Add titlebar and Frame
            let win = self.windows.get(id).unwrap();
            let winaux = ConfigureWindowAux::new()
                .x(win.x)
                .y(win.y)
                .width(win.width)
                .height(win.height);
            let conn = self.connection.borrow();
            conn.configure_window(win.window, &winaux).unwrap();

            conn.grab_server().unwrap();
            conn.map_window(win.window).unwrap();
            conn.ungrab_server().unwrap();
            conn.flush().unwrap();
        }
    }

    fn map_vertical_striped(&mut self) {
        let amount = self.order.len();
        println!("\n\nMapping {} windows with vertical striped layout.", amount);

        for (i, id) in self.order.iter().enumerate() {
            let current_window = self.windows.get_mut(id).unwrap();

            current_window.x = (i * self.width as usize / amount) as i32;
            current_window.y = self.y;

            current_window.width  = (self.width as usize / amount) as u32;
            current_window.height = self.height;
        }
    }

    fn map_horizontal_striped(&mut self) {
        let amount = self.order.len();
        println!("\n\nMapping {} windows with horizontal striped layout.", amount);

        for (i, id) in self.order.iter().enumerate() {
            let current_window = self.windows.get_mut(id).unwrap();

            current_window.x = self.x;
            current_window.y = (i * self.height as usize / amount) as i32;

            current_window.width  = self.width;
            current_window.height = (self.height as usize / amount) as u32;
        }
    }
}
