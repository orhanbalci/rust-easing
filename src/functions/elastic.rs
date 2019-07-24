use super::ease::Easing;
use functions::util::*;
use num_traits::float::FloatConst;

/// This struct captures Elastic easing functions
#[derive(Debug)]
pub struct Elastic;

impl<F: Float + FloatConst> Easing<F> for Elastic {
    fn ease_in(t: F, b: F, c: F, d: F) -> F {
        cast_constants!(F; _2=2, _10=10);

        if t == f(0.0) {
            return b;
        }

        let t = t / d;
        if t == f(1.0) {
            return b + c;
        }

        let p = d * f(0.3);
        let a = c;
        let s = p / f(4.0);
        let t = t - f(1.0);
        let post_fix = a * _2.powf(_10 * t);
        let temp = (t * d - s) * (_2 * F::PI()) / p;
        -(post_fix * temp.sin()) + b
    }

    fn ease_out(t: F, b: F, c: F, d: F) -> F {
        cast_constants!(F; _2=2, _10=10);

        if t == f(0.0) {
            return b;
        }

        let t = t / d;
        if t == f(1.0) {
            return b + c;
        }

        let p = d * f(0.3);
        let a = c;
        let s = p / f(4.0);
        let temp = (t * d - s) * (_2 * F::PI()) / p;
        a * _2.powf(-_10 * t) * temp.sin() + c + b
    }

    fn ease_in_out(t: F, b: F, c: F, d: F) -> F {
        cast_constants!(F; _2=2, _10=10, _0_5=0.5);

        if t == f(0.0) {
            return b;
        }

        let t = t / (d / _2);
        if t == _2 {
            return b + c;
        }

        let p = d * f(0.3) * f(1.5);
        let a = c;
        let s = p / f(4.0);

        if t < f(1.0) {
            let t = t - f(1.0);
            let post_fix = a * _2.powf(_10 * t);
            let temp = (t * d - s) * (_2 * F::PI()) / p;
            return -_0_5 * (post_fix * temp.sin()) + b;
        }

        let t = t - f(1.0);
        let post_fix = a * _2.powf(-_10 * t);
        let temp = (t * d - s) * (_2 * F::PI()) / p;
        post_fix * temp.sin() * f(0.5) + c + b
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ease_in() {
        assert_relative_eq!(Elastic::ease_in(1.0_f32, 2.0, 3.0, 4.0), 1.983427);
        assert_relative_eq!(Elastic::ease_in(0.0_f32, 1.0, 100.0, 100.0), 1.0000);
        assert_relative_eq!(Elastic::ease_in(100.0_f32, 1.0, 100.0, 100.0), 101.000);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(Elastic::ease_out(1.0_f32, 2.0, 3.0, 4.0), 4.734835);
        assert_relative_eq!(Elastic::ease_out(0.0_f32, 1.0, 100.0, 100.0), 1.0000);
        assert_relative_eq!(Elastic::ease_out(100.0_f32, 1.0, 100.0, 100.0), 101.000);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(Elastic::ease_in_out(1.0_f32, 2.0, 3.0, 4.0), 2.035908);
        assert_relative_eq!(Elastic::ease_in_out(0.0_f32, 1.0, 100.0, 100.0), 1.0000);
        assert_relative_eq!(Elastic::ease_in_out(100.0_f32, 1.0, 100.0, 100.0), 101.0000);
        assert_relative_eq!(Elastic::ease_in_out(51.0_f32, 1.0, 100.0, 100.0), 59.158646);
    }

    const PRECISE_RESULT: f64 = 1.9952083561735905;

    #[test]
    fn f32_precision() {
        let ease32 = Elastic::ease_in(10_f32.sqrt(), 2.0, 3.0, 10.0);
        assert_relative_ne!(ease32 as f64, PRECISE_RESULT); // f32 maths is actually happening
        assert_relative_eq!(ease32, PRECISE_RESULT as f32);
    }

    #[test]
    fn f64_precision() {
        let ease64 = Elastic::ease_in(10_f64.sqrt(), 2.0, 3.0, 10.0);
        assert_relative_eq!(ease64, PRECISE_RESULT);
    }
}
