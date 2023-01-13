use log::debug;
use uuid::Uuid;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::*;
use serde::Serialize;

use crate::workspace::Workspace;
use std::{cell::RefCell, rc::Rc, collections::HashMap};


#[derive(Debug, Clone, Serialize)]
pub struct ScreenInfo {
    #[serde(skip_serializing)]
    _connection: Rc<RefCell<RustConnection>>,
    #[serde(skip_serializing)]
    _screen_ref: Rc<RefCell<Screen>>,
    workspaces: HashMap<u16, Workspace>,
    pub active_workspace: u16,
    pub width: u32,
    pub height: u32,
}

impl ScreenInfo {
    pub fn new(connection: Rc<RefCell<RustConnection>>, screen_ref: Rc<RefCell<Screen>>, height: u32, width: u32) -> ScreenInfo {
        let active_workspace = 0;
        let workspaces = HashMap::new();
        let mut screen_info = ScreenInfo {
            _connection: connection,
            _screen_ref: screen_ref,
            workspaces,
            active_workspace,
            width,
            height,
        };
        screen_info.create_workspace(0);
        screen_info
    }
    
    pub fn create_new_workspace(&mut self){
        let mut index = u16::MAX;
        for i in 0..u16::MAX{
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

        let workspace_name = Uuid::new_v4().to_string();
        let new_workspace = Workspace::new(
            workspace_name,
            self._connection.clone(),
            self._screen_ref.clone(),
            0,
            0,
            self.height,
            self.width
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

    pub fn on_map_request(&mut self, event: &MapRequestEvent) {
        println!("WINMAN: MapRequestEvent: {:?}", event);
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

    /// If the workspace with the passed workspace_nr does not exist, it will be created
    pub fn set_workspace_create_if_not_exists(&mut self, workspace_nr: u16) -> &mut Workspace{
        debug!("Changing workspace from {} to {}", self.active_workspace, workspace_nr);

        let active_workspace = self.get_workspace(self.active_workspace);
        active_workspace.unmap_windows();

        if self.workspaces.contains_key(&workspace_nr){
            self.create_workspace(workspace_nr)
        }

        self.active_workspace = workspace_nr;
        let new_workspace = self.workspaces.get_mut(&self.active_workspace).unwrap();
        new_workspace.remap_windows();
        new_workspace
        
    }

    pub fn get_workspace_count(&self) -> usize{
        return self.workspaces.len();
    }
}
