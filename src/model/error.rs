use crate::model::store;
use crate::{crypt, model};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    EntityNotFound { entity: &'static str, id: i64 },

    // -- Modules
    Crypt(crypt::Error),
    Store(store::Error),

    // -- Externals
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

// region:    --- Froms
impl From<crypt::Error> for Error {
    fn from(val: crypt::Error) -> Self {
        Self::Crypt(val)
    }
}
impl From<store::Error> for Error {
    fn from(val: store::Error) -> Self {
        Self::Store(val)
    }
}
impl From<sqlx::Error> for Error {
    fn from(val: sqlx::Error) -> Self {
        Self::Sqlx(val)
    }
}
// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
