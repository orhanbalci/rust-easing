use super::ease::Easing;
use functions::util::*;

/// This struct captures Quint easing functions
#[derive(Debug)]
pub struct Quint;

impl<F: Float> Easing<F> for Quint {
    fn ease_in(t: F, b: F, c: F, d: F) -> F {
        let t = t / d;
        c * (t * t * t * t * t) + b
    }

    fn ease_out(t: F, b: F, c: F, d: F) -> F {
        let t = t / d - f(1.0);
        c * ((t * t * t * t * t) + f(1.0)) + b
    }

    fn ease_in_out(t: F, b: F, c: F, d: F) -> F {
        let t = t / (d / f(2.0));
        if t < f(1.0) {
            (c / f(2.0)) * (t * t * t * t * t) + b
        } else {
            let t = t - f(2.0);
            c / f(2.0) * ((t * t * t * t * t) + f(2.0)) + b
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ease_in() {
        assert_relative_eq!(Quint::ease_in(1.0_f32, 2.0, 3.0, 4.0), 2.002930);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(Quint::ease_out(1.0_f32, 2.0, 3.0, 4.0), 4.288086);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(Quint::ease_in_out(1.0_f32, 2.0, 3.0, 4.0), 2.046875);
        assert_relative_eq!(Quint::ease_in_out(51.0_f32, 1.0, 100.0, 100.0), 55.803956);
    }

    const PRECISE_RESULT: f64 = 2.0094868329805053;

    #[test]
    fn f32_precision() {
        let ease32 = Quint::ease_in(10_f32.sqrt(), 2.0, 3.0, 10.0);
        assert_relative_ne!(ease32 as f64, PRECISE_RESULT); // f32 maths is actually happening
        assert_relative_eq!(ease32, PRECISE_RESULT as f32);
    }

    #[test]
    fn f64_precision() {
        let ease64 = Quint::ease_in(10_f64.sqrt(), 2.0, 3.0, 10.0);
        assert_relative_eq!(ease64, PRECISE_RESULT);
    }
}
