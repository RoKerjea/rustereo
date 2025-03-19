
use nannou::prelude::*;
use nannou::event::Key;
use rand::Rng;

fn main() {
	nannou::app(model)
	.loop_mode(LoopMode::Wait)
	.update(update)
	.run();
}

struct Model {
	canvas: nannou::draw::Draw,
	height: u32,
	width: u32,
}

fn model(app: &App) -> Model {

	let height = 720;
	let width = 1280;
	let _w_id = app
		.new_window()
		.size(width, height)
		.title("Rustereo")
		.event(event)
		.view(view)
		.build()
		.unwrap();
	let canvas = watercolor(height, width);
	Model {
		canvas,
		height,
		width,
	}
}

fn watercolor_stroke(color: Hsv, _height:u32, _width:u32, canvas: &nannou::draw::Draw, point: &Vec<f32>)
{
	/*stroke point for watercolor effect
		- randomize the color, with a slight variation from the color given in parameter
		- randomize the position of the stroke(between min and max of screen for now)
		- randomize the radius of the stroke, with a defined range
		- randomize the number of points in the stroke??(for now, fixed at 12)
		TODO:
		-make each stroke close to the last stroke?
	 */
	let mut rng = rand::thread_rng();
	let n_points = 12;
    // let radius = _height as f32 * 0.25;
    let points_colored = (0..n_points).map(|i| {
        let fract = i as f32 / n_points as f32;
        let phase = fract;
		let radius = rng.gen_range(20.0..=50.0);
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
		let mut lin:LinSrgba = color.into();
		lin.red += rng.gen_range(-0.1..=0.1);
		lin.green += rng.gen_range(-0.1..=0.1);
		lin.blue += rng.gen_range(-0.1..=0.1);
		lin.alpha = 0.02;
        (pt2(x, y), lin)
    });
    canvas.polygon()
        .x(point[0])
		.y(point[1])
        .rotate(1.0 * 0.1)
        .points_colored(points_colored);
}

fn watercolor(_height:u32, _width:u32) -> nannou::draw::Draw {
	/*starting color
	random variation of 25 points from start color
	draw a circle
	draw a shape "like" a circle, but with less precision  */
	
		let mut rng = rand::thread_rng();
		let canvas = nannou::draw::Draw::new();
		let mut hsv = nannou::color::Hsv::new(rng.gen_range(0.0..=360.0), 0.5, 0.5);

		canvas.rect()
			.color(hsv)
			.w_h(_width as f32, _height as f32);
		// canvas.background().color(hsv);
		let mut point = vec![0.0, 0.0];
		let max_var = 20.0;
		let max_dist = 50.0;
		let range = 2.0;
		let ori_hue = hsv.hue.to_positive_degrees();
		for _ in 0..10000 {
			point[0] += rng.gen_range(-max_dist..=max_dist);
			point[1] += rng.gen_range(-max_dist..=max_dist);
			hsv.hue += rng.gen_range(-range..=range);
			hsv.saturation += rng.gen_range(-range..=range);
			hsv.value += rng.gen_range(-range..=range);
			if point[0] > _width as f32 / 2.0 {
				point[0] = -(_width as f32 / 2.0);
			}
			if point[0] < -(_width as f32 / 2.0) {
				point[0] = _width as f32 / 2.0;
			}
			if point[1] > _height as f32 / 2.0 {
				point[1] = -(_height as f32 / 2.0);
			}
			if point[1] < -(_height as f32 / 2.0) {
				point[1] = _height as f32 / 2.0;
			}
			if hsv.saturation > 1.0 {
				hsv.saturation = 1.0;
			}
			if hsv.saturation < 0.4 {
				hsv.saturation = 0.4;
			}
			if hsv.value > 0.7{
				hsv.value = 0.7;
			}
			if hsv.value < 0.3 {
				hsv.value = 0.30;
			}
			if hsv.hue.to_positive_degrees() > 360.0 {
				hsv.hue -= 360.0;
			}
			if hsv.hue.to_positive_degrees() < 0.0 {
				hsv.hue += 360.0;
			}
			if hsv.hue.to_positive_degrees() > ori_hue + max_var {
				hsv.hue = (ori_hue + max_var).into();
			}
			if hsv.hue.to_positive_degrees() < ori_hue - max_var {
				hsv.hue = (ori_hue - max_var).into();
			}
			watercolor_stroke(hsv, _height, _width, &canvas, &point);
		}
	
		let mut hsv2 = nannou::color::Hsv::new(rng.gen_range(0.0..=360.0), 0.5, 0.5);
		let ori_hue = hsv2.hue.to_positive_degrees();
		for _ in 0..1000 {
			point[0] += rng.gen_range(-max_dist..=max_dist);
			point[1] += rng.gen_range(-max_dist..=max_dist);
			hsv2.hue += rng.gen_range(-range..=range);
			hsv2.saturation += rng.gen_range(-range..=range);
			hsv2.value += rng.gen_range(-range..=range);
			if point[0] > _width as f32 / 2.0 {
				point[0] = -(_width as f32 / 2.0);
			}
			if point[0] < -(_width as f32 / 2.0) {
				point[0] = _width as f32 / 2.0;
			}
			if point[1] > _height as f32 / 2.0 {
				point[1] = -(_height as f32 / 2.0);
			}
			if point[1] < -(_height as f32 / 2.0) {
				point[1] = _height as f32 / 2.0;
			}
			if hsv2.saturation > 1.0 {
				hsv2.saturation = 1.0;
			}
			if hsv2.saturation < 0.4 {
				hsv2.saturation = 0.4;
			}
			if hsv2.value > 0.7{
				hsv2.value = 0.7;
			}
			if hsv2.value < 0.3 {
				hsv2.value = 0.30;
			}
			if hsv2.hue.to_positive_degrees() > 360.0 {
				hsv2.hue -= 360.0;
			}
			if hsv2.hue.to_positive_degrees() < 0.0 {
				hsv2.hue += 360.0;
			}
			if hsv2.hue.to_positive_degrees() > ori_hue + max_var {
				hsv2.hue = (ori_hue + max_var).into();
			}
			if hsv2.hue.to_positive_degrees() < ori_hue - max_var {
				hsv2.hue = (ori_hue - max_var).into();
			}
			watercolor_stroke(hsv, _height, _width, &canvas, &point);
		}
		canvas
	}

	fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
		match event {
			WindowEvent::KeyReleased(key) => {
				println!("{:?}", key);
				match key {
					Key::Space => {
						_model.canvas = watercolor(_model.height, _model.width);
					}
					_ => {}
				}
			}
			_ => {}
		}
	}
	
	fn update(_app: &App, _model: &mut Model, _update: Update) {
	}
	
	fn view(_app: &App, _model: &Model, frame: Frame) {
		_model.canvas.to_frame(_app, &frame).unwrap();
		let file_path = "assets/images/watercolor.png";
		_app.main_window().capture_frame(file_path);
	}