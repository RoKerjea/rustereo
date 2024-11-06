use rand::Rng;

fn main() {
	let imgx = 200;
	let imgy = 200;

	let mut imgbuf1 = image::ImageBuffer::new(imgx, imgy);
	let mut rng = rand::thread_rng();
	for (x, y, pixel) in imgbuf1.enumerate_pixels_mut() {
		let color = rng.gen_range(0..=1);
			*pixel = image::Rgb([255 * color, 255* color, 255* color]);

	}
	let mut finalimg = image::ImageBuffer::new(400, 200);
	for (x, y, pixel) in imgbuf1.enumerate_pixels_mut() {
		finalimg.put_pixel(x, y, *pixel);
		finalimg.put_pixel(x+200, y, *pixel);
	}
	for x in 275..325 {
		for y in 75..79 {
			let pixel = imgbuf1.get_pixel(x-4-200, y);
			finalimg.put_pixel(x, y, image::Rgb([255 , 255, 255]));
		}
	}
	for x in 275..325 {
		for y in 125..129 {
			let pixel = imgbuf1.get_pixel(x-4-200, y);
			finalimg.put_pixel(x, y, image::Rgb([255 , 255, 255]));
		}
	}
	for x in 275..279 {
		for y in 79..125 {
			let pixel = imgbuf1.get_pixel(x-4-200, y);
			finalimg.put_pixel(x, y, image::Rgb([255 , 255, 255]));
		}
	}
	for x in 321..325 {
		for y in 79..125 {
			let pixel = imgbuf1.get_pixel(x-4-200, y);
			finalimg.put_pixel(x, y, image::Rgb([255 , 255, 255]));
		}
	}
	// imgbuf.save("first_test.png").unwrap();
	image::save_buffer("image.png", &imgbuf1, 200, 200, image::ExtendedColorType::Rgb8).unwrap();
	image::save_buffer("image2.png", &finalimg, 400, 200, image::ExtendedColorType::Rgb8).unwrap()
}
