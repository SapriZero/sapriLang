/// Macro per definire un contesto con binding
#[macro_export]
macro_rules! define {
    // Con ereditarietà: using $parent, poi definizioni separate da ;
    (using $parent:expr, $($name:ident = $value:expr);* $(;)?) => {{
        let mut ctx = $crate::Context::with_parent(&std::sync::Arc::new($parent.clone()));
        $(
            ctx.set(stringify!($name), ::sapri_base::Atom::resolved($crate::AtomValue::from($value)));
        )*
        ctx
    }};
    
    // Senza ereditarietà
    ($($name:ident = $value:expr);* $(;)?) => {{
        let mut ctx = $crate::Context::new();
        $(
            ctx.set(stringify!($name), ::sapri_base::Atom::resolved($crate::AtomValue::from($value)));
        )*
        ctx
    }};
}

/// Versione con supporto espressioni (usa scan internamente)
#[macro_export]
macro_rules! define_expr {
    (using $parent:expr, $($name:ident = $expr:expr);* $(;)?) => {{
        let mut ctx = $crate::Context::with_parent(&std::sync::Arc::new($parent.clone()));
        $(
            let value = $crate::scan!($expr, &ctx)?;
            ctx.set(stringify!($name), value);
        )*
        Ok(ctx) as Result<_, String>
    }};
    
    ($($name:ident = $expr:expr);* $(;)?) => {{
        let mut ctx = $crate::Context::new();
        $(
            let value = $crate::scan!($expr, &ctx)?;
            ctx.set(stringify!($name), value);
        )*
        Ok(ctx) as Result<_, String>
    }};
}
