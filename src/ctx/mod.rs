// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

// endregion: --- Modules
#[derive(Clone, Debug)]
pub struct Ctx {
    pub user_id: u64,
}

// Constructor.
impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: 0 }
    }
    pub fn new(user_id: u64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }
}

// Property Accessors.
impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
