use super::ease::Easing;
pub struct Expo;

impl Easing for Expo {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
        if t == 0.0 {
            b
        } else {
            c * 2_f32.powf(10.0 * (t / d - 1.0)) + b
        }
    }

    fn ease_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        if t == d {
            b + c
        } else {
            c * (-2_f32.powf(-10.0 * t / d) + 1.0) + b
        }
    }

    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        if t == 0.0 {
            return b;
        }
        if t == d {
            return b + c;
        }
        let mut inner_t = t / (d / 2.0);
        if inner_t < 1.0 {
            return c / 2.0 * 2_f32.powf(10.0 * (inner_t - 1.0)) + b;
        }
        inner_t -= 1.0;
        return c / 2.0 * (-(2_f32.powf(-10.0 * inner_t)) + 2.0) + b;
    }
}

mod test {
    use functions::ease::Easing;
    #[test]
    fn ease_in() {
        assert_relative_eq!(super::Expo::ease_in(1.0, 2.0, 3.0, 4.0), 2.016573);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(super::Expo::ease_out(1.0, 2.0, 3.0, 4.0), 4.469670);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(super::Expo::ease_in_out(1.0, 2.0, 3.0, 4.0), 2.046875);
    }

}
