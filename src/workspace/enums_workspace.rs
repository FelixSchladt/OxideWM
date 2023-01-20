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

pub enum GoToWorkspace {
    Next,
    Previous,
}

impl TryFrom<&str> for GoToWorkspace {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "next" => Ok(GoToWorkspace::Next),
            "previous" => Ok(GoToWorkspace::Previous),
            _ => Err(format!("{} is not a valid option for traversing workspaces", value)),
        }
    }
}

impl GoToWorkspace {
    pub fn calculate_new_workspace(&self, active_workspace:usize, max_workspace:usize) -> usize {
        match self {
            GoToWorkspace::Next => (active_workspace + 1) % (max_workspace + 1),
            GoToWorkspace::Previous => {
                if active_workspace <= 1 {
                    max_workspace
                }else{
                    active_workspace - 1
                }
            }
        }
    }
}
