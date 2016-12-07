use super::ease::Easing;

/// This struct captures Circ easing functions
pub struct Circ;

impl Easing for Circ {
    fn ease_in(mut t: f32, b: f32, c: f32, d: f32) -> f32 {
        t = t / d;
        -c * ((1.0 - t * t).sqrt() - 1.0) + b
    }

    fn ease_out(mut t: f32, b: f32, c: f32, d: f32) -> f32 {
        t = t / d - 1.0;
        c * (1.0 - t * t).sqrt() + b
    }

    fn ease_in_out(mut t: f32, b: f32, c: f32, d: f32) -> f32 {
        t = t / (d / 2.0);
        if t < 1.0 {
            -c / 2.0 * ((1.0 - t * t).sqrt() - 1.0) + b
        }
        else {
            t -= 2.0;
            c / 2.0 * ((1.0 - t * t).sqrt() + 1.0) + b
        }
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use functions::ease::Easing;
    #[test]
    fn ease_in() {
        assert_relative_eq!(super::Circ::ease_in(1.0, 2.0, 3.0, 4.0), 2.0952625);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(super::Circ::ease_out(1.0, 2.0, 3.0, 4.0), 3.9843135);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(super::Circ::ease_in_out(1.0, 2.0, 3.0, 4.0), 2.200962);
        assert_relative_eq!(super::Circ::ease_in_out(51.0, 1.0, 100.0, 100.0), 60.949871);
    }
}
