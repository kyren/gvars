use crate::{FromStrError, GVar, GetError, Metadata, SetError};
use std::{
    any::TypeId,
    collections::HashMap,
    sync::atomic::{AtomicPtr, Ordering},
};

pub fn metadata() -> &'static [Metadata] {
    &FIELDS.1
}

pub fn get(name: &str) -> Result<String, GetError> {
    Ok(FIELDS
        .0
        .get(name)
        .ok_or(GetError::NoSuchField)?
        .get_string())
}

pub fn set(name: &str, val: &str) -> Result<(), SetError> {
    FIELDS
        .0
        .get(name)
        .ok_or(SetError::NoSuchField)?
        .set(val)
        .map_err(|e| SetError::FromStr(e))
}

#[doc(hidden)]
pub struct Field {
    pub unique_name: &'static str,
    pub aliases: &'static [&'static str],
    pub type_id: TypeId,
    pub setter: fn(&str) -> Result<*const (), FromStrError>,
    pub getter: unsafe fn(*const ()) -> String,
    pub ptr: AtomicPtr<()>,
}

impl Field {
    pub fn new<T: GVar>(
        unique_name: &'static str,
        aliases: &'static [&'static str],
        init: &'static T,
    ) -> Self {
        fn set<T: GVar>(s: &str) -> Result<*const (), FromStrError> {
            let t = T::from_str(s)?;
            Ok(Box::leak(Box::new(t)) as *const T as *const ())
        }

        unsafe fn get<T: GVar>(ptr: *const ()) -> String {
            (*(ptr as *const T)).to_string()
        }

        Self {
            unique_name,
            aliases,
            type_id: TypeId::of::<T>(),
            setter: set::<T>,
            getter: get::<T>,
            ptr: AtomicPtr::new(init as *const T as *mut ()),
        }
    }

    #[inline]
    pub fn get<T>(&self) -> &'static T {
        assert_eq!(self.type_id, TypeId::of::<T>());
        unsafe { &*(self.ptr.load(Ordering::Relaxed) as *const T) }
    }

    pub fn set(&self, s: &str) -> Result<(), FromStrError> {
        self.ptr
            .store((self.setter)(s)? as *mut (), Ordering::Relaxed);
        Ok(())
    }

    pub fn get_string(&self) -> String {
        unsafe { (self.getter)(self.ptr.load(Ordering::Relaxed)) }
    }
}

#[doc(hidden)]
pub use linkme;

#[doc(hidden)]
#[linkme::distributed_slice]
pub static FIELD_INITS: [fn() -> Field] = [..];

lazy_static::lazy_static! {
    #[doc(hidden)]
    pub static ref FIELDS: (HashMap<&'static str, &'static Field>, Vec<Metadata>) = {
        let mut map = HashMap::new();
        let mut met = Vec::new();
        for &f in FIELD_INITS.iter() {
            let field = (f)();
            let field: &'static Field = Box::leak(Box::new(field));
            assert!(map.insert(field.unique_name, field).is_none(), "unique field name is not unique");
            met.push(Metadata {
                unique_name: field.unique_name,
                aliases: field.aliases,
                type_id: field.type_id,
            });
            for &alias in field.aliases {
                map.insert(alias, field);
            }
        }
        (map, met)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! make {
    ($vis:vis, $name:ident, $ty:ty, $init:expr, $($alias:expr),*) => {
        $vis struct $name;

        $crate::assert_gvar!($ty);

        const _: () = {
            use std::any::Any;
            use $crate::{Field, linkme, FIELDS, FIELD_INITS};

            const FULL_NAME: &'static str = concat!(module_path!(), "::", stringify!($name));
            const SHORT_NAME: &'static str = stringify!($name);

            #[linkme::distributed_slice(FIELD_INITS)]
            fn field_init() -> Field {
                const INITIAL: $ty = $init;
                Field::new(FULL_NAME, &[SHORT_NAME, $($alias,)*], &INITIAL)
            }

            impl std::ops::Deref for $name {
                type Target = $ty;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    FIELDS.0.get(FULL_NAME).unwrap().get::<$ty>()
                }
            }
        };
    };
}
