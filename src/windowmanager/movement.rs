pub enum Movement {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&str> for Movement {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "left" => Ok(Movement::Left),
            "right" => Ok(Movement::Right),
            "up" => Ok(Movement::Up),
            "down" => Ok(Movement::Down),
            _ => Err(format!("{} is not a valid movement", value)),
        }
    }
}
