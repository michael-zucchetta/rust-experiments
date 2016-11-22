// necessary for args
use std::env;

fn get_option_val(input: Option<u32>) -> u32 {
    match input {
        Some(value) => value,
        None => 0,
    }
}

fn test_func() -> u32 {
    2
}

fn main() {
    let x: Option<u32> = Some(2);
    let y: Option<u32> = None;
    let option_x = get_option_val(x);
    println!("Option is {} vs None {}", option_x, get_option_val(y));
    // Prints each argument on a separate line
    for argument in env::args() {
        println!("{}", argument);
    }
}
