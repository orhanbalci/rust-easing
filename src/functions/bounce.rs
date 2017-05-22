use super::ease::Easing;
use functions::util::*;

/// This struct captures Bounce easing functions
#[derive(Debug)]
pub struct Bounce;

impl<F: Float> Easing<F> for Bounce {
    fn ease_in(t: F, b: F, c: F, d: F) -> F {
        c - Bounce::ease_out(d - t, f(0.0), c, d) + b
    }

    fn ease_out(t: F, b: F, c: F, d: F) -> F {
        cast_constants!(F; _1=1, _1_5=1.5, _2=2, _2_25=2.25, _2_5=2.5,
            _2_625=2.625, _7_5625=7.5625);

        let t = t / d;
        if t < _1 / f(2.75) {
            c * (_7_5625 * t * t) + b
        } else if t < _2 / f(2.75) {
            let t = t - _1_5 / f(2.75);
            c * (_7_5625 * t * t + f(0.75)) + b
        } else if t < _2_5 / f(2.75) {
            let t = t - _2_25 / f(2.75);
            c * (_7_5625 * t * t + f(0.9375)) + b
        } else {
            let t = t - _2_625 / f(2.75);
            c * (_7_5625 * t * t + f(0.984375)) + b
        }
    }

    fn ease_in_out(t: F, b: F, c: F, d: F) -> F {
        if t < (d / f(2.0)) {
            Bounce::ease_in(t * f(2.0), f(0.0), c, d) * f(0.5) + b
        } else {
            Bounce::ease_out(t * f(2.0) - d, f(0.0), c, d) * f(0.5) + c * f(0.5) + b
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ease_out() {
        assert_relative_eq!(Bounce::ease_out(1.0_f32, 2.0, 3.0, 4.0), 3.4179688);
        assert_relative_eq!(Bounce::ease_out(1.0_f32, 2.0, 3.0, 2.0), 4.296875);
        assert_relative_eq!(Bounce::ease_out(100.0_f32, 1.0, 100.0, 100.0), 101.000000);
    }

    #[test]
    fn ease_in() {
        assert_relative_eq!(Bounce::ease_in(1.0_f32, 2.0, 3.0, 4.0), 2.082031);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(Bounce::ease_in_out(1.0_f32, 2.0, 3.0, 4.0), 2.3515625);
        assert_relative_eq!(Bounce::ease_in_out(51.0_f32, 1.0, 100.0, 100.0), 51.151250);
    }

    const PRECISE_RESULT: f64 = 2.3159476740972824;

    #[test]
    fn f32_precision() {
        let ease32 = Bounce::ease_in(10_f32.sqrt(), 2.0, 3.0, 10.0);
        assert_relative_ne!(ease32 as f64, PRECISE_RESULT); // f32 maths is actually happening
        assert_relative_eq!(ease32, PRECISE_RESULT as f32);
    }

    #[test]
    fn f64_precision() {
        let ease64 = Bounce::ease_in(10_f64.sqrt(), 2.0, 3.0, 10.0);
        assert_relative_eq!(ease64, PRECISE_RESULT);
    }
}
