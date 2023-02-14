use itertools::*;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use super::commands::WmCommands;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenInfoDto {
    pub workspaces: HashMap<u16, WorkspaceDto>,
    pub active_workspace: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowStateDto {
    pub frame: u32,
    pub window: u32,
    pub title: String,
    pub visible: bool,
    pub urgent: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub gap_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceDto {
    pub name: u16,
    pub focused_window: Option<u32>,
    pub fullscreen: Option<u32>,
    pub urgent: bool,
    pub windows: HashMap<u32, WindowStateDto>,
    pub order: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WmCommandArgumentDto {
    pub command: WmCommands,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub args: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingDto {
    pub keys: Vec<String>,
    pub commands: Vec<WmCommandArgumentDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigDto {
    pub cmds: Vec<KeybindingDto>,
    pub exec: Vec<String>,
    pub exec_always: Vec<String>,
    pub border_width: u32,
    pub border_color: String,
    pub border_focus_color: String,
    pub gap: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxideStateDto {
    pub screeninfo: HashMap<u32, ScreenInfoDto>,
    pub config: ConfigDto,
    pub focused_screen: u32,
}

impl OxideStateDto {
    pub fn get_workspaces(&self, screen: u32) -> HashMap<u16, WorkspaceDto> {
        println!("screen: {}", screen);
        println!("screeninfo: {:?}", self.screeninfo);
        self.screeninfo.get(&screen).unwrap().workspaces.clone()
    }

    pub fn get_workspace_list(&self, screen: u32) -> Vec<u16> {
        let mut vec = self
            .get_workspaces(screen)
            .iter()
            .map(|(ws, _)| *ws)
            .collect_vec();
        vec.sort();
        vec
    }

    pub fn get_active_workspace(&self, screen: u32) -> u16 {
        self.screeninfo.get(&screen).unwrap().active_workspace
    }
}

pub fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let args = Option::<String>::deserialize(deserializer)?;
    let args = args.unwrap_or("".to_string());
    if args.is_empty() || args == "None" {
        Ok(None)
    } else {
        Ok(Some(args))
    }
}
