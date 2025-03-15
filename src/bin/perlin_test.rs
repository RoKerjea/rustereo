use std::u32::MAX;

use nannou::{color::Hue, noise::{NoiseFn, Perlin, Seedable}, prelude::*};
use rand::Rng;

fn main() {
    nannou::sketch(view).run()
}


fn view(app: &App, frame: Frame) {
    // Begin drawing
    let win = app.window_rect();
    let draw = app.draw();
	// let mut rng = rand::thread_rng();
	// let sedd = rng.gen_range(0..=MAX);
	let mut perl = Perlin::new();
    perl = perl.set_seed(500000);
    draw.background().color(WHITE);
	let rez = 0.008;
	let color = nannou::color::Rgb::new(247.0 / 255.0, 127.0 / 255.0, 0.0 / 255.0);
	let color1 :Hsv = Hsv::from(color);
	let color2 = color1.shift_hue(90.0);
	let color3 = color1.shift_hue(180.0);
	let color4 = color1.shift_hue(270.0);
	for i in (0..win.w() as u32).step_by(3) {
		for j in (0..win.h() as u32).step_by(3) {
			let dimension = [i as f64 * rez,j as f64 * rez];
			let n = (perl.get(dimension) as f32 + 1.0) / 2.0;//nannou noise return a value between 1 an -1, so i need to transform that into a value between 0 and 1
			// eprintln!("n = {}", n);
			let tcolor = color1.shift_hue(n * 360.0);
			// let tcolor = nannou::color::Rgb::new(1.0 * n, 1.0*n, 1.0*n);
			// let tcolor;
			// if n < 0.2 {
			// 	tcolor = color1;
			// } else if n < 0.4 {
			// 	tcolor = color2;
			// }else if n < 0.6 {
			// 	tcolor = color3;
			// }else {
			// 	tcolor = color4;
			// }
			draw.rect()
			.color(tcolor)
			.w_h(3.0, 3.0)
			.x(-win.w()/2.0 + i as f32)
			.y(win.h()/2.0 - j as f32);
		}
	}
	// eprintln!("done!")
    draw.to_frame(app, &frame).unwrap();
}