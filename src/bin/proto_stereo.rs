/*
redoing the existing code but with lessons from:
http://www.techmind.org/stereo/stech.html
aim:
multiple depth, curve if possible
make the deformation go from center to border,
instead of one side to the other
 */
use image::{imageops, DynamicImage, Pixel, RgbaImage, ExtendedColorType::Rgba8};
use rand::Rng;
use imageproc::{drawing::{self, draw_line_segment_mut, Blend, Canvas}, pixelops::interpolate, point::Point, rect::Rect};

pub fn main()
{
	const MAXWIDTH : usize = 1920;
	let xdpi = 75; //x resolution of picture to be created
	let ydpi = 75;
	let mut last_linked:i32;

	let width=800;//width of target pic
	let height=600;//height of target pic

	let mut look_l = [0;MAXWIDTH];//table of corresponding pixel from right  to left
	let mut look_r = [0;MAXWIDTH];
	let mut vis:bool;
	let mut color_t = vec![image::Rgba([255, 0, 0, 255]); MAXWIDTH];//store colors of those pixels, for lookup later when 2 pixels are linked
	let obs_dist = xdpi * 12;//distance between the observer and the picture
	let eye_sep = 200;
	let max_depth = obs_dist as f32;
	let max_sep = 100;
	// eprintln!("max_sep = {}", max_sep);
	let min_depth = (0.55 * max_depth*obs_dist as f32)/((1.0-0.55) * max_depth + obs_dist as f32);
	let depth_map = image::open("assets/box.jpg").unwrap();
	let pattern = image::open("assets/images/rect.png").unwrap();
	let mut canvas = image::ImageBuffer::new(width as u32, height as u32);
	for frame in 0..=100 {
		for y in 0..height {
			for x in 0..width {//link all pixels with itself at first
				look_l[x] = x; look_r[x] = x;
			}
			for x in 0..width {//link all pixels that are parts of the stereogram effect in pairs
				let object_z =(max_depth - get_z_value(x, y, &depth_map) as f32*(max_depth-min_depth)/256.0) as u32;//get the z depth of selected pixel in depth map reference picture

				let sep = ((eye_sep * object_z) / (object_z + obs_dist))as usize;
				let left : i32 = x as i32 -sep as i32/2;
				let right = left + sep as i32;
				vis = true;
				if left >= 0 && right < width  as i32{
					if look_l[right as usize] != right as usize{
						if look_l[right as usize] < left as usize {
							look_r[look_l[right as usize]] = look_l[right as usize];
							look_l[right as usize] = right as usize;
						}
						else {
							vis = false;
						}
					}
					if look_r[left as usize] != left  as usize{
						if look_r[left as usize] > right  as usize{
							look_l[look_r[left as usize]] = look_r[left as usize];
							look_r[left as usize] = left as usize;
						} else {
							vis = false;
						}
					}
					if vis == true {
						look_l[right as usize] = left as usize;
						look_r[left as usize] = right as usize;
					}
				}
			}
			last_linked = -10;
			for x in 0..width {//assign propoer color to pixels in current line
				if look_l[x] == x {
					if last_linked == x as i32 -1 {
						color_t[x] = color_t[x -1];
					} else {
						color_t[x] = pattern.get_pixel(((x + frame)%100) as u32, y as u32);//get new pixel in the pattern
					}
				} else {
					color_t[x] = color_t[look_l[x]];//pixel is linked with another one, so it take the same color
					last_linked = x as i32;
				}
			}
			for x in 0..width {//set pixel in current picture
				canvas.put_pixel(x as u32, y as u32, color_t[x]);
			}
		}
		let path = format!("frames/proto_stereo_{}.png", frame);
		image::save_buffer(path, &canvas, width as u32, height as u32, image::ExtendedColorType::Rgba8).unwrap();
	}
}

/*function to get depth value from a depthmap pic
will need to take into account the dimension of picture created to keep proportion correct
 */
pub fn get_z_value(x:usize, y:usize, depth_map:&DynamicImage) -> u32 //return a value between 0 & 255, depending on brightness
{

	let pixel = depth_map.get_pixel(x as u32, y as u32);
	let color = pixel.to_luma();
	color[0] as u32
}