use super::ease::Easing;

pub struct Circ;

impl Easing for Circ {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let inner_t = t / d;
        -c * ((1.0 - inner_t * inner_t).sqrt() - 1.0) + b
    }

    fn ease_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let inner_t = t / d - 1.0;
        c * (1.0 - inner_t * inner_t).sqrt() + b
    }

    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let mut inner_t = t / (d / 2.0);
        if inner_t < 1.0 {
            return -c / 2.0 * ((1.0 - inner_t * inner_t).sqrt() - 1.0) + b;
        }

        inner_t -= 2.0;
        return c / 2.0 * ((1.0 - inner_t * inner_t).sqrt() + 1.0) + b;
    }
}

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
