//! Macro per generare struct che leggono da Obj
//!
//! struct_with_keys! {
//!     Actions {
//!         save: Action,
//!         open: Action
//!     }
//! }

#[macro_export]
macro_rules! struct_with_keys {
    (
        $(#[$meta:meta])*
        $vis:vis $name:ident {
            $($field:ident : $type:ty),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $name {
            $( pub $field: $type ),*
        }

        impl $name {
            pub fn from_obj(obj: &$crate::Obj) -> Result<Self, String> {
                Ok(Self {
                    $(
                        $field: obj
                            .get(&$crate::path!($name.$field))
                            .ok_or_else(|| format!("Missing key: {}", stringify!($name.$field)))?
                            .clone(),
                    )*
                })
            }

            pub fn try_from_obj(obj: &$crate::Obj) -> Option<Self> {
                Some(Self {
                    $(
                        $field: obj.get(&$crate::path!($name.$field))?.clone(),
                    )*
                })
            }
        }
    };
}
