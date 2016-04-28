extern crate gnuplot;
extern crate easer;

use gnuplot::{Figure, Caption, Color, AxesCommon};
use easer::functions::*;

fn main() {
    let mut fg = Figure::new();
    plot_easing_function(&mut fg, Back::ease_in, Back::ease_out, Back::ease_in_out, 0);
    plot_easing_function(&mut fg,
                         Bounce::ease_in,
                         Bounce::ease_out,
                         Bounce::ease_in_out,
                         3);
    plot_easing_function(&mut fg, Circ::ease_in, Circ::ease_out, Circ::ease_in_out, 6);
    // plot_easing_function(&mut fg,
    // Cubic::ease_in,
    // Cubic::ease_out,
    // Cubic::ease_in_out,
    // 9);
    // plot_easing_function(&mut fg,
    // Elastic::ease_in,
    // Elastic::ease_out,
    // Elastic::ease_in_out,
    // 12);
    // plot_easing_function(&mut fg,
    // Expo::ease_in,
    // Expo::ease_out,
    // Expo::ease_in_out,
    // 15);
    // plot_easing_function(&mut fg,
    // Linear::ease_in,
    // Linear::ease_out,
    // Linear::ease_in_out,
    // 18);
    // plot_easing_function(&mut fg,
    // Quad::ease_in,
    // Quad::ease_out,
    // Quad::ease_in_out,
    // 21);
    // plot_easing_function(&mut fg,
    // Quart::ease_in,
    // Quart::ease_out,
    // Quart::ease_in_out,
    // 24);
    // plot_easing_function(&mut fg,
    // Quint::ease_in,
    // Quint::ease_out,
    // Quint::ease_in_out,
    // 27);
    // plot_easing_function(&mut fg,
    // Sine::ease_in,
    // Sine::ease_out,
    // Sine::ease_in_out,
    // 30);

    fg.show();
    // rintln!("Hello, world!");
}

fn plot_easing_function<F1, F2, F3>(fg: &mut Figure,
                                    fun_ease_in: F1,
                                    fun_ease_out: F2,
                                    fun_ease_in_out: F3,
                                    cell: u32)
    where F1: Fn(f32, f32, f32, f32) -> f32,
          F2: Fn(f32, f32, f32, f32) -> f32,
          F3: Fn(f32, f32, f32, f32) -> f32
{

    let mut x: [f32; 100] = [0.0; 100];
    let mut y: [f32; 100] = [0.0; 100];
    for i in 0..100 {
        x[i] = i as f32;
        y[i] = i as f32;
    }

    let back = y.iter()
                .map(|a| fun_ease_in(*a, 0f32, 100f32, 100f32))
                .collect::<Vec<f32>>();

    fg.axes2d()
      .lines(&x[..], &back, &[Caption("In Line"), Color("blue")])
      .set_title("Back Ease In", &[])
      .set_pos_grid(3, 3, cell);
    let back_ease_out = y.iter()
                         .map(|a| fun_ease_out(*a, 0f32, 100f32, 100f32))
                         .collect::<Vec<f32>>();

    fg.axes2d()
      .lines(&x[..],
             &back_ease_out,
             &[Caption("Out Line"), Color("blue")])
      .set_title("Back Ease Out", &[])
      .set_pos_grid(3, 3, cell + 1);

    let back_ease_in_out = y.iter()
                            .map(|a| fun_ease_in_out(*a, 0f32, 100f32, 100f32))
                            .collect::<Vec<f32>>();

    fg.axes2d()
      .lines(&x[..],
             &back_ease_in_out,
             &[Caption("In Out Line"), Color("blue")])
      .set_title("Back Ease In Out", &[])
      .set_pos_grid(3, 3, cell + 2);

}
