use image::Pixel;
use rand::Rng;

fn main() {
	let imgx = 80;
	let imgy = 640;

	let mut imgbuf1 = image::ImageBuffer::new(imgx, imgy);
	let mut rng = rand::thread_rng();
	for (_x, _y, pixel) in imgbuf1.enumerate_pixels_mut() {
		let color = rng.gen_range(0..=1);
			*pixel = image::Rgb([255 * color, 255* color, 255* color]);

	}
	let mut finalimg = image::ImageBuffer::new(640, 640);
	for stripe in 0..8 {
		for (x, y, pixel) in imgbuf1.enumerate_pixels_mut() {
			finalimg.put_pixel(x+(80*stripe), y, *pixel);
		}
	}
	let input = image::open("42_Logo.svg.png").unwrap();
	for (x, y, pixel) in input.to_rgba8().enumerate_pixels() {
		if *pixel == image::Rgba([0, 0, 0, 255]) {
			finalimg.put_pixel(x+80, y+80, image::Rgb([255, 255, 255]));
		}
	}
	// imgbuf.save("first_test.png").unwrap();
	// image::save_buffer("image.png", &imgbuf1, 80, 200, image::ExtendedColorType::Rgb8).unwrap();
	image::save_buffer("image2.png", &finalimg, 640, 640, image::ExtendedColorType::Rgb8).unwrap()
}
