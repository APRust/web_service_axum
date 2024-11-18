use serde::Serialize;
use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug, Serialize)]
pub enum Error {
    FailToCreatePool(String),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
