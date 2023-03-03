use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkspaceLayout {
    //Tiled, //blocked by https://github.com/DHBW-FN/OxideWM/issues/70
    VerticalStriped, //  |
    HorizontalStriped, // ---
    Tiled,
}

impl TryFrom<&str> for WorkspaceLayout {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "vertical" => Ok(WorkspaceLayout::VerticalStriped),
            "horizontal" => Ok(WorkspaceLayout::HorizontalStriped),
            "tiled" => Ok(WorkspaceLayout::Tiled),
            _ => Err(format!("{} is not a valid layout", value)),
        }
    }
}
