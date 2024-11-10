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

fn main()
{
	let xmax = 640;
	let ymax = 640;

	let white = image::Rgba([255, 255, 255, 255]);
	let black = image::Rgba([0, 0, 0, 255]);
	let mut canvas = RgbaImage::from_pixel(640, 640, white);
	let rng = rand::thread_rng();
	for j in 0..ymax/10 {
		for i in 0..xmax/80 {
			let color = random_color();
			drawing::draw_filled_rect_mut(&mut canvas, Rect::at(10*i, 10*j).of_size(10, 10), color);
			// drawing::draw_hollow_rect_mut(&mut canvas, Rect::at(10*i, 10*j).of_size(10, 10), black);
			let color = random_color();
			// drawing::draw_filled_circle_mut(&mut canvas, (10*i+5, 10*j+5), 4, color);
			// i need a slice of 3 points
			let points = [
				Point::new(10*i+2, 10*j+2),
				Point::new(10*i+8, 10*j+5),
				Point::new(10*i+2, 10*j+8),
			];
			// let mut blend = color.clone();
			// blend.blend(&color);
			// let blend = Blend::Alpha(color);
			drawing::draw_antialiased_polygon_mut(&mut canvas, &points, color, interpolate);
			let points_float = [
				Point::new(10.0*i as f32+2.0, 10.0*j as f32+2.0),
				Point::new(10.0*i as f32+8.0, 10.0*j as f32+5.0),
				Point::new(10.0*i as f32+2.0, 10.0*j as f32+8.0),
			];
			drawing::draw_hollow_polygon_mut(&mut canvas, &points_float, image::Rgba([0, 0, 0, 255]));
		}
	}
	//copy stripes from first generated image
	for stripe in 1..8 {
		for x in 0..80 {
			for y in 0..640 {
				let pixel = canvas.get_pixel(x, y);
				canvas.put_pixel(x+(80*stripe), y, *pixel);
			}
		}
	}
	//apply pixel shift to generate final image

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
	image::save_buffer("square_triangle_border.png", &canvas, 640, 640, image::ExtendedColorType::Rgba8).unwrap();
}