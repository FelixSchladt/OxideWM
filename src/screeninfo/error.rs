#[derive(Debug)]
pub struct MoveError {
    reason: String
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MoveError: {}", self.reason)
    }
}

impl MoveError {
    pub fn new(reason:String) -> MoveError{
        MoveError { reason }
    }
}

#[derive(Debug)]
pub struct QuitError {
    reason: String
}

impl std::fmt::Display for QuitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QuitError: {}", self.reason)
    }
}

impl QuitError {
    pub fn new(reason:String) -> QuitError{
        QuitError { reason }
    }
}