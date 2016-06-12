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

    // panic!("break everything");
}
