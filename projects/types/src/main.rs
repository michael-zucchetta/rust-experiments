fn main() {
    let a: i32 = 23;
    let b = 23;
    println!("ciao {}", a == b);

    let mut a1 = "ciao";
    let mut a2 = "ciao";
    fn plus_one(i: i32) -> i32 {
            i + 1
    }
    println!("are strings comparable? {}, static string? {}", a1 == a2, a1 == "ciao");
    
    
    println!("fun add result: {}", plus_one(2));
    
    let ref_fun: fn(i32) -> i32 = plus_one;

    let b: u32 = 32;
    let a: f32 = 32.0;

    let y = if b == 32 { 1000 } else { -1 };
    println!("print y {}", y);
    let mut myArray: Vec<u32> = vec![3,5,6];

    // doesn't work on normal arrays
    for mut el in myArray {
        el = 2;
        // el is usize
        println!("el {}", el);
    }
   
    // print_vec(myArray);

    // match myArray.get(3) {
    //    Some(x) => println!("any different than an if? {}", x),
    //    None => println!("weird syntax"),
    //}
}

fn print_vec(printArray: Vec<u32>) {
    for mut el in printArray {
        println!("element {}", el);
    }
}
