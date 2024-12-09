use image::{Pixel, RgbaImage, imageops};
use rand::Rng;
use imageproc::{drawing::{self, draw_line_segment_mut, Blend, Canvas}, pixelops::interpolate, point::Point, rect::Rect};
/*
fn main() {
	let imgx = 80;
	let imgy = 640;

	let mut imgbuf1 = image::ImageBuffer::new(imgx, imgy);
	let mut rng = rand::thread_rng();
	for (_x, _y, pixel) in imgbuf1.enumerate_pixels_mut() {
		let color = rng.gen_range(0..=2);
		let ratio = ((0.25 * _y as f32) *0.75) as u8;
		let ratio2 = _y as f32 / (640.0 / 2.0);
		match color {
			0 => *pixel = image::Rgb([0, 0, 200-ratio]),
			1 => *pixel = image::Rgb([255 - ratio, 165 - ratio, 0]),
			2 => *pixel = image::Rgb([255 - ratio, 165 - ratio, 0]),
			_ => *pixel = image::Rgb([0, 0, 0]),
		}
			// *pixel = image::Rgb([255 * color, 255* color, 255* color]);

	}
	let mut finalimg = image::ImageBuffer::new(640, 640);
	let input = image::open("42_Logo.svg.png").unwrap();

	for (x, y, pixel) in imgbuf1.enumerate_pixels_mut() {
		finalimg.put_pixel(x, y, *pixel);
	}
	for stripe in 1..8 {
		for x in 0..80 {
			for y in 0..640 {
				let pixel = finalimg.get_pixel(x, y);
				// if y > 80 && y < 560 && (x+(80*(stripe-1)) < 480) && (stripe == 1 || stripe == 2 || stripe == 3)
				// {
				// 	let inputpixel = input.get_pixel(x+(80*(stripe-1)), y-80);
				// 	if inputpixel == image::Rgba([0, 0, 0, 255]) {
				// 		pixel = finalimg.get_pixel(x+2+(80*(stripe-1)), y);
				// 		// pixel = &image::Rgb([255, 255, 255]);
				// 	}
				// }
				finalimg.put_pixel(x+(80*stripe), y, *pixel);
			}
		}
	}

	// image::save_buffer("image_pre-shift.png", &finalimg, 640, 640, image::ExtendedColorType::Rgb8).unwrap();
	for stripe in 1..8 {
		for x in 0..80 {
			for y in 0..640 {
				// let mut pixel = finalimg.get_pixel(x, y);
				if y > 80 && y < 560 && (x+(80*(stripe-1)) < 480) 
				{
					let inputpixel = input.get_pixel(x+(80*(stripe-1)), y-80);
					if inputpixel == image::Rgba([0, 0, 0, 255]) {
						let mut shift = 4;
						// if stripe > 4 {
						// 	shift = 3;
						// }
						let &pixel = finalimg.get_pixel(x+shift+(80*(stripe-1)), y);
						// let &pixel2 = finalimg.get_pixel(x+(80*(stripe-1)), y);
						for i in stripe..8 {
							finalimg.put_pixel(x+(80*i), y, pixel);
						}
						// finalimg.put_pixel(x+(80*(stripe+1)), y, pixel);
						// finalimg.put_pixel(x+(80*(stripe+2)), y, pixel);
						finalimg.put_pixel(x+(80*stripe), y, pixel);
						// pixel = &image::Rgb([255, 255, 255]);
					}
				}
			}
		}
	}
	// for (x, y, pixel) in input.to_rgba8().enumerate_pixels() {
	// 	if *pixel == image::Rgba([0, 0, 0, 255]) {
	// 		finalimg.put_pixel(x+80, y+80, image::Rgb([255, 255, 255]));
	// 	}
	// }

	/*
	for stripe{
		for x y in stripe
		pixel = stripe-1
		if ref.pixel(x -80, y-80)
			then shift
			} */
	// imgbuf.save("first_test.png").unwrap();
	// image::save_buffer("image.png", &imgbuf1, 80, 200, image::ExtendedColorType::Rgb8).unwrap();
	image::save_buffer("image2.png", &finalimg, 640, 640, image::ExtendedColorType::Rgb8).unwrap();
}
*/

fn draw_rectangle(canvas: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, x: u32, y: u32, r: u32, color: image::Rgb<u8>) {
	for i in x-r..x+r {
		for j in y-r..y+r {
				canvas.put_pixel(i, j, color);
		}
	}
	let black = image::Rgb([0, 0, 0]);
	for i in x-r..=x+r {
		canvas.put_pixel(i, y-r, black);
		canvas.put_pixel(i, y+r, black);
	}
	for j in y-r..y+r {
		canvas.put_pixel(x-r, j, black);
		canvas.put_pixel(x+r, j, black);
	}
}

fn draw_circle(canvas: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, x: u32, y: u32, r: u32, color: image::Rgba<u8>) {
	let xref = x as i32;
	let yref = y as i32;
	let r = r as i32;
	let black = image::Rgb([0, 0, 0]);
	for i in xref-r..xref+r {
		for j in yref-r..yref+r {
			//i go from 270 to 370
			if (i-xref)*(i-xref) + (j-yref)*(j-yref) <= r*r {
				canvas.put_pixel(i as u32, j as u32, color);
				
			}
		}
	}
}

pub fn draw_filled_circle_mut2<C>(canvas: &mut C, center: (i32, i32), radius: i32, color: C::Pixel)
where
    C: Canvas,
{
    let mut x = 0i32;
    let mut y = radius;
    let mut p = 1 - radius;
    let x0 = center.0;
    let y0 = center.1;
	//i need a vector of pixels i aldready coloried
	//need to be of size radius*2, and initialized to 0
	//TODO: refacto to reduce size of the vector
	let mut pixels = vec![0; canvas.height() as usize];
    while x <= y {
		if pixels[(y0 + y) as usize] == 0 {
			draw_line_segment_mut(
				canvas,
				((x0 - x) as f32, (y0 + y) as f32),
				((x0 + x) as f32, (y0 + y) as f32),
				color,
			);
			pixels[(y0 + y) as usize] = 1;
		}
		if pixels[(y0 +x) as usize] == 0 {
			draw_line_segment_mut(
				canvas,
				((x0 - y) as f32, (y0 + x) as f32),
				((x0 + y) as f32, (y0 + x) as f32),
				color,
			);
			pixels[(y0 + x) as usize] = 1;
		}
		if pixels[(y0 - y) as usize] == 0 {
			draw_line_segment_mut(
				canvas,
				((x0 - x) as f32, (y0 - y) as f32),
				((x0 + x) as f32, (y0 - y) as f32),
				color,
			);
			pixels[(y0 - y) as usize] = 1;
		}
		if pixels[(y0 - x) as usize] == 0 {
			draw_line_segment_mut(
				canvas,
				((x0 - y) as f32, (y0 - x) as f32),
				((x0 + y) as f32, (y0 - x) as f32),
				color,
			);
			pixels[(y0 - x) as usize] = 1;
		}

        x += 1;
        if p < 0 {
            p += 2 * x + 1;
        } else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }
    }
}

// fn main() {
// 	let imgx = 640;
// 	let imgy = 640;

// 	let mut imgbuf1 = image::ImageBuffer::new(imgx, imgy);
// 	let mut rng = rand::thread_rng();
// 	for (_x, _y, pixel) in imgbuf1.enumerate_pixels_mut() {
// 		let color = rng.gen_range(0..=2);
// 		let ratio = ((0.25 * _y as f32) *0.75) as u8;
// 		let ratio2 = _y as f32 / (640.0 / 2.0);
// 		match color {
// 			0 => *pixel = image::Rgba([0, 0, 200-ratio, 127]),
// 			1 => *pixel = image::Rgba([255 - ratio, 165 - ratio, 0, 127]),
// 			2 => *pixel = image::Rgba([255 - ratio, 165 - ratio, 0, 127]),
// 			_ => *pixel = image::Rgba([0, 0, 0, 255]),
// 		}
// 			// *pixel = image::Rgb([255 * color, 255* color, 255* color]);

// 	}
	
// 	let pixel = image::Rgba([255, 255, 255, 255]);
// 	let mut canvas = Blend(RgbaImage::from_pixel(640, 640, pixel));// can be used to genrate a new canvas with a single color
// 	// let mut canvas1 = image::ImageBuffer::from_pixel(640, 640, pixel);
// 	// let mut canvas = Blend(canvas1);
// 	let red = image::Rgba([255, 0, 0, 255]);
// 	let blue = image::Rgba([0, 0, 255, 255]);
// 	let black = image::Rgba([0, 0, 0, 255]);
// 	let grey = image::Rgba([128, 128, 128, 127]);
// 	// draw_rectangle(&mut canvas, 320, 320, 100, red);
// 	// draw_circle(&mut canvas, 320, 320, 50, blue);
// 	// let mut canvas = image::ImageBuffer::new(640, 640);
// 	// let color = image::Rgb([255, 0, 0]);
// 	// let blue = image::Rgb([0, 0, 255]);
// 	let blue2 = image::Rgba([0, 0, 255, 127]);
// 	drawing::draw_filled_rect_mut(&mut canvas, Rect::at(320, 320).of_size(100, 100), red);
// 	drawing::draw_hollow_rect_mut(&mut canvas, Rect::at(320, 320).of_size(100, 100), black);
// 	draw_filled_circle_mut2(&mut canvas, (300, 300), 50, grey);
// 	// draw_circle(&mut canvas.0, 300, 300, 50, blue2);
// 	drawing::draw_hollow_circle_mut(&mut canvas, (300, 300), 50, black);
// 	// image::save_buffer("video1.png", &canvas, 640, 640, image::ExtendedColorType::Rgba8).unwrap();
// 	// let Blend(canvas) = image;
// 	image::imageops::overlay(&mut canvas.0, &imgbuf1, 50, 50);
// 	canvas.0.save("video1.png").unwrap();
// }



fn random_color() -> image::Rgba<u8> {
	let color1 = image::Rgba([96, 108, 56, 255]);
	let color2 = image::Rgba([40, 54, 24, 255]);
	let color3 = image::Rgba([254, 250, 224, 255]);
	let color4 = image::Rgba([221, 161, 94, 255]);
	let color5 = image::Rgba([188, 108, 37, 255]);
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

fn draw_bordered_circle(canvas: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, x: i32, y: i32, r: u32, color: image::Rgba<u8>) {

	let black = image::Rgba([0, 0, 0, 255]);
	drawing::draw_filled_circle_mut(canvas, (x, y),10, color);
	drawing::draw_hollow_circle_mut(canvas, (x, y),10, black);
}

fn draw_bordered_rect(canvas: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, x: i32, y: i32, r: u32, color: image::Rgba<u8>) {
	let black = image::Rgba([0, 0, 0, 255]);
		drawing::draw_filled_rect_mut(canvas, Rect::at(x, y).of_size(r, r), color);
		drawing::draw_hollow_rect_mut(canvas, Rect::at(x, y).of_size(r, r), black);
}
/*
fn main()
{
	let xmax = 640;
	let ymax = 640;

	let white = image::Rgba([255, 255, 255, 255]);
	let black = image::Rgba([0, 0, 0, 255]);
	let mut canvas = RgbaImage::from_pixel(640, 640, white);
	let mut rng = rand::thread_rng();
	for _ in 0..500 {
		let x:i32 = rng.gen_range(0..80);
		let y:i32 = rng.gen_range(0..ymax);
		let color = random_color();
		// draw_bordered_circle(&mut canvas, x, y, 10, color);
		// if x<10 {
		// 	draw_bordered_circle(&mut canvas, x+80, y, 10, color);
		// }
		// if x>70 {
		// 	draw_bordered_circle(&mut canvas, x-80, y, 10, color);
		// }
		draw_bordered_rect(&mut canvas, x, y, 20, color);
		// if x<10 {
		// 	draw_bordered_rect(&mut canvas, x+80, y, 20, color);
		// }
		if x>60 {
			draw_bordered_rect(&mut canvas, x-80, y, 20, color);
		}
	}
	// for j in 0..ymax/10 {
	// 	for i in 0..xmax/80 {
	// 		let color = random_color();
	// 		drawing::draw_filled_rect_mut(&mut canvas, Rect::at(10*i, 10*j).of_size(10, 10), color);
	// 		// drawing::draw_hollow_rect_mut(&mut canvas, Rect::at(10*i, 10*j).of_size(10, 10), black);
	// 		let color = random_color();
	// 		// drawing::draw_filled_circle_mut(&mut canvas, (10*i+5, 10*j+5), 4, color);
	// 		// i need a slice of 3 points
	// 		let points = [
	// 			Point::new(10*i+2, 10*j+2),
	// 			Point::new(10*i+8, 10*j+5),
	// 			Point::new(10*i+2, 10*j+8),
	// 		];
	// 		// let mut blend = color.clone();
	// 		// blend.blend(&color);
	// 		// let blend = Blend::Alpha(color);
	// 		drawing::draw_antialiased_polygon_mut(&mut canvas, &points, color, interpolate);
	// 		let points_float = [
	// 			Point::new(10.0*i as f32+2.0, 10.0*j as f32+2.0),
	// 			Point::new(10.0*i as f32+8.0, 10.0*j as f32+5.0),
	// 			Point::new(10.0*i as f32+2.0, 10.0*j as f32+8.0),
	// 		];
	// 		drawing::draw_hollow_polygon_mut(&mut canvas, &points_float, image::Rgba([0, 0, 0, 255]));
	// 	}
	// }
	// copy stripes from first generated image
	for stripe in 1..8 {
		for x in 0..80 {
			for y in 0..640 {
				let pixel = canvas.get_pixel(x, y);
				canvas.put_pixel(x+(80*stripe), y, *pixel);
			}
		}
	}
	// //apply pixel shift to generate final image

	let input = image::open("42_Logo.svg.png").unwrap();
	for stripe in 1..8 {
		for x in 0..80 {
			for y in 0..640 {
				if y > 80 && y < 560 && (x+(80*(stripe-1)) < 480) 
				{
					let inputpixel = input.get_pixel(x+(80*(stripe-1)), y-80);
					if inputpixel == image::Rgba([0, 0, 0, 255]) {
						let shift = 4;
						let &pixel = canvas.get_pixel(x+shift+(80*(stripe-1)), y);
						for i in stripe..8 {
							canvas.put_pixel(x+(80*i), y, pixel);
						}
						canvas.put_pixel(x+(80*stripe), y, pixel);
					}
				}
			}
		}
	}
	image::save_buffer("random_squares.png", &canvas, 640, 640, image::ExtendedColorType::Rgba8).unwrap();
}*/
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
use nannou::image::GenericImage;
use nannou::image::GenericImageView;
// use image::GenericImage;
use nannou::LoopMode::Wait;
use nannou::wgpu::TextureBuilder;
/*
fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // get canvas to draw on
	app.set_loop_mode(Wait);
    let draw = app.draw();

    // set background to blue
    draw.background().color(PLUM);
	// draw.ellipse().color(STEELBLUE);
	for y in 0..80{
		for x in 0..10{
			let color1 = random_color2();
			for stripe in 0..8 {
				draw.rect()
					.color(color1)
					.w_h(10.0, 10.0)
					.x_y(-450.0 + (100 * stripe)as f32 + (x as f32*10.0), 400.0 - (y as f32 *10.0));
			}
			let mut color2 = random_color2();
			while color1 == color2{
				color2 = random_color2();
			}
			for stripe in 0..8 {
				draw.ellipse()
					.color(color2)
					.w_h(7.0,7.0)
					.x_y(-450.0 + (100 * stripe)as f32+ (x as f32*10.0), 400.0 - (y as f32 *10.0));
			}
		}
	}
    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
*/
/*
fn main() {
  nannou::app(model).run();
}
struct Model {
	texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
  // Create a new window!
  	app.new_window().size(800, 800).view(view).build().unwrap();
  // Load the image from disk and upload it to a GPU texture.
//   let assets = app.assets_path().unwrap();
//   let img_path = "/mnt/nfs/homes/rokerjea/rustereo/assets/images/image2.png";
	let input = nannou::image::open("/mnt/nfs/homes/rokerjea/rustereo/assets/images/42_Logo.svg.png").unwrap();
	let mut canvas = nannou::image::open("/mnt/nfs/homes/rokerjea/rustereo/assets/images/image_pre-shift.png").unwrap();
  	for stripe in 1..8 {
		for x in 0..80 {
			for y in 0..640 {
				// let mut pixel = finalimg.get_pixel(x, y);
				if y > 80 && y < 560 && (x+(80*(stripe-1)) < 480)
				{
					let inputpixel = input.get_pixel(x+(80*(stripe-1)), y-80);
					if inputpixel == nannou::image::Rgba([0, 0, 0, 255]) {
						let shift = 5;
						let pixel = canvas.get_pixel(x+shift+(80*(stripe-1)), y);
						for i in stripe..8 {
							canvas.put_pixel(x+(80*i), y, pixel);
						}
						canvas.put_pixel(x+(80*stripe), y, pixel);
					}
				}
			}
		}
	}
	

	let texture = wgpu::Texture::from_image(app, &canvas);
	Model { texture }
}
/*
fn update(app: &App, model: &mut Model, update: Update) {
    let draw = app.draw();

    // set background to blue
    draw.background().color(PLUM);
	// draw.ellipse().color(STEELBLUE);
	for y in 0..80{
		for x in 0..10{
			let color1 = random_color2();
			for stripe in 0..8 {
				draw.rect()
					.color(color1)
					.w_h(10.0, 10.0)
					.x_y(-450.0 + (100 * stripe)as f32 + (x as f32*10.0), 400.0 - (y as f32 *10.0));
			}
			let mut color2 = random_color2();
			while color1 == color2{
				color2 = random_color2();
			}
			for stripe in 0..8 {
				draw.ellipse()
					.color(color2)
					.w_h(7.0,7.0)
					.x_y(-450.0 + (100 * stripe)as f32+ (x as f32*10.0), 400.0 - (y as f32 *10.0));
			}
		}
	}
    // put everything on the frame
    // draw.to_frame(app, &frame).unwrap();
	// model.texture = wgpu::Texture::from_image(app, &canvas);
}*/

fn view(app: &App, model: &Model, frame: Frame) {
	frame.clear(BLACK);

	let draw = app.draw();
	draw.texture(&model.texture);
  
	draw.to_frame(app, &frame).unwrap();
}*/

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
	let canvas = draw_picture(height, width);
	Model {
		canvas,
		height,
		width,
	}
}

fn draw_picture(height:u32, width:u32 ) -> nannou::draw::Draw
{
	let s_width = width / 8;//stripe width
	let s_row_size = (s_width / 10) as f32;//size of elements in a row, for 10 elements per row
	let canvas = nannou::draw::Draw::new();
	// canvas.background().color(PLUM);
	// draw.ellipse().color(STEELBLUE);
	for y in 0..s_width{
		for x in 0..10{
			let color1 = random_color2();
			for stripe in 0..8 {
				canvas.rect()
					.color(color1)
					.w_h(s_row_size, s_row_size)
					.x_y(- ((width/2) as f32) + (s_width * stripe)as f32 + (x as f32*s_row_size), (height/2) as f32 - (y as f32 *s_row_size));
			}
			// let mut color2 = random_color2();
			// while color1 == color2{
			// 	color2 = random_color2();
			// }
			// for stripe in 0..8 {
			// 	canvas.ellipse()
			// 		.color(color2)
			// 		.w_h(7.0,7.0)
			// 		.x_y(-450.0 + (100 * stripe)as f32+ (x as f32*10.0), 400.0 - (y as f32 *10.0));
			// }
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
					_model.canvas = draw_picture(_model.height, _model.width);
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
	// let color = random_color2();
	_model.canvas.to_frame(_app, &frame).unwrap();
	// frame.clear(_model.color);
}