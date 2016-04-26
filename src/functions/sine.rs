use super::ease::Easing;
use std::f32::consts::PI;

pub struct Sine;

impl Easing for Sine {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
        -c * (t / d * (PI / 2.0)).cos() + c + b
    }

    fn ease_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        c * (t / d * (PI / 2.0)).sin() + b
    }

    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        -c / 2.0 * ((PI * t / d).cos() - 1.0) + b
    }
}

mod test {
    #[allow(unused_imports)]
    use functions::ease::Easing;
    #[test]
    fn ease_in() {
        assert_relative_eq!(super::Sine::ease_in(1.0, 2.0, 3.0, 4.0), 2.2283616);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(super::Sine::ease_out(1.0, 2.0, 3.0, 4.0), 3.148050);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(super::Sine::ease_in_out(1.0, 2.0, 3.0, 4.0), 2.439340);
    }
}
