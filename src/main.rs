use image::{GenericImageView, Pixel};
use rand::Rng;
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

fn draw_circle(canvas: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, x: u32, y: u32, r: u32, color: image::Rgb<u8>) {
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

fn main() {
	let pixel = image::Rgb([255, 255, 255]);
	let mut canvas = image::ImageBuffer::from_pixel(640, 640, pixel);// can be used to genrate a new canvas with a single color
	let color = image::Rgb([255, 0, 0]);
	let blue = image::Rgb([0, 0, 255]);
	draw_rectangle(&mut canvas, 320, 320, 100, color);
	draw_circle(&mut canvas, 320, 320, 50, blue);
	image::save_buffer("video1.png", &canvas, 640, 640, image::ExtendedColorType::Rgb8).unwrap();
}