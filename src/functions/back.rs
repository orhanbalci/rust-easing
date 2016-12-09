use super::ease::Easing;

/// This struct captures Back easing functions
pub struct Back;

impl Easing for Back {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let s = 1.70158_f32;
        let t = t / d;
        c * t * t * ((s + 1.0) * t - s) + b
    }

    fn ease_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let s = 1.70158_f32;
        let t = (t / d) - 1.0;
        c * (t * t * ((s + 1.0) * t + s) + 1.0) + b
    }

    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let s = 1.70158_f32;
        let t = t / (d / 2.0);
        if t < 1.0 {
            let s = s * 1.525f32;
            c / 2.0 * (t * t * ((s + 1.0) * t - s)) + b
        }
        else {
            let t = t - 2.0;
            let s = s * 1.525f32;
            c / 2.0 * (t * t * ((s + 1.0) * t + s) + 2.0) + b
        }
    }
}

#[cfg(test)]
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
