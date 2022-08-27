pub use num_traits::Float;

/// Casts a literal f64 to an appropriate `Float` value
#[inline]
pub fn f<F: Float>(x: f64) -> F {
    F::from(x).expect("cast failed, are you using non f32,f64 types?")
}

/// Macro for defining constants of generic Float type, reduces boiler plate when `f(1.23)` usage
/// isn't enough and the compiler needs explicit type, ie f::<F>(1.23). Such cases we can improve
/// equation readability by using constants like `_1_23`
///
/// `cast_constants!(F; _1_23=1.23, _10=10);` is equivalent to
/// ```no_run,ignore
/// let _1_23: F = f(1.23);
/// let _10: F = f(10 as f64);
/// ```
macro_rules! cast_constants {
    ($t:ty; $( $name:ident=$val:expr ),+) => {
        $(
            let $name: $t = f($val as f64);
        )+
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn float_casting() {
        type F1 = f64;
        type F2 = f32;

        let root2: f64 = 2.0.sqrt();
        let root2_f1: F1 = f(root2);
        assert_relative_eq!(root2_f1, root2);

        let root2_f2: F2 = f(root2);
        assert_relative_eq!(root2_f2, root2 as F2);
    }

    #[test]
    fn cast_constants_macro() {
        type F = f64;
        cast_constants!(F; _1_234=1.234, _234=234);

        assert_relative_eq!(_1_234, 1.234);
        assert_relative_eq!(_234, 234.0);
    }
}
