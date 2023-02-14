use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

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
