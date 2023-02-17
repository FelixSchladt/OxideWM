use oxide_common::ipc::{
    commands::WmCommands,
    state::{deserialize_optional_string, KeybindingDto, WmCommandArgumentDto},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WmCommandArgument {
    pub command: WmCommands,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub args: Option<String>,
}

impl WmCommandArgument {
    pub fn to_dto(&self) -> WmCommandArgumentDto {
        WmCommandArgumentDto {
            command: self.command.clone(),
            args: self.args.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WmCommand {
    pub keys: Vec<String>,
    pub commands: Vec<WmCommandArgument>,
}

impl WmCommand {
    pub fn to_dto(&self) -> KeybindingDto {
        let commands = self.commands.iter().map(|arg| arg.to_dto()).collect();
        KeybindingDto {
            keys: self.keys.clone(),
            commands: commands,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IterCmd {
    pub iter: Vec<String>,
    pub command: WmCommand,
}
