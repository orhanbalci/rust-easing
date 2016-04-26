use super::ease::Easing;
/// This struct captures Back easing functions
pub struct Back;

impl Easing for Back {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let s = 1.70158_f32;
        let inner_t = t / d;
        let post_fix = inner_t;
        c * (post_fix) * inner_t * ((s + 1.0) * inner_t - s) + b
    }

    fn ease_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let s = 1.70158_f32;
        let inner_t = (t / d) - 1.0;
        c * (inner_t * inner_t * ((s + 1.0) * inner_t + s) + 1.0) + b
    }

    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let mut s = 1.70158_f32;
        let mut inner_t = t / (d / 2.0);
        if inner_t < 1.0 {
            s *= 1.525f32;
            return c / 2.0 * (inner_t * inner_t * ((s + 1.0) * inner_t - s)) + b;
        }
        inner_t -= 2.0;
        let post_fix: f32 = inner_t;
        let inner_s = s * 1.525f32;
        return c / 2.0 * (post_fix * inner_t * ((inner_s + 1.0) * inner_t + inner_s) + 2.0) + b;
    }
}

mod test {
    #[allow(unused_imports)]
    use functions::ease::Easing;
    #[test]
    fn ease_in() {
        assert_relative_eq!(super::Back::ease_in(1.0, 2.0, 3.0, 4.0), 1.8075902);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(super::Back::ease_out(1.0, 2.0, 3.0, 4.0), 4.452229);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(super::Back::ease_in_out(1.0, 2.0, 3.0, 4.0), 1.7009544);
        assert_relative_eq!(super::Back::ease_in_out(51.0, 1.0, 100.0, 100.0), 56.432546);
    }
}
