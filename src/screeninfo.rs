use x11rb::rust_connection::RustConnection;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use serde::Serialize;

use crate::windowstate::WindowState;
use crate::workspace::{Workspace, enums_workspace::EnumWorkspaceNavigation};
use std::{cell::RefCell, rc::Rc, collections::HashMap};
use log::{info, debug, warn};

const LOWEST_WORKSPACE_NR: u16 = 1;

#[derive(Debug, Clone, Serialize)]
pub struct ScreenInfo {
    #[serde(skip_serializing)]
    _connection: Rc<RefCell<RustConnection>>,
    #[serde(skip_serializing)]
    _screen_ref: Rc<RefCell<Screen>>,
    workspaces: HashMap<u16, Workspace>,
    active_workspace: u16,
    pub ws_pos_x: i32,
    pub ws_pos_y: i32,
    pub ws_width: u32,
    pub ws_height: u32,
    pub width: u32,
    pub height: u32,
    pub status_bar: Option<WindowState>,
}

impl ScreenInfo {
    pub fn new(connection: Rc<RefCell<RustConnection>>, screen_ref: Rc<RefCell<Screen>>, height: u32, width: u32) -> ScreenInfo {
        let active_workspace = LOWEST_WORKSPACE_NR;
        let workspaces = HashMap::new();
        let mut screen_info = ScreenInfo {
            _connection: connection,
            _screen_ref: screen_ref,
            workspaces,
            active_workspace,
            ws_pos_x: 0,
            ws_pos_y: 0,
            ws_width: width,
            ws_height: height,
            width,
            height,
            status_bar: None,
        };
        screen_info.create_workspace(LOWEST_WORKSPACE_NR);
        screen_info
    }

    fn create_status_bar_window(&mut self, event: &CreateNotifyEvent) {
        let status_bar = self.status_bar.as_mut().unwrap();
        let conn = self._connection.borrow_mut();
        let window_aux = ConfigureWindowAux::new().x(status_bar.x).y(status_bar.y).width(event.width as u32).height(event.height as u32);
        conn.configure_window(event.window, &window_aux).unwrap();
        conn.map_window(event.window).unwrap();
        conn.flush().unwrap();

    }

    pub fn add_status_bar(&mut self, event: &CreateNotifyEvent) {
        self.status_bar = Some(WindowState::new(self._connection.clone(), &self._screen_ref.borrow(), event.window));

        //TODO: if the status bar is on the left or right
        //if the status bar is on the bottom
        let status_bar = self.status_bar.as_mut().unwrap();
        if event.y as i32 == (self.height - (event.height as u32)) as i32 {
            self.ws_height = self.height - event.height as u32;
            self.ws_pos_y = status_bar.height as i32;
        } //everything else will land on the top position
        else {
            self.ws_pos_y = event.height as i32;
            self.ws_height = self.height - event.height as u32;
            status_bar.x = 0;
            status_bar.y = 0;
        }
        status_bar.width = event.width as u32;
        status_bar.height = event.height as u32;

        self.create_status_bar_window(event);
        self.status_bar.as_mut().unwrap().draw();

        info!("Workspaceposition updated to x: {}, y: {}, width: {}, height: {}", self.ws_pos_x, self.ws_pos_y, self.ws_width, self.ws_height);
        //update the workspaces
        for (_, workspace) in self.workspaces.iter_mut() {
            workspace.set_bounds(self.ws_pos_x, self.ws_pos_y, self.ws_width, self.ws_height);
            workspace.remap_windows();
        }
    }

    pub fn create_new_workspace(&mut self){
        let mut index = u16::MAX;
        for i in 1..u16::MAX{
            if !self.workspaces.contains_key(&i){
                index = i;
                break;
            }
        }
        self.create_workspace(index);
        self.set_workspace_create_if_not_exists(index);
    }

    fn create_workspace(&mut self, workspace_nr: u16){
        if self.workspaces.contains_key(&workspace_nr) {
            debug!("workspace was already present {}", workspace_nr);
            return;
        }

        let new_workspace = Workspace::new(
            workspace_nr.to_string(),
            self._connection.clone(),
            self._screen_ref.clone(),
            self.ws_pos_x,
            self.ws_pos_y,
            self.ws_width,
            self.ws_height,
        );
        self.workspaces.insert(workspace_nr, new_workspace);
    }

    pub fn get_workspace(&mut self, workspace_nr: u16) -> &mut Workspace {
        if self.workspaces.contains_key(&workspace_nr){
            self.workspaces.get_mut(&workspace_nr).unwrap()
        }else{
            self.set_workspace_create_if_not_exists(workspace_nr)
        }
    }

    pub fn get_active_workspace(&mut self) -> &mut Workspace{
        self.get_workspace(self.active_workspace)
    }

    pub fn on_map_request(&mut self, event: &MapRequestEvent) {
        info!("WINMAN: MapRequestEvent: {:?}", event);
        let workspace_option = self.workspaces.get_mut(&self.active_workspace.clone());
        match workspace_option{
            Some(workspace)=>{
                workspace.new_window(event.window);
                workspace.remap_windows();
            },
            None => {
                self.set_workspace_create_if_not_exists(0)
                    .remap_windows();
            }
        }
    }

    pub fn switch_workspace(&mut self, arg: EnumWorkspaceNavigation) {
        match arg {
            EnumWorkspaceNavigation::Next => self.handle_go_to_workspace_next(),
            EnumWorkspaceNavigation::Previous => self.handle_go_to_workspace_previous(),
            EnumWorkspaceNavigation::Number(number) => {
                if number >= LOWEST_WORKSPACE_NR {
                    self.set_workspace_create_if_not_exists(number);
                }else{
                    warn!("workspace nr {} has to be greater than or equal to {}", number, LOWEST_WORKSPACE_NR)
                };
            },
        };
    }

    fn handle_go_to_workspace_next(&mut self){
        if let Some(next_workspace) = self.find_next_highest_workspace_nr(){
            self.set_workspace_create_if_not_exists(next_workspace);
        }else{
            if let Some(first_workspace) = self.find_lowest_workspace(){
                self.set_workspace_create_if_not_exists(first_workspace);
            }else{
                warn!("in a state where no workspace was selected");
                self.set_workspace_create_if_not_exists(LOWEST_WORKSPACE_NR);
            };
        };
    }

    fn handle_go_to_workspace_previous(&mut self){
        if let Some(previous_workspace) = self.find_next_lowest_workspace_nr(){
            self.set_workspace_create_if_not_exists(previous_workspace);
        }else{
            if let Some(last_workspace) = self.find_highest_workspace(){
                self.set_workspace_create_if_not_exists(last_workspace);
            }else{
                warn!("in a state where no workspace was selected");
                self.set_workspace_create_if_not_exists(LOWEST_WORKSPACE_NR);
            };
        };
    }

    fn find_highest_workspace(&self) -> Option<u16> {
        self.workspaces.iter()
            .map(|(workspace_nr, _)| *workspace_nr)
            .max()
    }

    fn find_lowest_workspace(&self) -> Option<u16> {
        self.workspaces.iter()
            .map(|(workspace_nr, _)| *workspace_nr)
            .min()
    }

    fn find_next_highest_workspace_nr(&self) -> Option<u16> {
        self.workspaces.iter()
            .map(|(workspace_nr, _)| *workspace_nr)
            .filter(|workspace_nr| *workspace_nr > self.active_workspace)
            .min()
    }

    fn find_next_lowest_workspace_nr(&self) -> Option<u16> {
        self.workspaces.iter()
            .map(|(workspace_nr, _)| *workspace_nr)
            .filter(|workspace_nr| *workspace_nr < self.active_workspace)
            .max()
    }

    /// If the workspace with the passed workspace_nr does not exist, it will be created
    pub fn set_workspace_create_if_not_exists(&mut self, workspace_nr: u16) -> &mut Workspace{
        debug!("Changing workspace from {} to {}", self.active_workspace, workspace_nr);

        let active_workspace = self.get_workspace(self.active_workspace);
        active_workspace.unmap_windows();
        active_workspace.focused = false;

        if !self.workspaces.contains_key(&workspace_nr){
            self.create_workspace(workspace_nr)
        }

        self.active_workspace = workspace_nr;
        let new_workspace = self.workspaces.get_mut(&self.active_workspace).unwrap();
        new_workspace.remap_windows();
        new_workspace.focused = true;
        new_workspace
        
    }

    pub fn get_workspace_count(&self) -> usize{
        return self.workspaces.len();
    }
}
