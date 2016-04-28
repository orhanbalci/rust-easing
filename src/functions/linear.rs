use super::ease::Easing;

/// This struct captures Linear easing functions
pub struct Linear;

impl Easing for Linear {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
        c * t / d + b
    }

    fn ease_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        c * t / d + b
    }

    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        c * t / d + b
    }
}

mod test {
    #[allow(unused_imports)]
    use functions::ease::Easing;
    #[test]
    fn ease_in() {
        assert_relative_eq!(super::Linear::ease_in(1.0, 2.0, 3.0, 4.0), 2.7500);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(super::Linear::ease_out(1.0, 2.0, 3.0, 4.0), 2.7500);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(super::Linear::ease_in_out(1.0, 2.0, 3.0, 4.0), 2.7500);
    }
}
