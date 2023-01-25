pub mod enums_workspace;

use self::enums_workspace::Layout;

use crate::{
    windowmanager::enums_windowmanager::Movement,
    windowstate::WindowState,
    config::Config,
    screeninfo::ScreenSize,
};

use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::*;
use x11rb::CURRENT_TIME;
use std::collections::HashMap;
use serde::Serialize;
use std::{cell::RefCell, rc::Rc};
use std::sync::Arc;
use log::{error, info, debug};

#[derive(Debug, Clone, Serialize)]
pub struct Workspace {
    #[serde(skip_serializing)]
    pub connection:  Arc<RustConnection>,
    #[serde(skip_serializing)]
    pub root_screen: Rc<RefCell<Screen>>,
    #[serde(skip_serializing)]
    pub screen_size: Rc<RefCell<ScreenSize>>,
    #[serde(skip_serializing)]
    pub config: Rc<RefCell<Config>>,
    pub name: String,
    pub visible: bool,
    pub focused: bool,
    pub focused_window: Option<u32>,
    pub fullscreen: Option<u32>,
    pub urgent: bool,
    pub windows: HashMap<u32, WindowState>,
    pub order: Vec<u32>,
    pub layout: Layout,
}


impl Workspace {
    pub fn new(name:String, 
               connection: Arc<RustConnection>, 
               root_screen: Rc<RefCell<Screen>>, 
               screen_size: Rc<RefCell<ScreenSize>>,
               config: Rc<RefCell<Config>>
               ) -> Workspace 
    {
        Workspace {
            connection,
            name,
            root_screen,
            screen_size,
            config,
            visible: false,
            focused: false,
            focused_window: None,
            fullscreen: None,
            urgent: false,
            windows: HashMap::new(),
            order: Vec::new(),
            layout: Layout::HorizontalStriped,
        }
    }

    pub fn get_focused_window(&self) -> Option<u32> {
        return self.focused_window;
    }

    pub fn move_focus(&mut self, mov: Movement) {
        let len = self.order.len();
        if let Some(focused_win) = self.get_focused_window() {
            if len < 2 {
                return;
            }
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
            if len < 2 {
                return move_occured;
            }
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

    pub fn toggle_fullscreen(&mut self) {
        if let Some(focused_win) = self.get_focused_window() {
            match self.fullscreen {
                Some(_) => {
                    self.fullscreen = None;
                },
                None => {
                    self.fullscreen = Some(focused_win);
                }
            }
            self.remap_windows();
        } else {
            error!("No window focused");
        }

    }

    pub fn kill_window(&mut self, winid: &u32) {
        //TODO implement soft kill via client message over x
        //(Tell window to close itself)
        //https://github.com/DHBW-FN/OxideWM/issues/46
        self.connection.kill_client(*winid).expect("Could not kill client");
        self.connection.flush().unwrap();
        self.remove_window(winid);
    }

    pub fn remove_window(&mut self, win_id: &u32) {
        self.windows.remove(&win_id);
        self.order.retain(|&x| x != *win_id);
        self.remap_windows();
    }

    pub fn new_window(&mut self, window: Window) {
        let windowstruct = WindowState::new(self.connection.clone(), &self.root_screen.borrow(), self.config.clone(), window);
        self.add_window(windowstruct);
    }

    pub fn show() { panic!("Not implemented"); }
    pub fn hide() { panic!("Not implemented"); }

    pub fn focus_window(&mut self, winid: u32) {
        debug!("focus_window");
        self.focused_window = Some(winid);
        self.connection.set_input_focus(InputFocus::PARENT, winid, CURRENT_TIME).unwrap().check().unwrap();
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
        self.connection.grab_server().unwrap();
        for window_id in self.windows.keys() {
            let resp = &self.connection.unmap_window(*window_id as Window);
            if resp.is_err() {
                error!("An error occured while trying to unmap window");
            }
        }
        self.connection.ungrab_server().unwrap();
        self.connection.flush().unwrap();
    }

    pub fn remap_windows(&mut self) {
        if let Some(fs_win) = self.fullscreen {
            self.unmap_windows();
            let screen_size = self.screen_size.borrow();
            let window = self.windows.get_mut(&fs_win).unwrap();
            window.set_bounds(
                0,
                0,
                screen_size.width as u32,
                screen_size.height as u32,
            ).draw();
            self.connection.flush().unwrap();
        } else {
            match self.layout {
                //Layout::Tiled => {},
                Layout::VerticalStriped => self.map_vertical_striped(),
                Layout::HorizontalStriped => self.map_horizontal_striped(),
            }
        }
    }

    fn map_vertical_striped(&mut self) {
        let amount = self.order.len();
        info!("Mapping {} windows with vertical striped layout.", amount);
        let screen_size = self.screen_size.borrow_mut();

        for (i, id) in self.order.iter().enumerate() {
            let current_window = self.windows.get_mut(id).unwrap();
            current_window.set_bounds(
                (i * screen_size.ws_width as usize / amount) as i32 + screen_size.ws_pos_x,
                screen_size.ws_pos_y,
                (screen_size.ws_width as usize / amount) as u32,
                screen_size.ws_height
            ).draw();
        }
    }

    fn map_horizontal_striped(&mut self) {
        let amount = self.order.len();
        info!("Mapping {} windows with horizontal striped layout.", amount);
        let screen_size = self.screen_size.borrow_mut();

        for (i, id) in self.order.iter().enumerate() {
            let current_window = self.windows.get_mut(id).unwrap();
            current_window.set_bounds(
                screen_size.ws_pos_x,
                (i * screen_size.ws_height as usize / amount) as i32 + screen_size.ws_pos_y,
                screen_size.ws_width,
                (screen_size.ws_height as usize / amount) as u32,
            ).draw();
        }
    }
}
