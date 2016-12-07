use super::ease::Easing;
use std::f32::consts::PI;

/// This struct captures Elastic easing functions
pub struct Elastic;

impl Easing for Elastic {
    fn ease_in(mut t: f32, b: f32, c: f32, d: f32) -> f32 {
        if t == 0.0 {
            return b;
        }

        t = t / d;
        if t == 1.0 {
            return b + c;
        }

        let p = d * 0.3;
        let a = c;
        let s = p / 4.0;
        t -= 1.0;
        let post_fix = a * 2_f32.powf(10.0 * t);
        let temp = (t * d - s) * (2.0 * PI) / p;
        -(post_fix * temp.sin()) + b
    }

    fn ease_out(mut t: f32, b: f32, c: f32, d: f32) -> f32 {
        if t == 0.0 {
            return b;
        }

        t = t / d;
        if t == 1.0 {
            return b + c;
        }

        let p = d * 0.3;
        let a = c;
        let s = p / 4.0;
        let temp = (t * d - s) * (2.0 * PI) / p;
        a * 2_f32.powf(-10.0 * t) * temp.sin() + c + b
    }

    fn ease_in_out(mut t: f32, b: f32, c: f32, d: f32) -> f32 {
        if t == 0.0 {
            return b;
        }

        t = t / (d / 2.0);
        if t == 2.0 {
            return b + c;
        }

        let p = d * 0.3 * 1.5;
        let a = c;
        let s = p / 4.0;

        if t < 1.0 {
            t -= 1.0;
            let post_fix = a * 2_f32.powf(10.0 * t);
            let temp = (t * d - s) * (2.0 * PI) / p;
            return -0.5 * (post_fix * temp.sin()) + b;
        }

        t -= 1.0;
        let post_fix = a * 2_f32.powf(-10.0 * t);
        let temp = (t * d - s) * (2.0 * PI) / p;
        post_fix * temp.sin() * 0.5 + c + b
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use functions::ease::Easing;

    #[test]
    fn ease_in() {
        assert_relative_eq!(super::Elastic::ease_in(1.0, 2.0, 3.0, 4.0), 1.983427);
        assert_relative_eq!(super::Elastic::ease_in(0.0, 1.0, 100.0, 100.0), 1.0000);
        assert_relative_eq!(super::Elastic::ease_in(100.0, 1.0, 100.0, 100.0), 101.000);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(super::Elastic::ease_out(1.0, 2.0, 3.0, 4.0), 4.734835);
        assert_relative_eq!(super::Elastic::ease_out(0.0, 1.0, 100.0, 100.0), 1.0000);
        assert_relative_eq!(super::Elastic::ease_out(100.0, 1.0, 100.0, 100.0), 101.000);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(super::Elastic::ease_in_out(1.0, 2.0, 3.0, 4.0), 2.035908);
        assert_relative_eq!(super::Elastic::ease_in_out(0.0, 1.0, 100.0, 100.0), 1.0000);
        assert_relative_eq!(super::Elastic::ease_in_out(100.0, 1.0, 100.0, 100.0),
                            101.0000);
        assert_relative_eq!(super::Elastic::ease_in_out(51.0, 1.0, 100.0, 100.0),
                            59.158646);

    }
}
