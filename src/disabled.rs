use crate::{GetError, Metadata, SetError};

pub fn metadata() -> &'static [Metadata] {
    &[]
}

pub fn get(_name: &str) -> Result<String, GetError> {
    Err(GetError::Disabled)
}

pub fn set(_name: &str, _val: &str) -> Result<(), SetError> {
    Err(SetError::Disabled)
}

#[doc(hidden)]
#[macro_export]
macro_rules! make {
    ($vis:vis, $name:ident, $ty:ty, $init:expr, $($alias:expr),*) => {
        $vis struct $name;

        impl std::ops::Deref for $name
            where $ty: $crate::GVar,
        {
            type Target = $ty;

            fn deref(&self) -> &'static Self::Target {
                const VAL: $ty = $init;
                &VAL
            }
        }
    }
}
