pub mod error;

use self::error::{MoveError, QuitError};

use crate::{
    config::Config,
    windowstate::WindowState,
    workspace::{workspace_navigation::WorkspaceNavigation, Workspace},
};

use log::{debug, error, info, warn};
use oxide_common::ipc::state::{ScreenInfoDto, WorkspaceDto};
use std::sync::Arc;
use std::{cell::RefCell, collections::HashMap};
use std::{collections::HashSet, rc::Rc};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;

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

#[derive(Debug, Clone)]
pub struct ScreenInfo {
    connection: Arc<RustConnection>,
    screen_ref: Rc<RefCell<Screen>>,
    workspaces: HashMap<u16, Rc<RefCell<Workspace>>>,
    pub active_workspace: Rc<RefCell<Workspace>>,
    config: Rc<RefCell<Config>>,
    pub screen_size: Rc<RefCell<ScreenSize>>,
    pub status_bar: Option<WindowState>,
}

impl ScreenInfo {
    pub fn new(
        connection: Arc<RustConnection>,
        screen_ref: Rc<RefCell<Screen>>,
        config: Rc<RefCell<Config>>,
        width: u32,
        height: u32,
    ) -> ScreenInfo {
        let screen_size = Rc::new(RefCell::new(ScreenSize::default(width, height)));
        let active_workspace = Rc::new(RefCell::new(Workspace::new(
            LOWEST_WORKSPACE_NR,
            connection.clone(),
            screen_ref.clone(),
            screen_size.clone(),
            config.clone(),
        )));
        let mut workspaces = HashMap::new();
        workspaces.insert(LOWEST_WORKSPACE_NR, active_workspace.clone());
        let screen_info = ScreenInfo {
            connection,
            screen_ref,
            workspaces,
            active_workspace,
            config,
            screen_size,
            status_bar: None,
        };
        screen_info
    }

    pub fn to_dto(&self) -> ScreenInfoDto {
        let workspaces: HashMap<u16, WorkspaceDto> = self
            .workspaces
            .iter()
            .map(|(key, workspace)| (*key, workspace.borrow().to_dto()))
            .collect();

        info!(
            "fdsakjfadslkfjadsfldskjfdsafkljadsfadskjf dto with number{}",
            self.active_workspace.borrow().name
        );
        ScreenInfoDto {
            workspaces: workspaces,
            active_workspace: self.active_workspace.borrow().name,
        }
    }

    pub fn get_active_workspace(&self) -> Rc<RefCell<Workspace>> {
        self.active_workspace.clone()
    }

    fn create_status_bar_window(&mut self, event: &CreateNotifyEvent) {
        let status_bar = self.status_bar.as_mut().unwrap();
        let window_aux = ConfigureWindowAux::new()
            .x(status_bar.x)
            .y(status_bar.y)
            .width(event.width as u32)
            .height(event.height as u32);
        self.connection
            .configure_window(event.window, &window_aux)
            .unwrap();
        self.connection.map_window(event.window).unwrap();
        self.connection.flush().unwrap();
    }

    pub fn add_status_bar(&mut self, event: &CreateNotifyEvent) {
        self.status_bar = Some(WindowState::new(
            self.connection.clone(),
            self.screen_ref.clone(),
            self.config.clone(),
            event.window,
        ));

        {
            let mut screen_size = self.screen_size.borrow_mut();

            //TODO: if the status bar is on the left or right
            //if the status bar is on the bottom
            let mut status_bar = self.status_bar.as_mut().unwrap();
            if event.y as i32 == (screen_size.height - (event.height as u32)) as i32 {
                screen_size.ws_height = screen_size.height - event.height as u32;
                screen_size.ws_pos_y = status_bar.height as i32;
            }
            //everything else will land on the top position
            else {
                screen_size.ws_pos_y = event.height as i32;
                screen_size.ws_height = screen_size.height - event.height as u32;
                status_bar.x = 0;
                status_bar.y = 0;
            }
            status_bar.width = event.width as u32;
            status_bar.height = event.height as u32;
            status_bar.border_width = 0;
            status_bar.gap_size = 0;

            info!(
                "status bar with width {} and height {}",
                status_bar.width, status_bar.height
            );

            info!(
                "Workspaceposition updated to x: {}, y: {}, width: {}, height: {}",
                screen_size.ws_pos_x,
                screen_size.ws_pos_y,
                screen_size.ws_width,
                screen_size.ws_height
            );
        }

        self.create_status_bar_window(event);
        self.status_bar.as_mut().unwrap().draw();

        //update the workspaces
        for (_, workspace) in self.workspaces.iter_mut() {
            workspace.borrow_mut().remap_windows();
        }
    }

    fn create_workspace(&mut self, workspace_nr: u16) -> Rc<RefCell<Workspace>> {
        debug!("creating new workspace {}", workspace_nr);
        let new_workspace = Workspace::new(
            workspace_nr,
            self.connection.clone(),
            self.screen_ref.clone(),
            self.screen_size.clone(),
            self.config.clone(),
        );

        let workspace_rc = Rc::new(RefCell::new(new_workspace));
        self.workspaces
            .entry(workspace_nr)
            .or_insert(workspace_rc.clone())
            .clone()
    }

    pub fn is_window_on_active_workspace_selected(&mut self) -> bool {
        match self.active_workspace.borrow().get_focused_window() {
            Some(_) => true,
            None => false,
        }
    }

    pub fn on_map_request(&mut self, event: &MapRequestEvent) {
        info!("WINMAN: MapRequestEvent: {:?}", event);
        self.active_workspace.borrow_mut().new_window(event.window);
        self.active_workspace.borrow_mut().remap_windows();
    }

    pub fn quit_workspace_select_new(&mut self) -> Result<(), QuitError> {
        let new_workspace = match self.find_next_lowest_workspace_nr() {
            Some(number) => {
                debug!("using next lowest workspace {}", number);
                number
            }
            None => match self.find_next_highest_workspace_nr() {
                Some(number) => {
                    debug!("using next highest workspace {}", number);
                    number
                }
                None => {
                    // removed last workspace
                    debug!("quit last workspace, creating {}", LOWEST_WORKSPACE_NR);
                    self.create_workspace(LOWEST_WORKSPACE_NR).borrow().name
                }
            },
        };

        info!(
            "quit workspace {}, switching to {}",
            self.active_workspace.borrow().name,
            new_workspace
        );

        let quit_workspace = self.active_workspace.borrow().name;

        if self.set_workspace(new_workspace).is_err() {
            return Err(QuitError::new(format!(
                "failed to set new workspace, after no workspace was present"
            )));
        }

        // the workspace number did not change
        if quit_workspace == self.active_workspace.borrow().name {
            self.active_workspace.borrow_mut().kill_all_windows();
            Ok(())
        } else {
            match self.quit_workspace(quit_workspace) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        }
    }

    fn quit_workspace(&mut self, workspace_name: u16) -> Result<(), QuitError> {
        info!("quitting workspace {}", workspace_name);
        match self.workspaces.remove(&workspace_name) {
            Some(workspace) => {
                workspace.borrow_mut().kill_all_windows();
                Ok(())
            }
            None => Err(QuitError::new(format!(
                "now workspace with workspace_name {}",
                workspace_name
            ))),
        }
    }

    pub fn move_window_to_workspace_and_follow(
        &mut self,
        arg: WorkspaceNavigation,
    ) -> Result<(), MoveError> {
        match self.get_next_workspace_nr(arg.clone()) {
            Ok(next_workspace) => {
                if !self.is_window_on_active_workspace_selected() {
                    return Err(MoveError::new(
                        "No window selected on active workspace".to_string(),
                    ));
                }
                if arg.is_create_if_not_exists() && !self.workspaces.contains_key(&next_workspace) {
                    self.create_workspace(next_workspace);
                }
                debug!("Moveing window to workspace {}", next_workspace);
                if self.move_window_to_workspace_nr(next_workspace).is_err() {
                    let error_msg =
                        format!("failed to move window to workspace nr {}", next_workspace);
                    return Err(MoveError::new(error_msg));
                }
                debug!("Switching to workspace {}", next_workspace);
                if self.set_workspace(next_workspace).is_err() {
                    let error_msg = format!("failed to set new workspace {}", next_workspace);
                    return Err(MoveError::new(error_msg));
                }
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    pub fn move_window_to_workspace(&mut self, arg: WorkspaceNavigation) -> Result<(), MoveError> {
        match self.get_next_workspace_nr(arg.clone()) {
            Ok(next_workspace) => {
                if !self.is_window_on_active_workspace_selected() {
                    return Err(MoveError::new(
                        "No window selected on active workspace".to_string(),
                    ));
                }
                if arg.is_create_if_not_exists() && !self.workspaces.contains_key(&next_workspace) {
                    self.create_workspace(next_workspace);
                }
                debug!("Moveing window to workspace {}", next_workspace);
                if self.move_window_to_workspace_nr(next_workspace).is_err() {
                    let error_msg =
                        format!("failed to move window to workspace nr {}", next_workspace);
                    return Err(MoveError::new(error_msg));
                }
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    fn get_next_workspace_nr(&self, arg: WorkspaceNavigation) -> Result<u16, MoveError> {
        match arg {
            WorkspaceNavigation::Next => Ok(self.find_next_workspace()),
            WorkspaceNavigation::Previous => Ok(self.find_previous_workspace()),
            WorkspaceNavigation::NextFree => Ok(self.find_next_free_workspace()),
            WorkspaceNavigation::Number(number) => {
                if number >= LOWEST_WORKSPACE_NR {
                    Ok(number)
                } else {
                    let error_msg = format!(
                        "workspace nr {} has to be greater than or equal to {}",
                        number, LOWEST_WORKSPACE_NR
                    );
                    Err(MoveError::new(error_msg))
                }
            }
        }
    }

    fn move_window_to_workspace_nr(&mut self, new_workspace_nr: u16) -> Result<(), MoveError> {
        if !self.workspaces.contains_key(&new_workspace_nr) {
            return Err(MoveError::new(format!(
                "could not move screen, workspace {} does not exist on screen",
                new_workspace_nr
            )));
        }
        if self.active_workspace.borrow().name == new_workspace_nr {
            info!(
                "window is already on desired workspace {}",
                new_workspace_nr
            );
            return Ok(());
        }

        let active_window = match self.active_workspace.borrow().get_focused_window() {
            Some(window) => window,
            None => return Err(MoveError::new("No active window".to_string())),
        };

        self.active_workspace
            .borrow_mut()
            .remove_window(&active_window);

        let windowsate = WindowState::new(
            self.connection.clone(),
            self.screen_ref.clone(),
            self.config.clone(),
            active_window,
        );

        let new_workspace = match self.workspaces.get(&new_workspace_nr) {
            Some(workspace) => workspace.clone(),
            None => self.create_workspace(new_workspace_nr),
        };

        new_workspace.borrow_mut().add_window(windowsate);
        Ok(())
    }

    pub fn go_to_workspace(&mut self, arg: WorkspaceNavigation) -> Result<(), MoveError> {
        let new_workspace_nr = match self.get_next_workspace_nr(arg.clone()) {
            Ok(next_workspace) => next_workspace,
            Err(error) => return Err(error),
        };

        if arg.is_create_if_not_exists() && !self.workspaces.contains_key(&new_workspace_nr) {
            self.create_workspace(new_workspace_nr);
        }

        if !self.workspaces.contains_key(&new_workspace_nr) {
            return Err(MoveError::new(format!(
                "could not go to workspace {}, it does not exist on screen",
                new_workspace_nr
            )));
        }
        if self.active_workspace.borrow().name == new_workspace_nr {
            info!("already on desired workspace {}", new_workspace_nr);
            return Ok(());
        }

        if self.set_workspace(new_workspace_nr).is_err() {
            return Err(MoveError::new(format!(
                "could not set workspace {}",
                new_workspace_nr
            )));
        };

        Ok(())
    }

    pub fn find_next_free_workspace(&self) -> u16 {
        let existing_workspaces: HashSet<u16> = self.workspaces.keys().map(|ws| *ws).collect();

        let mut next_free_workspace = u16::MAX;
        for i in LOWEST_WORKSPACE_NR..u16::MAX {
            if !existing_workspaces.contains(&i) {
                next_free_workspace = i;
                break;
            }
        }
        next_free_workspace
    }

    fn find_next_workspace(&self) -> u16 {
        if let Some(next_workspace) = self.find_next_highest_workspace_nr() {
            next_workspace
        } else {
            if let Some(first_workspace) = self.find_lowest_workspace() {
                first_workspace
            } else {
                warn!("in a state where no workspace was selected");
                LOWEST_WORKSPACE_NR
            }
        }
    }

    fn find_previous_workspace(&self) -> u16 {
        if let Some(previous_workspace) = self.find_next_lowest_workspace_nr() {
            previous_workspace
        } else {
            if let Some(last_workspace) = self.find_highest_workspace() {
                last_workspace
            } else {
                warn!("in a state where no workspace was selected");
                LOWEST_WORKSPACE_NR
            }
        }
    }

    fn find_highest_workspace(&self) -> Option<u16> {
        self.workspaces
            .iter()
            .map(|(workspace_nr, _)| *workspace_nr)
            .max()
    }

    fn find_lowest_workspace(&self) -> Option<u16> {
        self.workspaces
            .iter()
            .map(|(workspace_nr, _)| *workspace_nr)
            .min()
    }

    fn find_next_highest_workspace_nr(&self) -> Option<u16> {
        self.workspaces
            .iter()
            .map(|(workspace_nr, _)| *workspace_nr)
            .filter(|workspace_nr| *workspace_nr > self.active_workspace.borrow().name)
            .min()
    }

    fn find_next_lowest_workspace_nr(&self) -> Option<u16> {
        self.workspaces
            .iter()
            .map(|(workspace_nr, _)| *workspace_nr)
            .filter(|workspace_nr| *workspace_nr < self.active_workspace.borrow().name)
            .max()
    }

    /// If the workspace with the passed workspace_nr does not exist, it will be created
    pub fn set_workspace(&mut self, workspace_nr: u16) -> Result<(), ()> {
        debug!(
            "Changing workspace from {} to {}",
            self.active_workspace.borrow().name,
            workspace_nr
        );

        let mut quit_ws: Option<u16> = None;
        self.active_workspace.borrow_mut().unmap_windows();
        if self.active_workspace.borrow().windows.is_empty() {
            quit_ws = Some(self.active_workspace.borrow().name);
        }

        if let Some(name) = quit_ws {
            if name != workspace_nr {
                info!(
                    "closing empty workspace {} since switching to {}",
                    name, workspace_nr
                );
                if let Err(error) = self.quit_workspace(name) {
                    warn!("failed to quit empty workspace {}", error)
                }
            }
        }

        let new_workspace = match self.workspaces.get_mut(&workspace_nr) {
            Some(workspace) => workspace,
            None => {
                error!("could not switch to new Workspace {}", workspace_nr);
                return Err(());
            }
        };

        self.active_workspace = new_workspace.clone();
        new_workspace.borrow_mut().remap_windows();
        Ok(())
    }

    pub fn get_workspace_count(&self) -> usize {
        return self.workspaces.len();
    }
}

#[cfg(test)]
impl ScreenInfo {
    pub fn set_test_workspaces(&mut self, workspaces: Vec<u16>) {
        self.workspaces.clear();
        let mut first = true;
        for ws in workspaces {
            let ws_rc = self.create_workspace(ws);
            if first {
                self.active_workspace = ws_rc;
                first = false;
            }
        }
    }

    pub fn set_test_active_workspace(&mut self, workspace: u16) {
        self.active_workspace = self.workspaces.get(&workspace).unwrap().clone();
    }
}
