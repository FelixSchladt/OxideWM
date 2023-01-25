pub mod error;

use x11rb::rust_connection::RustConnection;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use serde::Serialize;

use crate::windowstate::WindowState;
use std::{cell::RefCell, collections::HashMap};
use std::sync::Arc;
use std::rc::Rc;
use log::{info, debug, warn};
use crate::workspace::{Workspace, enum_workspace_navigation::EnumWorkspaceNavigation};

use self::error::{MoveError, QuitError};

const LOWEST_WORKSPACE_NR: u16 = 1;

#[derive(Debug)]
pub struct ScreenSize {
    pub width: u32,
    pub height: u32,
    pub ws_pos_x: i32,
    pub ws_pos_y: i32,
    pub ws_width: u32,
    pub ws_height: u32,
}

impl ScreenSize {
    pub fn default(width: u32, height: u32) -> ScreenSize {
        ScreenSize {
            width,
            height,
            ws_pos_x: 0,
            ws_pos_y: 0,
            ws_width: width,
            ws_height: height,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ScreenInfo {
    #[serde(skip_serializing)]
    connection: Arc<RustConnection>,
    #[serde(skip_serializing)]
    screen_ref: Rc<RefCell<Screen>>,
    workspaces: HashMap<u16, Workspace>,
    pub active_workspace: u16,
    #[serde(skip_serializing)]
    pub screen_size: Rc<RefCell<ScreenSize>>,
    pub status_bar: Option<WindowState>,
}

impl ScreenInfo {
    pub fn new(connection: Arc<RustConnection>, screen_ref: Rc<RefCell<Screen>>, width: u32, height: u32) -> ScreenInfo {
        let active_workspace = LOWEST_WORKSPACE_NR;
        let workspaces = HashMap::new();
        let screen_size = Rc::new(RefCell::new(ScreenSize::default(width, height)));
        let mut screen_info = ScreenInfo {
            connection,
            screen_ref,
            workspaces,
            active_workspace,
            screen_size,
            status_bar: None,
        };
        screen_info.create_workspace(LOWEST_WORKSPACE_NR);
        screen_info
    }

    fn create_status_bar_window(&mut self, event: &CreateNotifyEvent) {
        let status_bar = self.status_bar.as_mut().unwrap();
        let window_aux = ConfigureWindowAux::new().x(status_bar.x).y(status_bar.y).width(event.width as u32).height(event.height as u32);
        self.connection.configure_window(event.window, &window_aux).unwrap();
        self.connection.map_window(event.window).unwrap();
        self.connection.flush().unwrap();

    }

    pub fn add_status_bar(&mut self, event: &CreateNotifyEvent) {
        self.status_bar = Some(WindowState::new(self.connection.clone(), self.screen_ref.clone(), event.window));
        
        {
            let mut screen_size = self.screen_size.borrow_mut();

            //TODO: if the status bar is on the left or right
            //if the status bar is on the bottom
            let mut status_bar = self.status_bar.as_mut().unwrap();
            if event.y as i32 == (screen_size.height - (event.height as u32)) as i32 {
                screen_size.ws_height = screen_size.height - event.height as u32;
                screen_size.ws_pos_y = status_bar.height as i32;
            } //everything else will land on the top position
            else {
                screen_size.ws_pos_y = event.height as i32;
                screen_size.ws_height = screen_size.height - event.height as u32;
                status_bar.x = 0;
                status_bar.y = 0;
            }
            status_bar.width = event.width as u32;
            status_bar.height = event.height as u32;
            info!("Workspaceposition updated to x: {}, y: {}, width: {}, height: {}", screen_size.ws_pos_x, screen_size.ws_pos_y, screen_size.ws_width, screen_size.ws_height);
        }

        self.create_status_bar_window(event);
        self.status_bar.as_mut().unwrap().draw();

        //update the workspaces
        for (_, workspace) in self.workspaces.iter_mut() {
            workspace.remap_windows();
        }
    }

    fn create_workspace(&mut self, workspace_nr: u16) -> &mut Workspace {
        let new_workspace = Workspace::new(
            workspace_nr,
            self.connection.clone(),
            self.screen_ref.clone(),
            self.screen_size.clone(),
        );

        self.workspaces.entry(workspace_nr).or_insert(new_workspace)
    }

    pub fn get_workspace(&mut self, workspace_nr: u16) -> Option<&mut Workspace> {
        self.workspaces.get_mut(&workspace_nr)
    }

    pub fn get_active_workspace(&mut self) -> Option<&mut Workspace> {
        self.get_workspace(self.active_workspace)
    }

    pub fn on_map_request(&mut self, event: &MapRequestEvent) {
        info!("WINMAN: MapRequestEvent: {:?}", event);
        let workspace_option = self.get_active_workspace();
        match workspace_option{
            Some(workspace)=>{
                workspace.new_window(event.window);
                workspace.remap_windows();
            },
            None => warn!("could not handle map request, no active workspace")
        }
    }

    pub fn quit_workspace(&mut self, workspace_name: u16) -> Result<(),QuitError>{
        match self.workspaces.remove(&workspace_name) {
            Some(mut workspace) => {
                workspace.kill_all_windows();
                let new_workspace = match self.find_next_lowest_workspace_nr(){
                    Some(number) => {
                        debug!("using next lowest workspace {}", number);
                        number
                    },
                    None => match self.find_next_highest_workspace_nr(){
                        Some(number) =>{
                            debug!("using next highest workspace {}", number);
                            number
                        },
                        None => {
                            // removed last workspace
                            debug!("quit last workspace, creating {}", LOWEST_WORKSPACE_NR);
                            self.create_workspace(LOWEST_WORKSPACE_NR).name
                        },
                    },
                };
                info!("quit workspace {}, switching to {}", self.active_workspace, new_workspace);
                if self.set_workspace(new_workspace).is_err() {
                    return Err(QuitError::new(format!("failed to set new workspace, after no workspace was present")));
                }
                Ok(())
            }
            None =>Err(QuitError::new(format!("now workspace with workspace_name {}", workspace_name)))
        }
    }

    pub fn move_window_to_workspace_and_follow(&mut self, arg: EnumWorkspaceNavigation) -> Result<(),MoveError> {
        match self.get_next_workspace_nr(arg) {
            Ok(next_workspace) => {
                if self.move_window_to_workspace_nr(next_workspace).is_err() {
                    let error_msg =format!("failed to move window to workspace nr {}", next_workspace); 
                    return Err(MoveError::new(error_msg));
                }
                if self.set_workspace(next_workspace).is_err() {
                    let error_msg =format!("failed to set new workspace {}", next_workspace); 
                    return Err(MoveError::new(error_msg));
                }
                Ok(())
            },
            Err(error) => Err(error)
        }
    }


    pub fn move_window_to_workspace(&mut self, arg: EnumWorkspaceNavigation) -> Result<(),MoveError> {
        match self.get_next_workspace_nr(arg) {
            Ok(next_workspace) => {
                if self.move_window_to_workspace_nr(next_workspace).is_err() {
                    let error_msg =format!("failed to move window to workspace nr {}", next_workspace); 
                    return Err(MoveError::new(error_msg));
                }
                Ok(())
            },
            Err(error) => Err(error)
        }
    }

    fn get_next_workspace_nr(&self, arg: EnumWorkspaceNavigation) -> Result<u16,MoveError> {
        match arg {
            EnumWorkspaceNavigation::Next => Ok(self.find_next_workspace()),
            EnumWorkspaceNavigation::Previous => Ok(self.find_previous_workspace()),
            EnumWorkspaceNavigation::Number(number) => {
                if number >= LOWEST_WORKSPACE_NR {
                    Ok(number)
                }else{
                    let error_msg =format!("workspace nr {} has to be greater than or equal to {}", number, LOWEST_WORKSPACE_NR); 
                    Err(MoveError::new(error_msg))
                }
            }
        }
    }


    fn move_window_to_workspace_nr(&mut self, new_workspace_nr: u16) -> Result<(),MoveError> {
        if !self.workspaces.contains_key(&new_workspace_nr) {
            return Err(MoveError::new(format!("could not move screen, workspace {} does not exist on screen", new_workspace_nr)));
        }
        if self.active_workspace == new_workspace_nr {
            info!("window is already on desired workspace {}", new_workspace_nr);
            return Ok(());
        }

        let active_workspace = match self.get_active_workspace() {
            Some(workspace) => workspace,
            None => return Err(MoveError::new("No active workspace".to_string()))
        };

        let active_window = match active_workspace.get_focused_window() {
            Some(window) => window,
            None => return Err(MoveError::new("No active window".to_string())),
        };

        active_workspace.remove_window(&active_window);

        let windowsate = WindowState::new(self.connection.clone(), self.screen_ref.clone(), active_window);

        let new_workspace = match self.get_workspace(new_workspace_nr) {
            Some(workspace) => workspace,
            None => self.create_workspace(new_workspace_nr)
        };

        new_workspace.add_window(windowsate);
        Ok(())
    }

    pub fn move_to_or_create_workspace(&mut self, arg: EnumWorkspaceNavigation) -> Result<(),MoveError> {
        let workspace_nr = match arg {
            EnumWorkspaceNavigation::Next => {
                if self.active_workspace == u16::MAX{
                    LOWEST_WORKSPACE_NR
                }else{
                    self.active_workspace + 1
                }
            },
            EnumWorkspaceNavigation::Previous => {
                if self.active_workspace <= LOWEST_WORKSPACE_NR {
                    let highest_workspace = self.workspaces.keys().max();
                    if let Some(workspace_nr) = highest_workspace {
                        if *workspace_nr == u16::MAX{
                            *workspace_nr
                        }else{
                            *workspace_nr + 1
                        }
                    } else {
                        LOWEST_WORKSPACE_NR
                    }
                }else{
                    self.active_workspace - 1
                }
            },
            EnumWorkspaceNavigation::Number(number) => {
                if number < LOWEST_WORKSPACE_NR {
                    let error_msg =format!("workspace nr {} has to be greater than or equal to {}", number, LOWEST_WORKSPACE_NR); 
                    return Err(MoveError::new(error_msg));
                }
                number
            },
        };
        if !self.workspaces.contains_key(&workspace_nr) {
            self.create_workspace(workspace_nr);
        }
        if self.set_workspace(workspace_nr).is_err() {
            let error_msg =format!("failed to set workspace {}", workspace_nr); 
            return Err(MoveError::new(error_msg));
        }
        Ok(())
    }

    pub fn switch_workspace(&mut self, arg: EnumWorkspaceNavigation) -> Result<(),MoveError> {
        let new_workspace_nr = match self.get_next_workspace_nr(arg) {
            Ok(next_workspace) => next_workspace,
            Err(error) => return Err(error)
        };

        if !self.workspaces.contains_key(&new_workspace_nr) {
            return Err(MoveError::new(format!("could not move screen, workspace {} does not exist on screen", new_workspace_nr)));
        }
        if self.active_workspace == new_workspace_nr {
            info!("window is already on desired workspace {}", new_workspace_nr);
            return Ok(());
        }

        if self.set_workspace(new_workspace_nr).is_err() {
            return Err(MoveError::new(format!("could not move set workspace {}", new_workspace_nr)));
        };

        Ok(())
    }

     fn find_next_workspace(&self) -> u16 {
        if let Some(next_workspace) = self.find_next_highest_workspace_nr(){
            next_workspace
        }else{
            if let Some(first_workspace) = self.find_lowest_workspace(){
                first_workspace
            }else{
                warn!("in a state where no workspace was selected");
                LOWEST_WORKSPACE_NR
            }
        }
    }

    fn find_previous_workspace(&self) -> u16 {
        if let Some(previous_workspace) = self.find_next_lowest_workspace_nr(){
            previous_workspace
        }else{
            if let Some(last_workspace) = self.find_highest_workspace(){
                last_workspace
            }else{
                warn!("in a state where no workspace was selected");
                LOWEST_WORKSPACE_NR
            }
        }
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
    pub fn set_workspace(&mut self, workspace_nr: u16) -> Result<(),()>{
        debug!("Changing workspace from {} to {}", self.active_workspace, workspace_nr);

        if let Some(active_workspace) = self.workspaces.get_mut(&self.active_workspace) {
            active_workspace.unmap_windows();
            active_workspace.focused = false;    
        };

        let new_workspace = match self.workspaces.get_mut(&workspace_nr) {
            Some(workspace)=> workspace,
            None=> return Err(())
        };

        self.active_workspace = workspace_nr;
        new_workspace.remap_windows();
        new_workspace.focused = true;
        Ok(())
    }

    pub fn get_workspace_count(&self) -> usize{
        return self.workspaces.len();
    }
}
