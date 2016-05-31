
fn main() {
        let a = add_one;
        print_sum(add_one(5), a(6));
        

        let mut y = 5;
        let x = (y = 6);
        // gives an error on x println!("checking expressions in Rust {}, {}", y, x);    

        let two_hearts = '💕';
        println!("is it really printing this char? {}", two_hearts);  
}

fn add_one(x: i32) -> i32 {
        x + 1
}

// the equivalent of the function above with a semicolon would create an error
// fn add_one(x: i32) -> i32 {
//         x + 1;
// }

fn print_sum(x: i32, y: i32) {
        println!("sum is: {}", x + y);
}
