use super::ease::Easing;

/// This struct captures Bounce easing functions
pub struct Bounce;

impl Easing for Bounce {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
        c - Bounce::ease_out(d - t, 0.0, c, d) + b
    }

    fn ease_out(mut t: f32, b: f32, c: f32, d: f32) -> f32 {
        t = t / d;
        if t < 1.0 / 2.75 {
            c * (7.5625 * t * t) + b
        } else if t < 2.0 / 2.75 {
            t -= 1.5 / 2.75;
            c * (7.5625 * t * t + 0.75) + b
        } else if t < 2.5 / 2.75 {
            t -= 2.25 / 2.75;
            c * (7.5625 * t * t + 0.9375) + b
        } else {
            t -= 2.625 / 2.75;
            c * (7.5625 * t * t + 0.984375) + b
        }
    }

    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        if t < (d / 2.0) {
            Bounce::ease_in(t * 2.0, 0.0, c, d) * 0.5 + b
        } else {
            Bounce::ease_out(t * 2.0 - d, 0.0, c, d) * 0.5 + c * 0.5 + b
        }

    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use functions::ease::Easing;
    #[test]
    fn ease_out() {
        assert_relative_eq!(super::Bounce::ease_out(1.0, 2.0, 3.0, 4.0), 3.4179688);
        assert_relative_eq!(super::Bounce::ease_out(1.0, 2.0, 3.0, 2.0), 4.296875);
        assert_relative_eq!(super::Bounce::ease_out(100.0, 1.0, 100.0, 100.0),
                            101.000000);
    }

    #[test]
    fn ease_in() {
        assert_relative_eq!(super::Bounce::ease_in(1.0, 2.0, 3.0, 4.0), 2.082031);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(super::Bounce::ease_in_out(1.0, 2.0, 3.0, 4.0), 2.3515625);
        assert_relative_eq!(super::Bounce::ease_in_out(51.0, 1.0, 100.0, 100.0),
                            51.151250);
    }
}
