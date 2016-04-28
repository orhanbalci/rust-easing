use super::ease::Easing;

/// This struct captures Cubic easing functions
pub struct Cubic;

impl Easing for Cubic {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let inner_t = t / d;
        c * inner_t.powi(3) + b
    }

    fn ease_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let inner_t = t / d - 1.0;
        c * (inner_t.powi(3) + 1.0) + b
    }

    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let mut inner_t = t / (d / 2.0);
        if inner_t < 1.0 {
            return c / 2.0 * inner_t.powi(3) + b;
        }
        inner_t = inner_t - 2.0;
        return c / 2.0 * (inner_t.powi(3) + 2.0) + b;
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use functions::ease::Easing;
    #[test]
    fn ease_in() {
        assert_relative_eq!(super::Cubic::ease_in(1.0, 2.0, 3.0, 4.0), 2.046875);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(super::Cubic::ease_out(1.0, 2.0, 3.0, 4.0), 3.734375);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(super::Cubic::ease_in_out(1.0, 2.0, 3.0, 4.0), 2.187500);
        assert_relative_eq!(super::Cubic::ease_in_out(51.0, 1.0, 100.0, 100.0),
                            53.940397);
    }
}
