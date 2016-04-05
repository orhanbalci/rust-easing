use super::ease::easing;
struct Bounce;

impl easing for Bounce {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
        c - Bounce::ease_out(d-t, 0.0, c, d)
    }

    fn ease_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        0.0
    }


    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        if t < (d / 2.0) {
            Bounce::ease_in(t * 2.0, 0.0, c, d) * 0.5 + b
        } else {
            Bounce::ease_out(t * 2.0 - d, 0.0, c, d) * 0.5 + c * 0.5 + b
        }

    }
}
