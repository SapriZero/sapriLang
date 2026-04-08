/// ================================================================
/// SAPRI FP MODULE - Functional & Monadic Core
/// ================================================================

#[macro_export]
macro_rules! compose {
    ($f:expr) => { $f };
    ($f:expr, $($rest:expr),+) => {
        move |x| $f($crate::compose!($($rest),+)(x))
    };
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    #[inline(always)]
    pub fn is_left(&self) -> bool { matches!(self, Either::Left(_)) }
    
    #[inline(always)]
    pub fn is_right(&self) -> bool { matches!(self, Either::Right(_)) }

    #[inline(always)]
    pub fn map_right<F, S>(self, f: F) -> Either<L, S>
    where F: FnOnce(R) -> S {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(f(r)),
        }
    }

    #[inline(always)]
    pub fn unwrap_right(self) -> R {
        match self {
            Either::Right(r) => r,
            Either::Left(_) => panic!("Called unwrap_right on a Left value"),
        }
    }
}

#[inline(always)]
pub fn bind<T, R, F>(value: Option<T>, f: F) -> Option<R>
where F: FnOnce(T) -> Option<R> {
    value.and_then(f)
}

#[inline(always)]
pub fn fmap<T, R, F>(value: Option<T>, f: F) -> Option<R>
where F: FnOnce(T) -> R {
    value.map(f)
}

#[inline(always)]
pub fn tap<T, F>(value: T, f: F) -> T
where F: FnOnce(&T) {
    f(&value);
    value
}

#[inline(always)]
pub fn mask(condition: bool) -> usize {
    condition as usize
}

#[inline(always)]
pub fn identity<T>(value: T) -> T {
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let res = eval(true, || "vero", || "falso");
        assert_eq!(res, "vero");
    }

    #[test]
    fn test_mask() {
        assert_eq!(mask(true), 1);
        assert_eq!(mask(false), 0);
    }
}


// ==========================================
// MACRO eval! (versione compatta)
// ==========================================

/// Macro per valutazione condizionale compatta
/// 
/// # Esempio
/// ```
/// let risultato = eval!(x > 10, 100, 0);
/// let risultato_lazy = eval!(x > 10, { calcola_complesso() }, { 0 });
/// ```
#[macro_export]
macro_rules! eval {
    // Versione con valori diretti
    ($cond:expr, $then:expr, $else:expr) => {
        if $cond { $then } else { $else }
    };
    
    // Versione con blocchi (per espressioni complesse)
    ($cond:expr, { $then:expr }, { $else:expr }) => {
        if $cond { $then } else { $else }
    };
}

// Core function (all others alias this)
pub fn or_else<T, F>(opt: &Option<T>, default_fn: F) -> T
where
    F: FnOnce() -> T,
    T: Clone,
{
    opt.clone().unwrap_or_else(default_fn)
}

// Alias declarations
pub use or_else as unwrap_or_else;
pub use or_else as opt_ref_or_else;
pub use or_else as opt_or_else;
pub use or_else as opt_as_ref_or_else;


// ==========================================
// GESTIONE STATO (get/set con default)
// ==========================================

/// Ottiene un valore da un riferimento Option, se None restituisce default
/// 
/// # Esempio
/// ```
/// let valore = get_or_default(&optional, 42);
/// ```
#[inline(always)]
pub fn get_or_default<T: Clone>(opt: &Option<T>, default: T) -> T {
    opt.as_ref().cloned().unwrap_or(default)
}

/// Ottiene un valore da un riferimento Option, se None calcola default con closure
/// 
/// # Esempio
/// ```
/// let valore = get_or_default_with(&optional, || calcola_default());
/// ```
#[inline(always)]
pub fn get_or_default_with<T, F>(opt: &Option<T>, default_fn: F) -> T
where
    F: FnOnce() -> T,
{
    opt.as_ref().cloned().unwrap_or_else(default_fn)
}

/// Imposta un valore, se il valore è None usa default
/// 
/// # Esempio
/// ```
/// set_or_default(&mut stato, Some(42), 0);
/// set_or_default(&mut stato, None, 0);  // imposta a 0
/// ```
#[inline(always)]
pub fn set_or_default<T: Clone>(target: &mut Option<T>, value: Option<T>, default: T) {
    *target = Some(value.unwrap_or(default));
}

/// Imposta un valore, se il valore è None calcola default con closure
/// 
/// # Esempio
/// ```
/// set_or_default_with(&mut stato, None, || calcola_default());
/// ```
#[inline(always)]
pub fn set_or_default_with<T, F>(target: &mut Option<T>, value: Option<T>, default_fn: F)
where
    F: FnOnce() -> T,
{
    *target = Some(value.unwrap_or_else(default_fn));
}

/// Versione currying di get_or_default
/// 
/// # Esempio
/// ```
/// let get_with_default = get_curried(42);
/// let valore = get_with_default(&optional);
/// ```
#[inline(always)]
pub fn get_curried<T: Clone>(default: T) -> impl Fn(&Option<T>) -> T {
    move |opt| opt.as_ref().cloned().unwrap_or(default.clone())
}

/// Versione currying di set_or_default
/// 
/// # Esempio
/// ```
/// let set_with_default = set_curried(0);
/// set_with_default(&mut stato, Some(42));
/// set_with_default(&mut stato, None);  // imposta a 0
/// ```
#[inline(always)]
pub fn set_curried<T: Clone>(default: T) -> impl Fn(&mut Option<T>, Option<T>) {
    move |target, value| {
        *target = Some(value.unwrap_or(default.clone()));
    }
}

// ==========================================
// TEST
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(eval(true, 10, 20), 10);
        assert_eq!(eval(false, 10, 20), 20);
    }

    #[test]
    fn test_eval_lazy() {
        let mut counter = 0;
        let risultato = eval_lazy(true, || { counter += 1; 42 }, || { counter += 2; 0 });
        assert_eq!(risultato, 42);
        assert_eq!(counter, 1);  // solo then_fn eseguita
    }

    #[test]
    fn test_eval_macro() {
        assert_eq!(eval!(true, 10, 20), 10);
        assert_eq!(eval!(false, 10, 20), 20);
        
        let risultato = eval!(true, { 10 + 5 }, { 20 - 5 });
        assert_eq!(risultato, 15);
    }

    #[test]
    fn test_get_or_default() {
        let some: Option<i32> = Some(42);
        let none: Option<i32> = None;
        
        assert_eq!(get_or_default(&some, 0), 42);
        assert_eq!(get_or_default(&none, 0), 0);
    }

    #[test]
    fn test_get_or_default_with() {
        let none: Option<i32> = None;
        let result = get_or_default_with(&none, || 99);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_set_or_default() {
        let mut opt: Option<i32> = None;
        
        set_or_default(&mut opt, Some(42), 0);
        assert_eq!(opt, Some(42));
        
        set_or_default(&mut opt, None, 99);
        assert_eq!(opt, Some(99));
    }

    #[test]
    fn test_curried() {
        let some: Option<i32> = Some(42);
        let none: Option<i32> = None;
        
        let get_with_default = get_curried(0);
        assert_eq!(get_with_default(&some), 42);
        assert_eq!(get_with_default(&none), 0);
        
        let mut opt: Option<i32> = None;
        let set_with_default = set_curried(99);
        
        set_with_default(&mut opt, Some(42));
        assert_eq!(opt, Some(42));
        
        set_with_default(&mut opt, None);
        assert_eq!(opt, Some(99));
    }
}

// ==========================================
// VALUTAZIONE FUNZIONALE (eval)
// ==========================================

/// Valutazione condizionale funzionale (If-Then-Else)
/// Restituisce il valore della branch scelta
/// 
/// # Esempio
/// ```
/// let risultato = eval(x > 10, || 100, || 0);
/// ```
#[inline(always)]
pub fn eval<T>(condition: bool, then_val: T, else_val: T) -> T {
    if condition { then_val } else { else_val }
}

/// Versione con closure (lazy evaluation)
/// Le closure vengono eseguite solo quando necessario
/// 
/// # Esempio
/// ```
/// let risultato = eval_lazy(x > 10, || calcola_100(), || 0);
/// ```
#[inline(always)]
pub fn eval_lazy<T, F1, F2>(condition: bool, then_fn: F1, else_fn: F2) -> T
where
    F1: FnOnce() -> T,
    F2: FnOnce() -> T,
{
    if condition { then_fn() } else { else_fn() }
}

// ==========================================
// MACRO eval! (versione compatta)
// ==========================================

/// Macro per valutazione condizionale compatta
/// 
/// # Esempio
/// ```
/// let risultato = eval!(x > 10, 100, 0);
/// let risultato_lazy = eval!(x > 10, { calcola_complesso() }, { 0 });
/// ```
#[macro_export]
macro_rules! eval {
    // Versione con valori diretti
    ($cond:expr, $then:expr, $else:expr) => {
        if $cond { $then } else { $else }
    };
    
    // Versione con blocchi (per espressioni complesse)
    ($cond:expr, { $then:expr }, { $else:expr }) => {
        if $cond { $then } else { $else }
    };
}

// ==========================================
// GESTIONE STATO (get/set con default)
// ==========================================

/// Ottiene un valore da un riferimento Option, se None restituisce default
/// 
/// # Esempio
/// ```
/// let valore = get_or_default(&optional, 42);
/// ```
#[inline(always)]
pub fn get_or_default<T: Clone>(opt: &Option<T>, default: T) -> T {
    opt.as_ref().cloned().unwrap_or(default)
}

/// Ottiene un valore da un riferimento Option, se None calcola default con closure
/// 
/// # Esempio
/// ```
/// let valore = get_or_default_with(&optional, || calcola_default());
/// ```
#[inline(always)]
pub fn get_or_default_with<T, F>(opt: &Option<T>, default_fn: F) -> T
where
    F: FnOnce() -> T,
{
    opt.into_iter().cloned().unwrap_or_else(default_fn)

}

/// Imposta un valore, se il valore è None usa default
/// 
/// # Esempio
/// ```
/// set_or_default(&mut stato, Some(42), 0);
/// set_or_default(&mut stato, None, 0);  // imposta a 0
/// ```
#[inline(always)]
pub fn set_or_default<T: Clone>(target: &mut Option<T>, value: Option<T>, default: T) {
    *target = Some(value.unwrap_or(default));
}

/// Imposta un valore, se il valore è None calcola default con closure
/// 
/// # Esempio
/// ```
/// set_or_default_with(&mut stato, None, || calcola_default());
/// ```
#[inline(always)]
pub fn set_or_default_with<T, F>(target: &mut Option<T>, value: Option<T>, default_fn: F)
where
    F: FnOnce() -> T,
{
    *target = Some(value.unwrap_or_else(default_fn));
}

/// Versione currying di get_or_default
/// 
/// # Esempio
/// ```
/// let get_with_default = get_curried(42);
/// let valore = get_with_default(&optional);
/// ```
#[inline(always)]
pub fn get_curried<T: Clone>(default: T) -> impl Fn(&Option<T>) -> T {
    move |opt| opt.as_ref().cloned().unwrap_or(default.clone())
}

/// Versione currying di set_or_default
/// 
/// # Esempio
/// ```
/// let set_with_default = set_curried(0);
/// set_with_default(&mut stato, Some(42));
/// set_with_default(&mut stato, None);  // imposta a 0
/// ```
#[inline(always)]
pub fn set_curried<T: Clone>(default: T) -> impl Fn(&mut Option<T>, Option<T>) {
    move |target, value| {
        *target = Some(value.unwrap_or(default.clone()));
    }
}

// ==========================================
// TEST
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(eval(true, 10, 20), 10);
        assert_eq!(eval(false, 10, 20), 20);
    }

    #[test]
    fn test_eval_lazy() {
        let mut counter = 0;
        let risultato = eval_lazy(true, || { counter += 1; 42 }, || { counter += 2; 0 });
        assert_eq!(risultato, 42);
        assert_eq!(counter, 1);  // solo then_fn eseguita
    }

    #[test]
    fn test_eval_macro() {
        assert_eq!(eval!(true, 10, 20), 10);
        assert_eq!(eval!(false, 10, 20), 20);
        
        let risultato = eval!(true, { 10 + 5 }, { 20 - 5 });
        assert_eq!(risultato, 15);
    }

    #[test]
    fn test_get_or_default() {
        let some: Option<i32> = Some(42);
        let none: Option<i32> = None;
        
        assert_eq!(get_or_default(&some, 0), 42);
        assert_eq!(get_or_default(&none, 0), 0);
    }

    #[test]
    fn test_get_or_default_with() {
        let none: Option<i32> = None;
        let result = get_or_default_with(&none, || 99);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_set_or_default() {
        let mut opt: Option<i32> = None;
        
        set_or_default(&mut opt, Some(42), 0);
        assert_eq!(opt, Some(42));
        
        set_or_default(&mut opt, None, 99);
        assert_eq!(opt, Some(99));
    }

    #[test]
    fn test_curried() {
        let some: Option<i32> = Some(42);
        let none: Option<i32> = None;
        
        let get_with_default = get_curried(0);
        assert_eq!(get_with_default(&some), 42);
        assert_eq!(get_with_default(&none), 0);
        
        let mut opt: Option<i32> = None;
        let set_with_default = set_curried(99);
        
        set_with_default(&mut opt, Some(42));
        assert_eq!(opt, Some(42));
        
        set_with_default(&mut opt, None);
        assert_eq!(opt, Some(99));
    }
}
