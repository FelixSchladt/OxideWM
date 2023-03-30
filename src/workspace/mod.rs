pub mod parse_error;
pub mod workspace_layout;
pub mod workspace_navigation;

use self::workspace_layout::WorkspaceLayout;

use crate::{
    atom::Atom,
    auxiliary::{atom_name, get_internal_atom},
    config::Config,
    screeninfo::ScreenSize,
    windowmanager::movement::Movement,
    windowstate::WindowState,
};

use log::{debug, error, info, warn};
use oxide_common::ipc::state::WorkspaceDto;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::{cell::RefCell, rc::Rc};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use x11rb::CURRENT_TIME;

#[derive(Debug, Clone, Serialize)]
pub struct Workspace {
    #[serde(skip_serializing)]
    pub connection: Arc<RustConnection>,
    pub name: u16,
    #[serde(skip_serializing)]
    pub root_screen: Rc<RefCell<Screen>>,
    #[serde(skip_serializing)]
    pub screen_size: Rc<RefCell<ScreenSize>>,
    #[serde(skip_serializing)]
    pub config: Rc<RefCell<Config>>,
    pub focused_window: Option<u32>,
    pub fullscreen: Option<u32>,
    pub urgent: bool,
    pub windows: HashMap<u32, WindowState>,
    pub order: Vec<u32>,
    pub layout: WorkspaceLayout,
}

impl Workspace {
    pub fn new(
        name: u16,
        connection: Arc<RustConnection>,
        root_screen: Rc<RefCell<Screen>>,
        screen_size: Rc<RefCell<ScreenSize>>,
        config: Rc<RefCell<Config>>,
    ) -> Workspace {
        let default_layout = config.borrow().default_layout.clone();
        Workspace {
            connection,
            name,
            root_screen,
            screen_size,
            config,
            focused_window: None,
            fullscreen: None,
            urgent: false,
            windows: HashMap::new(),
            order: Vec::new(),
            layout: default_layout,
        }
    }

    pub fn to_dto(&self) -> WorkspaceDto {
        let windows = self
            .windows
            .iter()
            .map(|(key, state)| (*key, state.to_dto()))
            .collect();

        WorkspaceDto {
            name: self.name,
            layout: self.layout.to_string(),
            focused_window: self.focused_window,
            fullscreen: self.fullscreen,
            urgent: self.urgent,
            windows,
            order: self.order.clone(),
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
                }
                Movement::Right => {
                    if pos == len - 1 {
                        self.focus_window(self.order[0]);
                    } else {
                        self.focus_window(self.order[pos + 1]);
                    }
                }
                Movement::Up => {
                    //TODO: blocked by https://github.com/DHBW-FN/OxideWM/issues/25
                }
                Movement::Down => {
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
                }
                Movement::Right => {
                    if pos == self.order.len() - 1 {
                        self.order.swap(pos, 0);
                    } else {
                        self.order.swap(pos, pos + 1);
                    }
                    move_occured = Some(focused_win);
                }
                Movement::Up => {
                    //TODO: blocked by https://github.com/DHBW-FN/OxideWM/issues/25
                }
                Movement::Down => {
                    //TODO: blocked by https://github.com/DHBW-FN/OxideWM/issues/25
                }
            }
            self.remap_windows();
        }
        return move_occured;
    }

    pub fn rename(&mut self, name: u16) {
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
                }
                None => {
                    self.fullscreen = Some(focused_win);
                }
            }
            self.remap_windows();
        } else {
            error!("No window focused");
        }
    }

    pub fn kill_all_windows(&mut self) {
        let windows: HashSet<u32> = self
            .windows
            .keys()
            .map(|window| (*window).clone())
            .collect();

        self.windows.clear();
        self.order.clear();

        for window in windows.iter() {
            if self.connection.unmap_window(*window).is_err() {
                warn!("failed to unmap window {}", window);
            }
            if self.connection.kill_client(*window).is_err() {
                warn!("failed to kill client {}", window);
            }
        }

        if self.connection.flush().is_err() {
            warn!("failed to flush connection")
        }
    }

    fn is_softkill_supported(&self, winid: u32) -> bool {
        let atom_id = get_internal_atom(&self.connection, Atom::WmProtocols.as_ref());

        self.connection.flush().unwrap();
        let atom_reply = self
            .connection
            .get_property(false, winid, atom_id, AtomEnum::ANY, 0, 1024)
            .unwrap()
            .reply();

        if let Ok(atom_reply) = atom_reply {
            self.connection.flush().unwrap();

            let prop_type = match atom_reply.type_ {
                0 => return false, // Null response
                atomid => atom_name(&self.connection, atomid),
            };

            if prop_type == "ATOM" {
                let atoms = atom_reply
                    .value32()
                    .unwrap()
                    .map(|a| atom_name(&self.connection, a))
                    .collect::<Vec<String>>();

                return atoms.iter().any(|p| p == "WM_DELETE_WINDOW");
            }
        }
        false
    }

    pub fn kill_window(&mut self, winid: &u32) {
        if self.is_softkill_supported(*winid) {
            self.execute_softkill(*winid);
        } else {
            self.connection
                .kill_client(*winid)
                .expect("Could not kill client");
            self.remove_window(winid);
        }
        self.connection.flush().unwrap();
    }

    fn execute_softkill(&mut self, winid: u32) {
        let type_ = get_internal_atom(&self.connection, Atom::WmProtocols.as_ref());

        let data = ClientMessageData::from([
            get_internal_atom(&self.connection, Atom::WmDeleteWindow.as_ref()),
            0,
            0,
            0,
            0,
        ]);

        let clnt_msg_vnt = ClientMessageEvent {
            response_type: CLIENT_MESSAGE_EVENT,
            format: 32,
            sequence: 0,
            window: winid,
            type_,
            data,
        };

        self.connection
            .send_event(false, winid, EventMask::NO_EVENT, clnt_msg_vnt)
            .unwrap();
    }

    pub fn remove_window(&mut self, win_id: &u32) {
        if self.fullscreen == Some(*win_id) {
            self.fullscreen = None
        }
        self.windows.remove(&win_id);
        self.order.retain(|&x| x != *win_id);
        self.remap_windows();
        self.connection.grab_server().unwrap();
        let resp = &self.connection.unmap_window(*win_id as Window);
        if resp.is_err() {
            error!("An error occured while trying to unmap window");
        }
        self.connection.ungrab_server().unwrap();
        self.connection.flush().unwrap();
    }

    pub fn new_window(&mut self, window: Window) {
        let windowstruct = WindowState::new(
            self.connection.clone(),
            self.root_screen.clone(),
            self.config.clone(),
            window,
        );
        self.add_window(windowstruct);
    }

    pub fn show() {
        panic!("Not implemented");
    }
    pub fn hide() {
        panic!("Not implemented");
    }

    pub fn focus_window(&mut self, winid: u32) {
        debug!("focus_window");
        self.focused_window = Some(winid);
        if let Ok(result) = self
            .connection
            .set_input_focus(InputFocus::PARENT, winid, CURRENT_TIME)
        {
            if let Err(_) = result.check() {
                warn!("Failed to focus window");
            }
        } else {
            warn!("Failed to focus window");
        }
        //TODO: Change color of border to focus color
    }

    pub fn unfocus_window(&mut self) {
        self.focused_window = None;
        //TODO: Change color of border to unfocus color
    }

    pub fn set_layout(&mut self, layout: WorkspaceLayout) {
        self.layout = layout;
        self.remap_windows();
    }

    pub fn next_layout(&mut self) {
        match self.layout {
            WorkspaceLayout::HorizontalStriped => self.set_layout(WorkspaceLayout::VerticalStriped),
            WorkspaceLayout::VerticalStriped => self.set_layout(WorkspaceLayout::Tiled),
            WorkspaceLayout::Tiled => self.set_layout(WorkspaceLayout::HorizontalStriped),
        }
        self.remap_windows();
    }

    pub fn unmap_windows(&mut self) {
        debug!(
            "Unmapping {} Windows from workspace {}",
            self.windows.len(),
            self.name
        );
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
            window
                .set_bounds(0, 0, screen_size.width as u32, screen_size.height as u32)
                .draw_frameless();
            self.connection.flush().unwrap();
        } else {
            match self.layout {
                //Layout::Tiled => {},
                WorkspaceLayout::VerticalStriped => self.map_vertical_striped(),
                WorkspaceLayout::HorizontalStriped => self.map_horizontal_striped(),
                WorkspaceLayout::Tiled => self.map_tiled(),
            }
        }
    }

    fn map_vertical_striped(&mut self) {
        let amount = self.order.len();
        info!("Mapping {} windows with vertical striped layout.", amount);
        let screen_size = self.screen_size.borrow_mut();

        for (i, id) in self.order.iter().enumerate() {
            let current_window = self.windows.get_mut(id).unwrap();
            current_window
                .set_bounds(
                    (i * screen_size.ws_width as usize / amount) as i32 + screen_size.ws_pos_x,
                    screen_size.ws_pos_y,
                    (screen_size.ws_width as usize / amount) as u32,
                    screen_size.ws_height,
                )
                .draw();
        }
    }

    fn map_horizontal_striped(&mut self) {
        let amount = self.order.len();
        info!("Mapping {} windows with horizontal striped layout.", amount);
        let screen_size = self.screen_size.borrow_mut();

        for (i, id) in self.order.iter().enumerate() {
            let current_window = self.windows.get_mut(id).unwrap();
            current_window
                .set_bounds(
                    screen_size.ws_pos_x,
                    (i * screen_size.ws_height as usize / amount) as i32 + screen_size.ws_pos_y,
                    screen_size.ws_width,
                    (screen_size.ws_height as usize / amount) as u32,
                )
                .draw();
        }
    }

    fn map_tiled(&mut self) {
        let amount = self.order.len();

        if amount == 0 {
            return;
        } else if amount == 2 {
            self.map_vertical_striped();
            return;
        }

        let mut col = 0;
        let mut index = 0;
        let screen_size = self.screen_size.borrow();
        let even_amount = amount % 2 == 0;
        let window_height = screen_size.ws_height / 2;
        let window_width = match even_amount {
            true => screen_size.ws_width / (amount / 2) as u32,
            false => screen_size.ws_width / ((amount + 1) / 2) as u32,
        };

        if !even_amount {
            let window = self.windows.get_mut(&self.order[index]).unwrap();
            window
                .set_bounds(
                    screen_size.ws_pos_x,
                    screen_size.ws_pos_y,
                    window_width,
                    screen_size.ws_height,
                )
                .draw();
            index += 1;
            col += 1;
        }

        let mut x: u32;
        let mut y: u32;
        let mut is_upper_row: bool;
        while index < amount {
            let window = self.windows.get_mut(&self.order[index]).unwrap();

            is_upper_row = if even_amount {
                index % 2 == 0
            } else {
                index % 2 == 1
            };

            x = screen_size.ws_pos_x as u32 + (window_width * col);
            y = if is_upper_row {
                screen_size.ws_pos_y as u32
            } else {
                screen_size.ws_pos_y as u32 + window_height
            };

            window
                .set_bounds(x as i32, y as i32, window_width, window_height)
                .draw();

            if !is_upper_row {
                col += 1;
            }

            index += 1;
        }
    }
}
