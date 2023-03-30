use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkspaceLayout {
    VerticalStriped,   //  |
    HorizontalStriped, // ---
    Tiled,
}

impl WorkspaceLayout {
    pub fn to_string(&self) -> String {
        match self {
            WorkspaceLayout::VerticalStriped => "vertical_striped".into(),
            WorkspaceLayout::HorizontalStriped => "horizontal_striped".into(),
            WorkspaceLayout::Tiled => "tiled".into(),
        }
    }
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
