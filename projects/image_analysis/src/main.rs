extern crate image;

use std::fs::File;

use image::{
    GenericImage
};

fn main() {
	println!("trying to open an image");
	let img = image::open("src/resources/tizian.jpg").expect("Image was not opened");
	let filtered = img.fliph();
	let mut out = File::create("out.png").unwrap();
	let _ = filtered.save(&mut out, image::PNG).expect("Saving image failed");
    println!("dimensions {:?}", img.dimensions());
    println!("printing imgage pixel value {:?}", img.get_pixel(100, 100));
}
