// use rand::Rng;

fn main() {
	let imgx = 200;
	let imgy = 200;

	let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
	// let mut rng = rand::thread_rng();
	for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
		let r = (0.3*x as f32) as u8;
		let b = (0.3*y as f32) as u8;
		*pixel = image::Rgb([r, 0, b]);
	}
	imgbuf.save("first_test.png").unwrap();
}
