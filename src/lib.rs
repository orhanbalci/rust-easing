//! This library implements Robert Penner's easing functions.
//! Easing functions calculate rate of change of a property over time.
//!
//! A little website [easings.net](http://easings.net) can help visualising
//! functions. Another can help you understanding Easing functions in details
//! [upshots.org/jsas-understanding-easing](http://upshots.org/actionscript/jsas-understanding-easing).
//!
//! - **`t`** is the current time (or position) of the tween.
//! This can be seconds or frames, steps, seconds, ms, whatever
//! as long as the unit is the same as is used for the total time.
//! - **`b`** is the beginning value of the property.
//! - **`c`** is the change between the beginning and destination value of the property.
//! - **`d`** is the total time of the tween.
//!
//! #example
//!
//! ```no_run
//! use easer::functions::*;
//! let mut y: [f32; 100] = [0.0; 100];
//! for i in 0..100 {
//!     y[i] = i as f32;
//! }
//! println!("Before {:?}", &y[..]);
//! y.iter_mut().map(|a| *a = Back::ease_in(*a, 0f32, 100f32, 100f32)).count();
//! println!("After {:?}", &y[..]);
//!
//! ```

pub mod functions;

#[cfg(test)]
#[macro_use]
extern crate approx;
