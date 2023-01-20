use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub enum EnumWorkspaceLayout {
    //Tiled, //blocked by https://github.com/DHBW-FN/OxideWM/issues/70
    VerticalStriped,   //  |
    HorizontalStriped, // ---
    //Different layout modes and better names wanted C:
}

impl TryFrom<&str> for EnumWorkspaceLayout {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "vertical" => Ok(EnumWorkspaceLayout::VerticalStriped),
            "horizontal" => Ok(EnumWorkspaceLayout::HorizontalStriped),
            _ => Err(format!("{} is not a valid layout", value)),
        }
    }
}
