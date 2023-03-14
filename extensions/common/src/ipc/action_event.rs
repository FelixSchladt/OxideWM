use super::commands::WmCommands;
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

#[derive(Type, DeserializeDict, SerializeDict, Debug)]
#[zvariant(signature = "dict")]
pub struct WmActionEvent {
    pub command: WmCommands,
    pub args: Option<String>,
}

impl WmActionEvent {
    pub fn new(command: &str, args: Option<String>) -> Result<Self, String> {
        let parsed_command = match WmCommands::try_from(command) {
            Ok(parsed_command) => parsed_command,
            Err(msg) => {
                return Err(msg);
            }
        };

        Ok(WmActionEvent {
            command: parsed_command,
            args,
        })
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
