//! This library implements Robert Penner's easing functions
//! Easing functions calculate rate of change of a property over time
//!
//! #example
//!
//! ```no_run
//! use easer::functions::*;
//! let mut x: [f32; 100] = [0.0; 100];
//! let mut y: [f32; 100] = [0.0; 100];
//! for i in 0..100 {
//!     x[i] = i as f32;
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
