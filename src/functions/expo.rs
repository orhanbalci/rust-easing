use super::ease::Easing;

/// This struct captures Expo easing functions
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

    fn ease_in_out(mut t: f32, b: f32, c: f32, d: f32) -> f32 {
        if t == 0.0 {
            return b;
        }
        if t == d {
            return b + c;
        }
        t = t / (d / 2.0);
        if t < 1.0 {
            c / 2.0 * 2_f32.powf(10.0 * (t - 1.0)) + b
        }
        else {
            t -= 1.0;
            c / 2.0 * (-(2_f32.powf(-10.0 * t)) + 2.0) + b
        }
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use functions::ease::Easing;
    #[test]
    fn ease_in() {
        assert_relative_eq!(super::Expo::ease_in(1.0, 2.0, 3.0, 4.0), 2.016573);
        assert_relative_eq!(super::Expo::ease_in(0.0, 1.0, 100.0, 100.0), 1.000000);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(super::Expo::ease_out(1.0, 2.0, 3.0, 4.0), 4.469670);
        assert_relative_eq!(super::Expo::ease_out(100.0, 1.0, 100.0, 100.0), 101.0000);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(super::Expo::ease_in_out(1.0, 2.0, 3.0, 4.0), 2.046875);
        assert_relative_eq!(super::Expo::ease_in_out(0.0, 1.0, 100.0, 100.0), 1.0000);
        assert_relative_eq!(super::Expo::ease_in_out(100.0, 1.0, 100.0, 100.0), 101.000);
        assert_relative_eq!(super::Expo::ease_in_out(51.0, 1.0, 100.0, 100.0), 57.472466);
    }

}
