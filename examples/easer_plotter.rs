extern crate gnuplot;
extern crate easer;

use gnuplot::{Figure, Caption, Color, AxesCommon};
use easer::functions::*;
use std::fmt::Debug;

fn main() {
    let mut fg = Figure::new();
    const ROWS: u32 = 3;

    plot_easing(Back, &mut fg, 0, ROWS);
    plot_easing(Bounce, &mut fg, 1, ROWS);
    plot_easing(Circ, &mut fg, 2, ROWS);
    // plot_easing(Cubic, &mut fg, 3, ROWS);
    // plot_easing(Elastic, &mut fg, 4, ROWS);
    // plot_easing(Expo, &mut fg, 5, ROWS);
    // plot_easing(Linear, &mut fg, 6, ROWS);
    // plot_easing(Quad, &mut fg, 7, ROWS);
    // plot_easing(Quart, &mut fg, 8, ROWS);
    // plot_easing(Quint, &mut fg, 9, ROWS);
    // plot_easing(Sine, &mut fg, 10, ROWS);

    fg.show();
}

fn plot_easing<E: Easing<f32> + Debug>(easing: E, fg: &mut Figure, row_idx: u32, nrows: u32) {
    let mut x: [f32; 100] = [0.0; 100];
    let mut y: [f32; 100] = [0.0; 100];
    for i in 0..100 {
        x[i] = i as f32;
        y[i] = i as f32;
    }

    let back = y.iter()
                .map(|a| E::ease_in(*a, 0f32, 100f32, 100f32))
                .collect::<Vec<f32>>();

    fg.axes2d()
      .lines(&x[..], &back, &[Caption("In Line"), Color("blue")])
      .set_title(&format!("{:?} Ease In", easing), &[])
      .set_pos_grid(nrows, 3, row_idx * 3);
    let back_ease_out = y.iter()
                         .map(|a| E::ease_out(*a, 0f32, 100f32, 100f32))
                         .collect::<Vec<f32>>();

    fg.axes2d()
      .lines(&x[..],
             &back_ease_out,
             &[Caption("Out Line"), Color("blue")])
      .set_title(&format!("{:?} Ease Out", easing), &[])
      .set_pos_grid(nrows, 3, row_idx * 3 + 1);

    let back_ease_in_out = y.iter()
                            .map(|a| E::ease_in_out(*a, 0f32, 100f32, 100f32))
                            .collect::<Vec<f32>>();

    fg.axes2d()
      .lines(&x[..],
             &back_ease_in_out,
             &[Caption("In Out Line"), Color("blue")])
      .set_title(&format!("{:?} Ease In Out", easing), &[])
      .set_pos_grid(nrows, 3, row_idx * 3 + 2);

}
