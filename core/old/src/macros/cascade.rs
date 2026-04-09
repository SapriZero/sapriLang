//! Macro per creare oggetti a cascata (ereditarietà)
//!
//! cascade!(base, user, session)

#[macro_export]
macro_rules! cascade {
    ($first:expr $(, $rest:expr)*) => {
        {
            let mut __result = $first.clone();
            $(
                __result = __result.merge($rest.clone());
            )*
            __result
        }
    };
}
