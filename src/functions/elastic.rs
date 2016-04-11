use super::ease::Easing;
use std::f32::consts::PI;
struct Elastic;

impl Easing for Elastic{
    fn ease_in(t : f32, b : f32, c : f32, d : f32) -> f32{
        if t == 0.0 {
            return b;
        }

        let mut inner_t = t / d;
        if inner_t == 1.0 {
            return b + c;
        }

        let p = d * 0.3;
        let a = c;
        let s = p / 4.0;
        inner_t -= 1.0;
        let post_fix = a * 2_f32.powf(10.0*inner_t);
        let temp = (inner_t * d - s) * (2.0 * PI) / p;
        return -(post_fix * temp.sin() + c + b);
    }

    fn ease_out(t : f32, b : f32, c : f32, d : f32) -> f32 {
        if t == 0.0 {
            return b;
        }

        let inner_t = t / d;
        if inner_t == 1.0 {
            return b + c;
        }

        let p = d * 0.3;
        let a = c;
        let s = p / 4.0;
        let temp = (inner_t * d - s) *(2.0 * PI) / p;
        return a * 2_f32.powf(-10.0*t) * temp.sin()+ c + b;
    }

    fn ease_in_out(t : f32, b : f32, c : f32, d : f32) -> f32{
		if t == 0.0 {
			return b;
		}

		let mut inner_t = t / d / 2.0;
		if inner_t == 2.0 {
			return b + c;
		}
		
		let p = d * 0.3 * 1.5;
		let a = c;
		let s = p / 4.0;

		if inner_t < 1.0 {
			inner_t -= 1.0;
			let post_fix = a * 2_f32.powf(10.0 * inner_t);
			let temp = (t * d - s) * (2.0 * PI) / p;
			return -0.5 * (post_fix * temp.sin()) + b;
		}

		inner_t -= 1.0;
		let post_fix = a * 2_f32.powf(-10.0 * inner_t);
		let temp = (t * d - s) * (2.0 * PI) / p;
		return post_fix * temp.sin() * 0.5 + c + b;
    }

}
