use oxide_common::ipc::action_event::WmActionEvent;
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

pub enum EventType {
    X11rbEvent(x11rb::protocol::Event),
    OxideEvent(crate::eventhandler::events::IpcEvent),
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
