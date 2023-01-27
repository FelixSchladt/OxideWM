#[derive(Debug)]
pub struct ParseError {
    details: String,
}

impl ParseError {
    pub fn new(details: String) -> ParseError {
        ParseError { details }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        &self.details
    }
}
