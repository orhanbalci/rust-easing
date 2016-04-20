use super::ease::Easing;
use std::f32::consts::PI;
pub struct Elastic;

impl Easing for Elastic {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
        if t == 0.0 {
            return b;
        }

        let mut inner_t = t / d;
        if inner_t == 1.0 {
            return b + c;
        }

        let p = d * 0.3;
        let a = c;
        let s = p / 4.0;
        inner_t -= 1.0;
        let post_fix = a * 2_f32.powf(10.0 * inner_t);
        let temp = (inner_t * d - s) * (2.0 * PI) / p;
        return -(post_fix * temp.sin()) + b;
    }

    fn ease_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        if t == 0.0 {
            return b;
        }

        let inner_t = t / d;
        if inner_t == 1.0 {
            return b + c;
        }

        let p = d * 0.3;
        let a = c;
        let s = p / 4.0;
        let temp = (inner_t * d - s) * (2.0 * PI) / p;
        return a * 2_f32.powf(-10.0 * inner_t) * temp.sin() + c + b;
    }

    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        if t == 0.0 {
            return b;
        }

        let mut inner_t = t / (d / 2.0);
        if inner_t == 2.0 {
            return b + c;
        }

        let p = d * 0.3 * 1.5;
        let a = c;
        let s = p / 4.0;

        if inner_t < 1.0 {
            inner_t -= 1.0;
            let post_fix = a * 2_f32.powf(10.0 * inner_t);
            let temp = (inner_t * d - s) * (2.0 * PI) / p;
            return -0.5 * (post_fix * temp.sin()) + b;
        }

        inner_t -= 1.0;
        let post_fix = a * 2_f32.powf(-10.0 * inner_t);
        let temp = (inner_t * d - s) * (2.0 * PI) / p;
        return post_fix * temp.sin() * 0.5 + c + b;
    }
}

mod test {
    use functions::ease::Easing;

    #[test]
    fn ease_in() {
        assert_relative_eq!(super::Elastic::ease_in(1.0, 2.0, 3.0, 4.0), 1.983427);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(super::Elastic::ease_out(1.0, 2.0, 3.0, 4.0), 4.734835);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(super::Elastic::ease_in_out(1.0, 2.0, 3.0, 4.0), 2.035908);
    }
}
