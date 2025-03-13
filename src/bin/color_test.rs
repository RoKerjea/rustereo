use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}
// i need to generate a vec of color from a field in a json file

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let win = app.window_rect();
    let draw = app.draw();

    draw.background().color(BLACK);
	let width = win.w() / 5.0;
	let height = win.h() / 2.0;
	let mut palette = vec![];
	let mut color = nannou::color::Rgb::new(0.0, 48.0/255.0, 73.0/255.0);
	let color1 :Hsv = Hsv::from(color);
	palette.push(color1);
	color = nannou::color::Rgb::new(214.0/255.0, 40.0/255.0, 40.0/255.0);
	let color2 :Hsv = Hsv::from(color);
	palette.push(color2);
	color = nannou::color::Rgb::new(247.0/255.0, 127.0/255.0, 0.0/255.0);
	let color3 :Hsv = Hsv::from(color);
	palette.push(color3);
	 color = nannou::color::Rgb::new(252.0/255.0, 191.0/255.0, 73.0/255.0);
	let color4 :Hsv = Hsv::from(color);
	palette.push(color4);
	color = nannou::color::Rgb::new(234.0/255.0, 226.0/255.0, 183.0/255.0);
	let color5 :Hsv = Hsv::from(color);
	palette.push(color5);
	let mut palette2 = vec![];
	let mut color = nannou::color::Rgb::new(96.0 / 255.0, 108.0 / 255.0, 56.0 / 255.0);
	let color1 :Hsv = Hsv::from(color);
	palette2.push(color1);
	color = nannou::color::Rgb::new(40.0 / 255.0, 54.0 / 255.0, 24.0 / 255.0);
	let color2 :Hsv = Hsv::from(color);
	palette2.push(color2);
	color = nannou::color::Rgb::new(254.0 / 255.0, 250.0 / 255.0, 224.0 / 255.0);
	let color3 :Hsv = Hsv::from(color);
	palette2.push(color3);
	 color = nannou::color::Rgb::new(221.0 / 255.0, 161.0 / 255.0, 94.0 / 255.0);
	let color4 :Hsv = Hsv::from(color);
	palette2.push(color4);
	color = nannou::color::Rgb::new(188.0 / 255.0, 108.0 / 255.0, 37.0 / 255.0);
	let color5 :Hsv = Hsv::from(color);
	palette2.push(color5);

	let mut palettes = vec![];
	palettes.push(palette);
	palettes.push(palette2);
	nannou::io::save_to_json("assets/palette.json", &palettes).expect("Error saving file");
	let colors :Vec<Vec<Hsv>>= nannou::io::load_from_json("assets/palette.json").expect("Error loading palette");
	// let color = nannou::color::Hsv::new(0.0, 0.81, 0.170);
	// eprintln!("h : {:?} s: {:?} v :{:?}", color.hue, color.saturation, color.value);
	for i in 0..5 {
		draw.rect()
			.w_h(width, height)
			.x_y(-win.w() / 2.0 + width / 2.0 + width * i as f32,0.0)
			.color(colors[1][i])
			.stroke(BLACK)
			.stroke_weight(2.0);
	}

    draw.to_frame(app, &frame).unwrap();
}