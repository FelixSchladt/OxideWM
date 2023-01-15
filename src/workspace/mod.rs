pub mod enums;

use self::enums::Layout;

use crate::{
    windowmanager::enums_windowmanager::Movement,
    windowstate::WindowState,
};

use log::{debug, error, info};
use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::{ConnectionExt, InputFocus, Screen, Window};
use x11rb::CURRENT_TIME;
use std::collections::HashMap;
use serde::Serialize;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Serialize)]
pub struct Workspace {
    #[serde(skip_serializing)]
    pub connection:  Rc<RefCell<RustConnection>>,
    pub name: String,
    #[serde(skip_serializing)]
    pub root_screen: Rc<RefCell<Screen>>,
    pub visible: bool,
    pub focused: bool,
    pub focused_window: Option<u32>,
    pub urgent: bool,
    pub windows: HashMap<u32, WindowState>,
    pub order: Vec<u32>,
    pub layout: Layout,
    pub x: i32,
    pub y: i32,
    pub height: u32,
    pub width: u32,
}


impl Workspace {
    pub fn new(name:String ,connection: Rc<RefCell<RustConnection>>,root_screen: Rc<RefCell<Screen>>, x: i32, y: i32, height: u32, width: u32) -> Workspace {
        Workspace {
            connection,
            name,
            root_screen,
            visible: false,
            focused: false,
            focused_window: None,
            urgent: false,
            windows: HashMap::new(),
            order: Vec::new(),
            layout: Layout::HorizontalStriped,
            x,
            y,
            height,
            width,
        }
    }

    #[must_use]
    pub fn get_focused_window(&self) -> Option<u32> {
        return self.focused_window;
    }

    pub fn move_focus(&mut self, mov: &Movement) {
        let len = self.order.len();

        if let Some(focused_win) = self.get_focused_window() {
            if len < 2 {
                return;
            }

            let pos = if let Some(pos) = self.order.iter().position(|&x| x == focused_win) { pos } else {
                 error!("Failed to get position of window");
                 return;
            };

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
                Movement::Up | Movement::Down => {
                    //TODO: blocked by https://github.com/DHBW-FN/OxideWM/issues/25
                }
            }
        } else {
            //Shouldnt really happen but just in case
            if len > 0 {
                self.focus_window(self.order[0]);
            }
        }
    }

    pub fn move_window(&mut self, mov: &Movement) -> Option<u32> {
        let len = self.order.len();
        let mut move_occured: Option<u32> = None; //Its hacky but works good

        if let Some(focused_win) = self.get_focused_window() {
            if len < 2 {
                return move_occured;
            }

            let pos = if let Some(pos) = self.order.iter().position(|&x| x == focused_win) { pos } else {
                 error!("Failed to get position of window");
                 return None;
             };

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
                Movement::Up | Movement::Down => {
                    //TODO: blocked by https://github.com/DHBW-FN/OxideWM/issues/25
                }
            }
            self.remap_windows();
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
        self.connection.borrow().flush().ok();

        self.remove_window(winid);
    }

    pub fn remove_window(&mut self, win_id: &u32) {
        self.windows.remove(&win_id);
        self.order.retain(|&x| x != *win_id);

        self.remap_windows();
    }

    pub fn new_window(&mut self, window: Window) {
        let windowstruct = WindowState::new(self.connection.clone(), &self.root_screen.borrow(), window);
        self.add_window(windowstruct);
    }

    pub fn focus_window(&mut self, winid: u32) {
        debug!("focus_window");

        self.focused_window = Some(winid);

        match self.connection.borrow().set_input_focus(InputFocus::PARENT, winid, CURRENT_TIME) {
            Ok(cookie) => {
                if let Err(reason) = cookie.check() {
                    error!("`set_input_focus` request caused an X11 error:\n{}", reason);
                }
            },
            Err(reason) => {
                error!("Failed to est focus on window {} because {}", winid, reason);
            }
        }
        //TODO: Change color of border to focus color
    }

    pub fn unfocus_window(&mut self) {
        self.focused_window = None;
        //TODO: Change color of border to unfocus color
    }

    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
        self.remap_windows();
    }

    pub fn next_layout(&mut self) {
        match self.layout {
            Layout::HorizontalStriped => self.set_layout(Layout::VerticalStriped),
            Layout::VerticalStriped => self.set_layout(Layout::HorizontalStriped),
        }
        self.remap_windows();
    }

    pub fn unmap_windows(&mut self){
        debug!("Unmapping {} Windows from workspace {}", self.windows.len(), self.name);

        let conn = self.connection.borrow();
        conn.grab_server().ok();

        for window_id in self.windows.keys() {
            match &conn.unmap_window(*window_id as Window) {
                Ok(_) => {},
                Err(reason) => {
                    error!("Failed to unmap window because {}", reason);
                }
            };
        }

        conn.ungrab_server().ok();
        conn.flush().ok();
    }

    pub fn remap_windows(&mut self) {
        match self.layout {
            //Layout::Tiled => {},
            Layout::VerticalStriped => self.map_vertical_striped(),
            Layout::HorizontalStriped => self.map_horizontal_striped(),
        }
    }

    fn map_vertical_striped(&mut self) {
        let mut cast_i: i32;
        let mut current_window: &WindowState;
        let amount = u32::try_from(self.order.len()).unwrap();

        info!("\n\nMapping {} windows with vertical striped layout.", amount);

        for (i, id) in self.order.iter().enumerate() {
            cast_i = i32::try_from(i).unwrap();
            current_window = self.windows.get_mut(id).unwrap();

            current_window.set_bounds(
                cast_i * i32::try_from(self.width / amount).unwrap(),
                self.y,
                self.width / amount,
                self.height
            ).draw();
        }
    }

    fn map_horizontal_striped(&mut self) {
        let mut cast_i: i32;
        let mut current_window: &WindowState;
        let amount = u32::try_from(self.order.len()).unwrap();

        info!("Mapping {} windows with horizontal striped layout.", amount);

        for (i, id) in self.order.iter().enumerate() {
            cast_i = i32::try_from(i).unwrap();
            current_window = self.windows.get_mut(id).unwrap();

            current_window.set_bounds(
                self.x,
                cast_i * i32::try_from(self.height / amount).unwrap(),
                self.width,
                self.height / amount,
            ).draw();
        }
    }
}