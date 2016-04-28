use super::ease::Easing;
/// This struct captures quadratic easing functions
pub struct Quad;

impl Easing for Quad {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let inner_t = t / d;
        c * inner_t.powi(2) + b
    }

    fn ease_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let inner_t = t / d;
        -c * inner_t * (inner_t - 2.0) + b
    }

    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let inner_t = t / (d / 2.0);
        if inner_t < 1.0 {
            return (c / 2.0 * (inner_t.powi(2))) + b;
        }
        let temp = inner_t - 1.0;
        return -c / 2.0 * (((inner_t - 2.0) * (temp)) - 1.0) + b;
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use functions::ease::Easing;
    #[test]
    fn ease_in() {
        assert_relative_eq!(super::Quad::ease_in(1.0, 2.0, 3.0, 4.0), 2.187500);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(super::Quad::ease_out(1.0, 2.0, 3.0, 4.0), 3.312500);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(super::Quad::ease_in_out(1.0, 2.0, 3.0, 4.0), 2.37500);
        assert_relative_eq!(super::Quad::ease_in_out(51.0, 1.0, 100.0, 100.0), 51.979999);
    }

}
