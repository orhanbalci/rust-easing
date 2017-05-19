use num_traits::Float;

/// Interface encapsulating general easing functions
///
/// - **`t`** is the current time (or position) of the tween.
/// This can be seconds or frames, steps, seconds, ms, whatever
/// as long as the unit is the same as is used for the total time.
/// - **`b`** is the beginning value of the property.
/// - **`c`** is the change between the beginning and destination value of the property.
/// - **`d`** is the total time of the tween.
pub trait Easing<F: Float> {
    fn ease_in(t: F, b: F, c: F, d: F) -> F;
    fn ease_out(t: F, b: F, c: F, d: F) -> F;
    fn ease_in_out(t: F, b: F, c: F, d: F) -> F;
}
