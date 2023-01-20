use log::error;
use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub enum Layout {
    //Tiled, //blocked by https://github.com/DHBW-FN/OxideWM/issues/70
    VerticalStriped,   //  |
    HorizontalStriped, // ---
    //Different layout modes and better names wanted C:
}

impl TryFrom<&str> for Layout {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "vertical" => Ok(Layout::VerticalStriped),
            "horizontal" => Ok(Layout::HorizontalStriped),
            _ => Err(format!("{} is not a valid layout", value)),
        }
    }
}

pub enum EnumWorkspaceNavigation {
    Next,
    Previous,
    Number(u16),
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
