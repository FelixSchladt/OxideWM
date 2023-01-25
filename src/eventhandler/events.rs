use crate::eventhandler::commands::WmCommands;
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

pub enum EnumEventType {
    X11rbEvent(x11rb::protocol::Event),
    OxideEvent(crate::eventhandler::events::IpcEvent)
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