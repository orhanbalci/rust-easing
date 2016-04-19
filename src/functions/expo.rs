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
        if t == d {
            return b;
        }
        if t == d {
            return b + c;
        }
        let mut inner_t = t / d / 2.0;
        if inner_t < 1.0 {
            return c / 2.0 * 2_f32.powf(10.0 * (t - 1.0)) + b;
        }
        inner_t -= 1.0;
        return c / 2.0 * (-(2_f32.powf(-10.0 * inner_t)) + 2.0) + b;
    }
}
