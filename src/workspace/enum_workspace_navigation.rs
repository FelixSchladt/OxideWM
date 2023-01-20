use log::error;

use super::parse_error::ParseError;

pub enum EnumWorkspaceNavigation {
    Next,
    Previous,
    Number(u16),
}

impl EnumWorkspaceNavigation {
    pub fn parse_enum_workspace_navigation(args_option: Option<String>)->Result<EnumWorkspaceNavigation, ParseError>{
        if let Some(args) = args_option {
            let go_to_result = EnumWorkspaceNavigation::try_from(args.as_str());
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

impl TryFrom<&str> for EnumWorkspaceNavigation {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 1 {
            if let Some(character) = value.chars().next() {
                if let Some(digit) = character.to_digit(10) {
                    if let Ok(digit_u16) = u16::try_from(digit){
                        return Ok(EnumWorkspaceNavigation::Number(digit_u16));
                    }else{
                        error!("Number to big for workspace :'{}'",digit);
                        return Err(format!("Number to big for workspace :'{}'",digit));
                    }
                }
            }
        }

        match value.to_lowercase().as_str() {
            "next" => Ok(EnumWorkspaceNavigation::Next),
            "previous" => Ok(EnumWorkspaceNavigation::Previous),
            _ => Err(format!("{} is not a valid option for traversing workspaces", value)),
        }
    }
}