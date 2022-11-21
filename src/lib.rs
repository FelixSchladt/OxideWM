
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("grab_key failed: {0}")]
    GrabKey(String),

    #[error("unknown: {0}")]
    Unknown(String),
}
