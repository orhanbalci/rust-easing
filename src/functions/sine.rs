use super::ease::Easing;
use functions::util::*;
use num_traits::FloatConst;

/// This struct captures Sine easing functions
#[derive(Debug)]
pub struct Sine;

impl<F: Float + FloatConst> Easing<F> for Sine {
    fn ease_in(t: F, b: F, c: F, d: F) -> F {
        -c * (t / d * (F::PI() / f(2.0))).cos() + c + b
    }

    fn ease_out(t: F, b: F, c: F, d: F) -> F {
        c * (t / d * (F::PI() / f(2.0))).sin() + b
    }

    fn ease_in_out(t: F, b: F, c: F, d: F) -> F {
        -c / f(2.0) * ((F::PI() * t / d).cos() - f(1.0)) + b
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ease_in() {
        assert_relative_eq!(Sine::ease_in(1.0_f32, 2.0, 3.0, 4.0), 2.2283616);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(Sine::ease_out(1.0_f32, 2.0, 3.0, 4.0), 3.148050);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(Sine::ease_in_out(1.0_f32, 2.0, 3.0, 4.0), 2.439340);
    }

    const PRECISE_RESULT: f64 = 2.362562395222677;

    #[test]
    fn f32_precision() {
        let ease32 = Sine::ease_in(10_f32.sqrt(), 2.0, 3.0, 10.0);
        assert_relative_ne!(ease32 as f64, PRECISE_RESULT); // f32 maths is actually happening
        assert_relative_eq!(ease32, PRECISE_RESULT as f32);
    }

    #[test]
    fn f64_precision() {
        let ease64 = Sine::ease_in(10_f64.sqrt(), 2.0, 3.0, 10.0);
        assert_relative_eq!(ease64, PRECISE_RESULT);
    }
}
