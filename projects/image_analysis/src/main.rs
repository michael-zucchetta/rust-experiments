extern crate image;

use std::fs::File;

fn main() {
	println!("trying to open an image");
	let img = image::open("src/resources/tizian.jpg").expect("Image was not opened");
	let filtered = img.fliph();
	let mut out = File::create("out.png").unwrap();
	let _ = filtered.save(&mut out, image::PNG).expect("Saving image failed");
}
