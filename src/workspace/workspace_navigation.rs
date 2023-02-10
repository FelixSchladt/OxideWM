use super::parse_error::ParseError;

#[derive(Debug, Clone)]
pub enum WorkspaceNavigation {
    NextFree,
    Next,
    Previous,
    Number(u16),
}

impl WorkspaceNavigation {
    pub fn parse_workspace_navigation(
        args_option: Option<String>,
    ) -> Result<WorkspaceNavigation, ParseError> {
        if let Some(args) = args_option {
            let go_to_result = WorkspaceNavigation::try_from(args.as_str());
            match go_to_result {
                Ok(arg) => Ok(arg),
                Err(_) => {
                    return Err(ParseError::new(format!(
                        "Argumet '{}' could not be parsed",
                        args
                    )))
                }
            }
        } else {
            Err(ParseError::new(format!("No argument was passed")))
        }
    }

    pub fn is_create_if_not_exists(&self) -> bool {
        match self {
            WorkspaceNavigation::NextFree => true,
            WorkspaceNavigation::Number(_) => true,
            _ => false,
        }
    }
}

impl TryFrom<&str> for WorkspaceNavigation {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 1 {
            let character = value.chars().next().unwrap_or('_');
            if let Some(digit) = character.to_digit(10) {
                let digit_u16 = u16::try_from(digit).unwrap();
                return Ok(WorkspaceNavigation::Number(digit_u16));
            }
        }

        match value.to_lowercase().as_str() {
            "next" => Ok(WorkspaceNavigation::Next),
            "previous" => Ok(WorkspaceNavigation::Previous),
            "next_free" => Ok(WorkspaceNavigation::NextFree),
            _ => Err(format!(
                "{} is not a valid option for traversing workspaces",
                value
            )),
        }
    }
}
