use super::ease::Easing;
use functions::util::*;

/// This struct captures Quart easing functions
#[derive(Debug)]
pub struct Quart;

impl<F: Float> Easing<F> for Quart {
    fn ease_in(t: F, b: F, c: F, d: F) -> F {
        let t = t / d;
        c * (t * t * t * t) + b
    }

    fn ease_out(t: F, b: F, c: F, d: F) -> F {
        let t = t / d - f(1.0);
        -c * ((t * t * t * t) - f(1.0)) + b
    }

    fn ease_in_out(t: F, b: F, c: F, d: F) -> F {
        let t = t / (d / f(2.0));
        if t < f(1.0) {
            c / f(2.0) * (t * t * t * t) + b
        }
        else {
            let t = t - f(2.0);
            -c / f(2.0) * ((t * t * t * t) - f(2.0)) + b
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ease_in() {
        assert_relative_eq!(Quart::ease_in(1.0_f32, 2.0, 3.0, 4.0), 2.011719);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(Quart::ease_out(1.0_f32, 2.0, 3.0, 4.0), 4.050781);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(Quart::ease_in_out(1.0_f32, 2.0, 3.0, 4.0), 2.093750);
        assert_relative_eq!(Quart::ease_in_out(51.0_f32, 1.0, 100.0, 100.0), 54.881588);
    }

    const PRECISE_RESULT: f64 = 2.0204904036609523;

    #[test]
    fn f32_precision() {
        let ease32 = Quart::ease_in(10_f32.sqrt(), 2.0, 3.0, 11.0);
        assert_relative_ne!(ease32 as f64, PRECISE_RESULT); // f32 maths is actually happening
        assert_relative_eq!(ease32, PRECISE_RESULT as f32);
    }

    #[test]
    fn f64_precision() {
        let ease64 = Quart::ease_in(10_f64.sqrt(), 2.0, 3.0, 11.0);
        assert_relative_eq!(ease64, PRECISE_RESULT);
    }
}
