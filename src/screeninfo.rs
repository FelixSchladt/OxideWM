use log::warn;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::MapRequestEvent;
use crate::workspace::Workspace;
use std::{cell::RefCell, rc::Rc, collections::HashSet};


#[derive(Debug)]
pub struct ScreenInfo {
    pub connection: Rc<RefCell<RustConnection>>,
    pub id: u32,
    workspaces: Vec<Workspace>,
    pub active_workspace: usize,
    pub width: u32,
    pub height: u32,
}

impl ScreenInfo {
    pub fn new(connection: Rc<RefCell<RustConnection>>, id: u32, height: u32, width: u32) -> ScreenInfo {
        let active_workspace = 0;
        let workspaces = Vec::new();
        ScreenInfo {
            connection,
            id,
            workspaces,
            active_workspace,
            width,
            height,
        }
    }
    
    pub fn create_new_workspace(&mut self){
        let mut indices: HashSet<usize> = HashSet::new();
        for workspace in self.workspaces.iter() {
            indices.insert(workspace.index);
        }
        let mut index = u16::MAX;
        for i in 0..u16::MAX{
            if !indices.contains(&(i as usize)){
                index = i;
                break;
            }
        } 

        let new_workspace = Workspace::new(index as usize, self.connection.clone(), 0, 0, self.height, self.width);
        self.workspaces.push(new_workspace);
        self.set_workspace(index as usize);
    }

    pub fn get_workspace(&mut self, workspace_id: usize) -> Option<&mut Workspace>{
        for workspace in self.workspaces.iter_mut() {
            if workspace_id == workspace.index {
                return Some(workspace);
            }
        }
        None
    }

    pub fn on_map_request(&mut self, event: &MapRequestEvent) {
        println!("WINMAN: MapRequestEvent: {:?}", event);
        let workspace = &mut self.workspaces[self.active_workspace.clone()];
        workspace.new_window(event.window);
        workspace.remap_windows();
    }

    pub fn set_workspace(&mut self, workspace_id: usize){
        let workspace_option= self.get_workspace(workspace_id);

        if workspace_option.is_none() {
            warn!("Invalid active workspace {}", workspace_id);
            return;
        }

        debug!("Changing workspace from {} to {}", self.active_workspace, workspace_id);

        let active_workspace = self.get_workspace(self.active_workspace);
        if active_workspace.is_some(){
            active_workspace.unwrap().unmap_windows();
        }
        self.active_workspace = workspace_id;
        self.workspaces[self.active_workspace].remap_windows();
    }

    pub fn get_workspace_count(&self) -> usize{
        return self.workspaces.len();
    }
}
