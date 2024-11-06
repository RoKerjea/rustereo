use image::{GenericImageView, Pixel};
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
	let input = image::open("42_Logo.svg.png").unwrap();

	for (x, y, pixel) in imgbuf1.enumerate_pixels_mut() {
		finalimg.put_pixel(x, y, *pixel);
	}
	for stripe in 1..8 {
		for x in 0..80 {
			for y in 0..640 {
				let mut pixel = finalimg.get_pixel(x, y);
				if y > 80 && y < 560 && (x+(80*(stripe-1)) < 480) && (stripe == 1 || stripe == 2 || stripe == 3)
				{
					let inputpixel = input.get_pixel(x+(80*(stripe-1)), y-80);
					if inputpixel == image::Rgba([0, 0, 0, 255]) {
						pixel = finalimg.get_pixel(x+2+(80*(stripe-1)), y);
						// pixel = &image::Rgb([255, 255, 255]);
					}
				}
				finalimg.put_pixel(x+(80*stripe), y, *pixel);
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
	image::save_buffer("image2.png", &finalimg, 640, 640, image::ExtendedColorType::Rgb8).unwrap()
}
