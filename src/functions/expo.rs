use super::ease::Easing;
use functions::util::*;

/// This struct captures Expo easing functions
#[derive(Debug)]
pub struct Expo;

impl<F: Float> Easing<F> for Expo {
    fn ease_in(t: F, b: F, c: F, d: F) -> F {
        cast_constants!(F; _2=2, _10=10);

        if t == f(0.0) {
            b
        } else {
            c * _2.powf(_10 * (t / d - f(1.0))) + b
        }
    }

    fn ease_out(t: F, b: F, c: F, d: F) -> F {
        cast_constants!(F; _2=2, _10=10);

        if t == d {
            b + c
        } else {
            c * (-_2.powf(-_10 * t / d) + f(1.0)) + b
        }
    }

    fn ease_in_out(t: F, b: F, c: F, d: F) -> F {
        cast_constants!(F; _2=2, _10=10);

        if t == f(0.0) {
            return b;
        }
        if t == d {
            return b + c;
        }
        let t = t / (d / _2);
        if t < f(1.0) {
            c / _2 * _2.powf(_10 * (t - f(1.0))) + b
        }
        else {
            let t = t - f(1.0);
            c / _2 * (-(_2.powf(-_10 * t)) + _2) + b
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ease_in() {
        assert_relative_eq!(Expo::ease_in(1.0_f32, 2.0, 3.0, 4.0), 2.016573);
        assert_relative_eq!(Expo::ease_in(0.0_f32, 1.0, 100.0, 100.0), 1.000000);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(Expo::ease_out(1.0_f32, 2.0, 3.0, 4.0), 4.469670);
        assert_relative_eq!(Expo::ease_out(100.0_f32, 1.0, 100.0, 100.0), 101.0000);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(Expo::ease_in_out(1.0_f32, 2.0, 3.0, 4.0), 2.046875);
        assert_relative_eq!(Expo::ease_in_out(0.0_f32, 1.0, 100.0, 100.0), 1.0000);
        assert_relative_eq!(Expo::ease_in_out(100.0_f32, 1.0, 100.0, 100.0), 101.000);
        assert_relative_eq!(Expo::ease_in_out(51.0_f32, 1.0, 100.0, 100.0), 57.472466);
    }

    const PRECISE_RESULT: f64 = 2.0262277918539184;

    #[test]
    fn f32_precision() {
        let ease32 = Expo::ease_in(10_f32.sqrt(), 2.0, 3.0, 10.0);
        assert_relative_ne!(ease32 as f64, PRECISE_RESULT); // f32 maths is actually happening
        assert_relative_eq!(ease32, PRECISE_RESULT as f32);
    }

    #[test]
    fn f64_precision() {
        let ease64 = Expo::ease_in(10_f64.sqrt(), 2.0, 3.0, 10.0);
        assert_relative_eq!(ease64, PRECISE_RESULT);
    }
}
