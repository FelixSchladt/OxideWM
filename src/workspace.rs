use super::windowstate::WindowState;
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
pub enum Layout {
    TILING,
    //Different layout modes and better names wanted C:
}

#[derive(Debug)]
pub struct Workspace {
    pub name: String,
    pub index: u16,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub windows: HashMap<u32, WindowState>,
    pub order: Vec<u32>,
    pub layout: Layout,
}


impl Workspace {
    pub fn new(index: u16) -> Workspace {
        Workspace {
            name: index.to_string(),
            index: index,
            visible: false,
            focused: false,
            urgent: false,
            windows: HashMap::new(),
            order: Vec::new(),
            layout: Layout::TILING,
        }
    }

    pub fn rename(&mut self, name: String) {
        self.name = name;
    }

    fn find_winid(&self, winid: &u32) -> bool {
        self.order.contains(&winid)
    }
    
    pub fn add_window(&mut self, win: WindowState) {
        self.order.push(win.window.clone());
        self.windows.insert(win.window, win);
    }

    pub fn new_window(&mut self, connection: &RustConnection, window: Window) {
        let windowstruct = WindowState::new(connection, window);
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

    pub fn remap_windows(&mut self, connection: &RustConnection) {
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
            connection.configure_window(win.window, &winaux).unwrap();

            connection.grab_server().unwrap();
            connection.map_window(win.window).unwrap();
            connection.ungrab_server().unwrap();
            connection.flush().unwrap();
        }
    }

    fn tiling_layout(&mut self) {
        let amount = self.order.len();
        println!("\n\nAmount of windows: {}", amount);
        /*
        for (i, window) in self.windows.iter_mut().enumerate() {
            window.x = (i * self.width / amount) as u16;
            window.y = 0;
            window.width = (self.width / amount) as u16;
            window.height = self.height as u16;
        }*/
        /*
        if amount==1 {
            for id in self.order.iter() {
                let curwin = self.windows.get_mut(id).unwrap();
                curwin.x = 0;
                curwin.y = 0;
                //TODO: Get screen size
                curwin.width = 1000;
                curwin.height = 1000;
            }
        } */
        //TODO: Implement tiling layout
        //TODO: GET ACTUAL SCREEN SIZE
        for (i, id) in self.order.iter().enumerate() {
            let curwin = self.windows.get_mut(id).unwrap();
            curwin.x = (i * 1000 / amount) as i32;
            curwin.y = 0;
            curwin.width = 1000/ amount as u32;
            curwin.height = 1000;
        }
    }
}
