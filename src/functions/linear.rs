use super::ease::Easing;

pub struct Linear;

impl Easing for Linear {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
        c * t / d + b
    }

    fn ease_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        c * t / d + b
    }

    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        c * t / d + b
    }
}
