use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum WorkspaceLayout {
    //Tiled, //blocked by https://github.com/DHBW-FN/OxideWM/issues/70
    VerticalStriped, //  |
    HorizontalStriped, // ---
                     //Different layout modes and better names wanted C:
}

impl TryFrom<&str> for WorkspaceLayout {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "vertical" => Ok(WorkspaceLayout::VerticalStriped),
            "horizontal" => Ok(WorkspaceLayout::HorizontalStriped),
            _ => Err(format!("{} is not a valid layout", value)),
        }
    }
}
