use super::ease::Easing;

/// This struct captures Quint easing functions
pub struct Quint;

impl Easing for Quint {
    fn ease_in(mut t: f32, b: f32, c: f32, d: f32) -> f32 {
        t = t / d;
        c * (t * t * t * t * t) + b
    }

    fn ease_out(mut t: f32, b: f32, c: f32, d: f32) -> f32 {
        t = t / d - 1.0;
        c * ((t * t * t * t * t) + 1.0) + b
    }

    fn ease_in_out(mut t: f32, b: f32, c: f32, d: f32) -> f32 {
        t = t / (d / 2.0);
        if t < 1.0 {
            (c / 2.0) * (t * t * t * t * t) + b
        } else {
            t -= 2.0;
            c / 2.0 * ((t * t * t * t * t) + 2.0) + b
        }
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use functions::ease::Easing;
    #[test]
    fn ease_in() {
        assert_relative_eq!(super::Quint::ease_in(1.0, 2.0, 3.0, 4.0), 2.002930);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(super::Quint::ease_out(1.0, 2.0, 3.0, 4.0), 4.288086);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(super::Quint::ease_in_out(1.0, 2.0, 3.0, 4.0), 2.046875);
        assert_relative_eq!(super::Quint::ease_in_out(51.0, 1.0, 100.0, 100.0),
                            55.803956);
    }
}
