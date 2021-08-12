pub use gvars_macro::gvar;
use serde::de::DeserializeOwned;
use std::{any::{Any, TypeId}, fmt, error::Error};

/// All constants that are annotated with `#[gvars::gvar]` must implement
/// `serde::de::DeserializeOwned`, `std::any::Any`, `Send`, and `Sync`.  Any such type will
/// automatically implement this alias trait.
// TODO: replace with trait_alias
pub trait GVar: DeserializeOwned + Any + Send + Sync {}
impl<T: DeserializeOwned + Any + Send + Sync> GVar for T {}

pub type DeserializeError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Copy, Clone)]
pub struct Metadata {
    pub unique_name: &'static str,
    pub aliases: &'static [&'static str],
    pub type_id: TypeId,
}

#[derive(Debug)]
pub enum SetError {
    Disabled,
    NoSuchField,
    Deserialization(DeserializeError),
}

impl fmt::Display for SetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetError::Disabled => write!(f, "gvars are not enabled"),
            SetError::NoSuchField => write!(f, "no such field or field alias"),
            SetError::Deserialization(err) => write!(f, "deserialization error: {}", err),
        }
    }
}

impl Error for SetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SetError::Disabled => None,
            SetError::NoSuchField => None,
            SetError::Deserialization(err) => Some(&**err),
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(any(all(feature = "enable-for-debug", debug_assertions), feature = "enable-always"))] {
        pub const ENABLED: bool = true;

        mod enabled;
        pub use self::enabled::*;
    } else {
        pub const ENABLED: bool = false;

        mod disabled;
        pub use self::disabled::*;
    }
}
