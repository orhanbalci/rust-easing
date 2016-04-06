pub trait Easing {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32;
    fn ease_out(t: f32, b: f32, c: f32, d: f32) -> f32;
    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32;
}
