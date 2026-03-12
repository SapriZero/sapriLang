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
pub fn eval<T, F, G>(condition: bool, op_true: F, op_false: G) -> T 
where 
    F: FnOnce() -> T, 
    G: FnOnce() -> T 
{
    if condition { op_true() } else { op_false() }
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
