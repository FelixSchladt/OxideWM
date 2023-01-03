use super::windowstate::WindowState;
use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::*;
use x11rb::CURRENT_TIME;
use std::collections::HashMap;

use std::{cell::RefCell, rc::Rc};

use crate::windowmanager::Movement;


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

    pub fn get_focused_window(&self) -> Option<u32> {
        for (_, window) in self.windows.iter() {
            if window.focused {
                return Some(window.window);
            }
        }
        None
    }

    pub fn move_focus(&mut self, mov: Movement) {
        let len = self.order.len();
        if let Some(focused_win) = self.get_focused_window() {
            if len > 1 {
                let pos = self.order.iter().position(|&x| x == focused_win).unwrap();
                match mov {
                    Movement::Left => {
                        if pos == 0 {
                            self.focus_window(self.order[len - 1]);
                        } else {
                            self.focus_window(self.order[pos - 1]);
                        }
                    },
                    Movement::Right => {
                        if pos == len - 1 {
                            self.focus_window(self.order[0]);
                        } else {
                            self.focus_window(self.order[pos + 1]);
                        }
                    },
                    Movement::Up => {
                        //TODO: blocked by https://github.com/DHBW-FN/OxideWM/issues/25
                    },
                    Movement::Down => {
                        //TODO: blocked by https://github.com/DHBW-FN/OxideWM/issues/25
                    },
                }
            }
        } else {
            //Shouldnt really happen but just in case
            if len > 0 {
                self.focus_window(self.order[0]);
            }
        }
    }

    pub fn move_window(&mut self, mov: Movement) -> Option<u32> {
        let len = self.order.len();
        let mut move_occured: Option<u32> = None; //Its hacky but works good
        if let Some(focused_win) = self.get_focused_window() {
            if len > 1 {
                let pos = self.order.iter().position(|&x| x == focused_win).unwrap();
                match mov {
                    Movement::Left => {
                        if pos == 0 {
                            self.order.swap(pos, len - 1);
                        } else {
                            self.order.swap(pos, pos - 1);
                        }
                        move_occured = Some(focused_win);
                    },
                    Movement::Right => {
                        if pos == self.order.len() - 1 {
                            self.order.swap(pos, 0);
                        } else {
                            self.order.swap(pos, pos + 1);
                        }
                        move_occured = Some(focused_win);
                    },
                    Movement::Up => {
                        //TODO: blocked by https://github.com/DHBW-FN/OxideWM/issues/25
                    },
                    Movement::Down => {
                        //TODO: blocked by https://github.com/DHBW-FN/OxideWM/issues/25
                    },
                }
                self.remap_windows();
            }
        }
        return move_occured;
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
        for window in self.windows.values_mut() {
            window.focused = false;
        }
        self.connection.borrow().set_input_focus(InputFocus::PARENT, winid, CURRENT_TIME).unwrap().check().unwrap();
        self.windows.get_mut(&winid).unwrap().focused = true;
        //TODO: Chagnge color of border to focus color
    }

    pub fn unfocus_window(&mut self, winid: u32) {
        if let Some(win) = self.windows.get_mut(&winid) {
            win.focused = false;
        }
        //TODO: Change color of border to unfocus color
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
