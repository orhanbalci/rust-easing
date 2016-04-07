use super::ease::Easing;

struct Circ;

impl Easing for Circ {

    fn ease_in(t : f32, b : f32, c : f32, d : f32)-> f32 {
        let inner_t = t / d;
        -c * ((1.0 - inner_t * inner_t).sqrt() - 1.0) + b
    }

    fn ease_out(t:f32, b : f32, c : f32, d : f32) -> f32{
        let inner_t = t / d - 1.0;
        c * (1.0 - inner_t * inner_t).sqrt() +b
    }

    fn ease_in_out( t : f32, b : f32, c : f32, d : f32)-> f32{
        let mut inner_t = t / (d / 2.0);
        if inner_t < 1.0 {
            return -c/2.0 * ((1.0 - inner_t * inner_t).sqrt() -1.0) +  b;
        }

        inner_t -= 2.0;
        return c/2.0 * ((1.0 - inner_t * inner_t).sqrt() + 1.0 ) + b;

    }

}
