use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    window_id: window::Id,
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().size(1280, 720).view(view).build().unwrap();

    // Load the image from disk and upload it to a GPU texture.
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("watercolor.png");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model { window_id, texture }
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(DIMGRAY);
    let draw = app.draw();

	let n_points = 4;
    let radius = 200.0;
    let points = (0..n_points).map(|i| {
        let fract = (i as f32 / n_points as f32) + ((i+1)%2) as f32 * 0.1;
        let phase = fract +0.075 ;//the added value can be the random rotation value(only problem is if want a "normal" starting position for later calculation)
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        let point = pt2(x, y);
        let tex_coords = [point.x/1280.0 + 0.5, 1.0 - (point.y/640.0 + 0.5)];
        (point, tex_coords)
    });
	//need to create a double of the point structure to rewrite the polygon outline on top of it's texture
    let points2 = (0..n_points).map(|i| {
        let fract = (i as f32 / n_points as f32) + ((i+1)%2) as f32 * 0.1;
        let phase = fract +0.075 ;//the added value can be the random rotation value(only problem is if want a "normal" starting position for later calculation)
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        let point = pt2(x, y);
        point
    });

	draw.polygon()
		.points_textured(&model.texture, points);

	draw.polygon()
		.no_fill()
		.stroke(BLACK)
		.stroke_weight(2.0)
		.points(points2);
    // Draw to the frame!
    draw.to_frame(app, &frame).unwrap();
}