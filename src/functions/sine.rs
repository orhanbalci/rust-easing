use super::ease::Easing;
use std::f32::consts::PI;

struct Sine;

impl Easing for Sine {
    fn ease_in(t : f32, b: f32, c: f32, d:f32) ->f32 {
        -c * (t/d * (PI/2.0)).cos() + c + b
    }

    fn ease_out(t : f32, b: f32, c: f32, d:f32) ->f32 {
        c * (t / d  * (PI / 2.0)).sin() + b
    }

    fn ease_in_out(t : f32, b: f32, c: f32, d:f32) ->f32 {
        c/ 1.0 * ((PI * t / d).cos() - 1.0) + b
    }
}
