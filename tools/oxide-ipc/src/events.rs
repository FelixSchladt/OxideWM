use serde::{Deserialize, Serialize};
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub enum WmCommands {
    Move, //args: left, up, right, down
    Focus,
    Resize,
    Quit,    // Quit the window manager
    Kill,    // Kill the focused window
    Restart, // Restart the window manager
    Layout,  //args: horizontal, vertical
    MoveToWorkspace,
    GoToWorkspace,
    MoveToWorkspaceAndFollow,
    QuitWorkspace,
    Exec,
    Fullscreen,
}

impl TryFrom<&str> for WmCommands {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "move" => Ok(WmCommands::Move),
            "focus" => Ok(WmCommands::Focus),
            "resize" => Ok(WmCommands::Resize),
            "quit" => Ok(WmCommands::Quit),
            "kill" => Ok(WmCommands::Kill),
            "restart" => Ok(WmCommands::Restart),
            "layout" => Ok(WmCommands::Layout),
            "movetoworkspace" => Ok(WmCommands::MoveToWorkspace),
            "gotoworkspace" => Ok(WmCommands::GoToWorkspace),
            "movetoworkspaceandfollow" => Ok(WmCommands::MoveToWorkspaceAndFollow),
            "exec" => Ok(WmCommands::Exec),
            "fullscreen" => Ok(WmCommands::Fullscreen),
            _ => Err(format!("{} is not a valid command", value)),
        }
    }
}

#[derive(Type, DeserializeDict, SerializeDict, Debug)]
#[zvariant(signature = "dict")]
pub struct WmActionEvent {
    pub command: WmCommands,
    pub args: Option<String>,
}

impl WmActionEvent {
    pub fn new(command: &str, args: Option<String>) -> Self {
        WmActionEvent {
            command: WmCommands::try_from(command).unwrap(),
            args,
        }
    }
}

impl std::fmt::Display for WmActionEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Err(error) = write!(f, "Command: {:?}", self.command) {
            return Err(error);
        }

        if let Some(args) = self.args.clone() {
            if let Err(error) = write!(f, ", Args: {}", args) {
                return Err(error);
            }
        }
        Ok(())
    }
}
