use rand::Rng;

fn random_color2() -> nannou::color::Rgba {
	let color1 = nannou::color::Rgba::new(96.0 / 255.0, 108.0 / 255.0, 56.0 / 255.0, 1.0);
	let color2 = nannou::color::Rgba::new(40.0 / 255.0, 54.0 / 255.0, 24.0 / 255.0, 1.0);
	let color3 = nannou::color::Rgba::new(254.0 / 255.0, 250.0 / 255.0, 224.0 / 255.0, 1.0);
	let color4 = nannou::color::Rgba::new(221.0 / 255.0, 161.0 / 255.0, 94.0 / 255.0, 1.0);
	let color5 = nannou::color::Rgba::new(188.0 / 255.0, 108.0 / 255.0, 37.0 / 255.0, 1.0);
	let mut rng = rand::thread_rng();
	let color = rng.gen_range(0..=4);
	match color {
		0 => color1,
		1 => color2,
		2 => color3,
		3 => color4,
		4 => color5,
		_ => color1,
	}
}
use nannou::prelude::*;
use nannou::event::Key;
// use nannou::LoopMode::Wait;

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
	let canvas = truchet(height, width);
	Model {
		canvas,
		height,
		width,
	}
}

/*function/struct per type of tiles:
2 curves
1cross
1line, 2 points
1 point 3 lines?
4 points?

array of structs
if rng x% then tile x

maybe general parameters for all tiles too:
color, weight, perturbation, size/pos, etc?
 */

fn truchet(height:u32, width:u32) -> nannou::draw::Draw
{
	let canvas = nannou::draw::Draw::new();
	canvas.background().color(WHITE);
	let col_count = 75;
    let radius = height as f32/(col_count as f32 * 2.0);
	let row_count = (width as f32/(radius*2.0)) as u32 + 1;
	let line_weight = 3.0;

	let color = random_color2();
	let n_points = 91;
    // let radius = height as f32 * 0.16;
    let points = (0..n_points).map(|i| {
        let fract = i as f32 / 360.0;
        let phase = fract;
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        (pt2(x, y), color)
    });
	let mut rng = rand::thread_rng();
	// eprintln!("rotation: {}", rotation);
	for col in 0..col_count{
		for row in 0..row_count{
			let rotation = rng.gen_range(0..=3);
			let rot = rotation as f32 * PI/2.0;
			let mut x = 0.0;
			let mut y = 0.0;
			match rotation {
				1 => {
					x += radius*2.0;
				}
				2 => {
					x += radius*2.0;
					y += radius*2.0;
				}
				3 => {
					y += radius*2.0;
				}
				_ => {}
			}
					canvas.polyline()
						.rotate(0.0 + rot)
						.x_y(x - (width as f32 /2.0) + (radius * 2.0 * row as f32), y + (height as f32 /2.0) -radius * 2.0 - (radius * 2.0 * col as f32))
						// .x_y(-radius*5.0 + (radius * 2.0 * x as f32), radius*5.0 - (radius * 2.0 * y as f32))
						.stroke_weight(line_weight)
						.join_round()
						.caps_round()
						.points_colored(points.clone());

					match rotation {
						0 => {
							x = radius*2.0;
							y = radius*2.0;
						}
						1 => {
							x = 0.0;
							y = radius*2.0;
						}
						2 => {
							x = 0.0;
							y = 0.0;
						}
						3 => {
							x = radius*2.0;
							y = 0.0;
						}
						_ => {}
					}
					canvas.polyline()
						.rotate(PI + rot)
						.x_y(x - (width as f32 /2.0) + (radius * 2.0 * row as f32), y + (height as f32 /2.0) - radius * 2.0- (radius * 2.0 * col as f32))
						// .x_y(-radius*7.0+ (radius * 2.0 * x as f32), radius*3.0 - (radius * 2.0 * y as f32))
						.stroke_weight(line_weight)
						.join_round()
						.caps_round()
						.points_colored(points.clone());
		}
	}
	canvas
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
	match event {
		WindowEvent::KeyReleased(key) => {
			println!("{:?}", key);
			match key {
				Key::Space => {
					_model.canvas = truchet(_model.height, _model.width);
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
	let file_path = "/mnt/nfs/homes/rokerjea/rustereo/assets/images/truchet.png";
    _app.main_window().capture_frame(file_path);
}