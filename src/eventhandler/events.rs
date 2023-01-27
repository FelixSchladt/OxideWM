use crate::eventhandler::commands::WmCommands;
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

pub enum EventType {
    X11rbEvent(x11rb::protocol::Event),
    OxideEvent(crate::eventhandler::events::IpcEvent),
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

#[derive(DeserializeDict, SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct IpcEvent {
    pub status: bool,
    pub event: Option<WmActionEvent>,
}

impl From<WmActionEvent> for IpcEvent {
    fn from(command: WmActionEvent) -> Self {
        IpcEvent {
            status: false,
            event: Some(command),
        }
    }
}
