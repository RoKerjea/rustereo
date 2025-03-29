use apng::{Encoder, Frame, PNGImage};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let mut files = vec![];
	for i in 0..=100 {

		let path = format!("frames/proto_stereo_{}.png", i);
		files.push(path);
	}

    let mut png_images: Vec<PNGImage> = Vec::new();
    for f in files.iter() {
        png_images.push(apng::load_png(f).unwrap());
    }

    let path = Path::new(r"out.png");
    let mut out = BufWriter::new(File::create(path).unwrap());

    let config = apng::create_config(&png_images, None).unwrap();
    let mut encoder = Encoder::new(&mut out, config).unwrap();
    let frame = Frame {
        delay_num: Some(32),
        delay_den: Some(1000),
        ..Default::default()
    };

    match encoder.encode_all(png_images, Some(&frame)) {
        Ok(_n) => println!("success"),
        Err(err) => eprintln!("{}", err),
    }
}