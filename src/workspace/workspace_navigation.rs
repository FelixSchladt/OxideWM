use log::error;

use super::parse_error::ParseError;

pub enum WorkspaceNavigation {
    Next,
    Previous,
    Number(u16),
}

impl WorkspaceNavigation {
    pub fn parse_workspace_navigation(args_option: Option<String>)->Result<WorkspaceNavigation, ParseError>{
        if let Some(args) = args_option {
            let go_to_result = WorkspaceNavigation::try_from(args.as_str());
            match go_to_result {
                Ok(arg) => Ok(arg),
                Err(_) => return Err(ParseError::new( 
                    format!("Argumet '{}' could not be parsed", args)
                ))
            }
        }else{
            Err(ParseError::new(format!("No argument was passed")))
        }
    }
}

impl TryFrom<&str> for WorkspaceNavigation {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 1 {
            if let Some(character) = value.chars().next() {
                if let Some(digit) = character.to_digit(10) {
                    if let Ok(digit_u16) = u16::try_from(digit){
                        return Ok(WorkspaceNavigation::Number(digit_u16));
                    }else{
                        error!("Number to big for workspace :'{}'",digit);
                        return Err(format!("Number to big for workspace :'{}'",digit));
                    }
                }
            }
        }

        match value.to_lowercase().as_str() {
            "next" => Ok(WorkspaceNavigation::Next),
            "previous" => Ok(WorkspaceNavigation::Previous),
            _ => Err(format!("{} is not a valid option for traversing workspaces", value)),
        }
    }
}
