use super::windowstate::WindowState;
use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::*;
use std::collections::HashMap;

use std::{cell::RefCell, rc::Rc};


#[derive(Debug)]
pub enum Layout {
    TILING,
    //Different layout modes and better names wanted C:
}

#[derive(Debug)]
pub struct Workspace {
    pub connection:  Rc<RefCell<RustConnection>>,
    pub name: String,
    pub index: u16,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub windows: HashMap<u32, WindowState>,
    pub order: Vec<u32>,
    pub layout: Layout,
    //this makes sense because if we have a bar we need have a different workspace size compared to
    //screen size. Additionally we will need to adjust the coordinates depending on the start of
    //the workspace or we map the bar as part of the workspace but i think this would be
    //unnecessary and not very efficient.
    pub x: i32,
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
            visible: false,
            focused: false,
            urgent: false,
            windows: HashMap::new(),
            order: Vec::new(),
            layout: Layout::TILING,
            x, 
            y,
            height,
            width,
        }
    }

    pub fn rename(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_window(&mut self, win: WindowState) {
        self.order.push(win.window);
        self.windows.insert(win.window, win);
    }

    pub fn new_window(&mut self, window: Window) {
        let windowstruct = WindowState::new(self.connection.clone(), window);
        self.add_window(windowstruct);
    }

    pub fn show() { panic!("Not implemented"); }
    pub fn hide() { panic!("Not implemented"); }

    pub fn open_window(executable: String) {
        panic!("Not implemented");
    }

    pub fn close_window(executable: String) {
        panic!("Not implemented");
    }

    pub fn remap_windows(&mut self) {
        match self.layout {
            Layout::TILING => self.tiling_layout(),
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

    fn tiling_layout(&mut self) {
        let amount = self.order.len();
        println!("\n\nAmount of windows: {}", amount);

        
        //TODO: Implement tiling layout
        for (i, id) in self.order.iter().enumerate() {
            let curwin = self.windows.get_mut(id).unwrap();
            //TODO How are we implementing the area the workspace filled?
            //The bar could be on the top or bottem, or maybey even on the side
            curwin.x = (i * self.width as usize / amount) as i32;
            curwin.y = self.y;
            curwin.width = (self.width as usize / amount) as u32;
            curwin.height = self.height;
        }
    }
}
