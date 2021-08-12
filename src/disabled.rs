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

        $crate::assert_gvar!($ty);

        impl std::ops::Deref for $name {
            type Target = $ty;

            #[inline]
            fn deref(&self) -> &'static Self::Target {
                const VAL: $ty = $init;
                &VAL
            }
        }
    }
}
