extern crate iron;

mod utils;

use iron::prelude::*;
use iron::status;

fn main() {
    trait Foo {
        fn method(&self) -> String;
    }
    impl Foo for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }
    
    fn do_something(x: &Foo) {
        x.method();
    }
    
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World! ")))
    }

    let _server = Iron::new(hello_world).http("localhost:3000").unwrap();
    let value = 3;
    let ciao = || {
        2 + value
    };
    let value_string = "something".to_string();
    let x = do_something(&value_string);
    // x prints as () as the value Foo does not implement the Display trait and :? allows it
    println!("On 3000 {} {:?}", ciao() + utils::utils::test(3), x);
}
