pub use gvars_macro::gvar;
use std::{
    any::{Any, TypeId},
    error::Error,
    fmt,
    str::FromStr,
};

/// All constants that are annotated with `#[gvars::gvar]` must implement this trait.  Any type that
/// implements `ToString`, `FromStr`, `std::any::Any`, `Send`, and `Sync will automatically
/// implement this trait.
pub trait GVar: Any + Send + Sync + Sized {
    type FromStrErr: Error + Send + Sync;

    fn to_string(&self) -> String;
    fn from_str(s: &str) -> Result<Self, Self::FromStrErr>;
}

impl<T> GVar for T
where
    T: FromStr + ToString + Any + Send + Sync,
    <T as FromStr>::Err: Error + Send + Sync,
{
    type FromStrErr = <T as FromStr>::Err;

    fn to_string(&self) -> String {
        ToString::to_string(self)
    }

    fn from_str(s: &str) -> Result<Self, Self::FromStrErr> {
        FromStr::from_str(s)
    }
}

pub type FromStrError = Box<dyn Error + Send + Sync>;

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
    FromStr(FromStrError),
}

impl fmt::Display for SetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetError::Disabled => write!(f, "gvars are not enabled"),
            SetError::NoSuchField => write!(f, "no such field or field alias"),
            SetError::FromStr(err) => write!(f, "`FromStr` error: {}", err),
        }
    }
}

impl Error for SetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SetError::Disabled => None,
            SetError::NoSuchField => None,
            SetError::FromStr(err) => Some(&**err),
        }
    }
}

#[derive(Debug)]
pub enum GetError {
    Disabled,
    NoSuchField,
}

impl fmt::Display for GetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetError::Disabled => write!(f, "gvars are not enabled"),
            GetError::NoSuchField => write!(f, "no such field or field alias"),
        }
    }
}

impl Error for GetError {}

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
