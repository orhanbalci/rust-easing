use super::ease::Easing;
use functions::util::*;

/// This struct captures Back easing functions
#[derive(Debug)]
pub struct Back;

impl<F: Float> Easing<F> for Back {
    fn ease_in(t: F, b: F, c: F, d: F) -> F {
        let s: F = f(1.70158);
        let t = t / d;
        c * t * t * ((s + f(1.0)) * t - s) + b
    }

    fn ease_out(t: F, b: F, c: F, d: F) -> F {
        let s: F = f(1.70158);
        let t = (t / d) - f(1.0);
        c * (t * t * ((s + f(1.0)) * t + s) + f(1.0)) + b
    }

    fn ease_in_out(t: F, b: F, c: F, d: F) -> F {
        let s: F = f(1.70158);
        let t = t / (d / f(2.0));
        if t < f(1.0) {
            let s = s * f(1.525);
            c / f(2.0) * (t * t * ((s + f(1.0)) * t - s)) + b
        }
        else {
            let t = t - f(2.0);
            let s = s * f(1.525);
            c / f(2.0) * (t * t * ((s + f(1.0)) * t + s) + f(2.0)) + b
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ease_in() {
        assert_relative_eq!(Back::ease_in(1.0_f32, 2.0, 3.0, 4.0), 1.8075902);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(Back::ease_out(1.0_f32, 2.0, 3.0, 4.0), 4.452229);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(Back::ease_in_out(1.0_f32, 2.0, 3.0, 4.0), 1.7009544);
        assert_relative_eq!(Back::ease_in_out(51.0_f32, 1.0, 100.0, 100.0), 56.432546);
    }

    const PRECISE_RESULT: f64 = 1.7458203824347307;

    #[test]
    fn f32_precision() {
        let ease32 = Back::ease_in(10_f32.sqrt(), 2.0, 3.0, 10.0);
        assert_relative_ne!(ease32 as f64, PRECISE_RESULT); // f32 maths is actually happening
        assert_relative_eq!(ease32, PRECISE_RESULT as f32);
    }

    #[test]
    fn f64_precision() {
        let ease64 = Back::ease_in(10_f64.sqrt(), 2.0, 3.0, 10.0);
        assert_relative_eq!(ease64, PRECISE_RESULT);
    }
}
