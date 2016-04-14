use super::ease::Easing;

struct Quad;

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
        let mut inner_t = t / d / 2.0;
        if inner_t < 1.0 {
            return (c / 2.0 * (inner_t.powi(2))) + b;
        }
        let temp = inner_t - 1.0;
        return -c / 2.0 * (((inner_t - 2.0) * (temp)) - 1.0) + b;
    }
}
