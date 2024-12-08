use image::{Pixel, RgbaImage, imageops};
use rand::Rng;


fn main(){
	let depth_map = image::open("/mnt/nfs/homes/rokerjea/rustereo/assets/images/42_Logo.svg.png").unwrap();
	let xdpi = 300;//probably not 300 on the screen, but 300 on the printer would work

	let width = 1680;
	let height = 1050;

	let mut final_image = RgbaImage::new(width, height);
	let obs_dist = xdpi * 12;
	let eye_sep = xdpi * 2.5;
	let sep_factor = 0.55;
	let max_depth = xdpi * 12;
	let mindepth = (sep_factor * max_depth * obs_dist) / (( 1.0 - sep_factor) *max_depth + obs_dist);
	let look_l = vec[width];

	for y in 0..height{
		for x in 0..width{
			look_l[x] = x;
		}
		for x in 0..width{
			let depth = get_depth(x, y, depth_map);//need to use image of same size, or center/scale it
			let feature_z = max_depth - depth*(max_depth - mindepth) / 255;

			let sep = (eye_sep * depth) / (depth + obs_dist);

			let left = x - sep / 2;
			let right = left + sep;
			if left >= 0 && right < width{
				look_l[right] = left;
			}
		}
		for x in 0..width{
			if look_l[x] == x{
				color[x] = random_color();
			}
			else{
				color[x] = color[look_l[x]];
			}
		}
		for x in 0..width{
			final_image.put_pixel(x, y, color[x]);
			// draw_pixel(x, y, color[x]);
		}
	}
	image::save_buffer("sandbox1.png", &finalimg, width, heigth, image::ExtendedColorType::Rgb8).unwrap();
}


fn random_color() -> Rgba<u8> {
	let mut rng = rand::thread_rng();
	let r = rng.gen_range(0..1);
	if r == 0{
		return Rgba([0, 0, 0, 255]);
	}
	else{
		return Rgba([255, 255, 255, 255]);
	}
}

fn get_depth(x: u32, y: u32, depth_map: &DynamicImage) -> u32 {
	let pixel = depth_map.get_pixel(x, y);
	if pixel != 0{
		return 0;
	}
	return 255;
}

// new framework for future plans:

fn stereogram(){
	//Const values
	let width = 1680;
	let height = 1050;
	let xdpi = 300;//max useful dpi for printer/better screens, etc..

	let obs_dist = xdpi * 12;//observer distance to the screen
	let eye_sep = xdpi * 2.5;//eye separation, mult by number of pixels to get actual separation in pixels
	let sep_factor = 0.55;//the factor by how much the max and min depth can differ
	let max_depth = xdpi * 12;//max depth of the image, same as observer distance
	let mindepth = (sep_factor * (max_depth * obs_dist) as f32) / (( 1.0 - sep_factor) *max_depth as f32 + obs_dist as f32);//min depth of the image
	let max_sep = (eye_sep * max_depth) / (max_depth + obs_dist);//minimum width of the pattern that need to be repeated
	//Load the depth map
	let depth_map = image::open("/mnt/nfs/homes/rokerjea/rustereo/assets/images/42_Logo.svg.png").unwrap();//path can be changed
	let look_l = vec![width];//vector to link the left and right eye pixels
	let look_r = vec![width];//vector to link the right and left eye pixels

	//create the stripe pattern that will be used to create the stereogram
	//TODO: create the stripe pattern
	//let stripe_pattern = create_stripe_pattern(width, height, max_sep);

	//create the final image

	let mut final_image = RgbaImage::new(width, height);
	for y in 0..height{
		for x in 0..width{
			look_l[x] = x;
		}
		for x in 0..width{
			let point_depth = get_depth(x, y, depth_map);//need to use image of same size, or center/scale it before using

			let feature_z = max_depth - point_depth*(max_depth - mindepth) / 255;//depth of that point to the screen
			let sep = (eye_sep * feature_z) / (feature_z + obs_dist);//separation of the eyes ray whwn they touch the screen(cf shema in techmind site)
			let left = x - sep / 2;
			let right = left + sep;
			if left >= 0 && right < width{
				look_l[right] = left;
			}
		}
		for x in 0..width{
			//if pixel is linked to itself, value = pixel in stripe pattern
			//else
			//value = pixel linked to it
		}
		for x in 0..width{
			final_image.put_pixel(x, y, color[x]);
		}
	}
	image::save_buffer("name.png", &finalimg, width, heigth, image::ExtendedColorType::Rgb8).unwrap();
}