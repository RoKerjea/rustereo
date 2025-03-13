use nannou::{noise::{NoiseFn, Perlin, Seedable}, prelude::*};

fn main() {
    nannou::sketch(view).run()
}
// i need to generate a vec of color from a field in a json file

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let win = app.window_rect();
    let draw = app.draw();
	let mut perl = Perlin::new();
    perl = perl.set_seed(1);
    draw.background().color(WHITE);
	let rez = 0.2;

	for i in 0..(win.w() /3.0) as u32 {
		for j in 0..(win.h()/3.0) as u32 {
			let dimension = [i as f64 * rez,j as f64 * rez];
			let n = perl.get(dimension) as f32;
			// eprintln!("n = {}", n);
			let color = Hsva::new(0.0, 0.0, abs(n), 1.0);
			draw.rect()
			.color(color)
			.w_h(6.0, 6.0)
			.x(-win.w()/2.0 + (i * 3) as f32)
			.y(win.h()/2.0 - (j * 3) as f32);
		}
	}
	// eprintln!("done!")
    draw.to_frame(app, &frame).unwrap();
}