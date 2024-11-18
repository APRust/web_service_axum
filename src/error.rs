use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // -- Config
    ConfigMissingEnv(&'static str),

    // -- Modules
    Model(model::Error),
}

// region:    --- Froms
impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Self::Model(val)
    }
}
// endregion: --- Froms

// region:   ---Error boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for Error {}

// endregion:   ---Error boilerplate
