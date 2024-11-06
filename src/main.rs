use rand::Rng;

fn main() {
	let imgx = 100;
	let imgy = 200;

	let mut imgbuf1 = image::ImageBuffer::new(imgx, imgy);
	let mut rng = rand::thread_rng();
	for (x, y, pixel) in imgbuf1.enumerate_pixels_mut() {
		let color = rng.gen_range(0..=1);
			*pixel = image::Rgb([255 * color, 255* color, 255* color]);

	}
	let mut finalimg = image::ImageBuffer::new(200, 200);
	for (x, y, pixel) in imgbuf1.enumerate_pixels_mut() {
		finalimg.put_pixel(x, y, *pixel);
		finalimg.put_pixel(x+100, y, *pixel);
	}
	for x in 120..175 {
		for y in 25..29 {
			let pixel = imgbuf1.get_pixel(x+2-100, y);
			finalimg.put_pixel(x, y, *pixel);
		}
	}
	for x in 120..175 {
		for y in 175..179 {
			let pixel = imgbuf1.get_pixel(x+2-100, y);
			finalimg.put_pixel(x, y, *pixel);
		}
	}
	for x in 120..124 {
		for y in 29..175 {
			let pixel = imgbuf1.get_pixel(x+2-100, y);
			finalimg.put_pixel(x, y, *pixel);
		}
	}
	for x in 171..175 {
		for y in 29..175 {
			let pixel = imgbuf1.get_pixel(x+2-100, y);
			finalimg.put_pixel(x, y, *pixel);
		}
	}
	// imgbuf.save("first_test.png").unwrap();
	image::save_buffer("image.png", &imgbuf1, 100, 200, image::ExtendedColorType::Rgb8).unwrap();
	image::save_buffer("image2.png", &finalimg, 200, 200, image::ExtendedColorType::Rgb8).unwrap()
}
