use std::io::BufReader;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;



fn main() {
    let file = match File::open("src/resources/data.txt") {
        Err(why) => panic!("couldn't open {}: {}", "src/resources/data.txt", Error::description(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line  {
            Ok(line) => println!("{}", line), //if valid(&line) {println!("{}", line) },
            Err(e) => println!("ERROR: {}", e),
        }
    } 
}
