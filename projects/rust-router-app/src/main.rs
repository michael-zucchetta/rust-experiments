extern crate iron;
extern crate rustc_serialize;
extern crate router;

use rustc_serialize::json;
use std::collections::HashMap;

use iron::prelude::*;
use iron::{Handler};
use iron::status;

use router::Router;

struct RouterImpl {
    // Routes here are simply matched with the url path.
    routes: HashMap<String, Box<Handler>>
}

impl RouterImpl {
    fn new() -> Self {
        RouterImpl { routes: HashMap::new() }
    }

    fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for RouterImpl {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound))
        }
    }
}


#[derive(RustcDecodable, RustcEncodable)]
pub struct TestStruct  {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
}

fn main() {
    let mut custom_router = RouterImpl::new();

    custom_router.add_route("hello".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello world !")))
    });   
    
    custom_router.add_route("json".to_string(), |_: &mut Request| {
        let object = TestStruct {
            data_int: 13,
            data_str: "something 123".to_string(),
            data_vector: vec![2,3,4,5, 123 , 3],
        };
        // Serialize using `json::encode`
        let encoded = json::encode(&object).unwrap();
        // Deserialize using `json::decode`
        let decoded: TestStruct = json::decode(&encoded).unwrap();
        Ok(Response::with((status::Ok, encoded)))
    });   

    // Iron::new(custom_router).http("localhost:3000").unwrap();

    let mut router = Router::new();           // Alternative syntax:
    router.get("/", handler, "index");        // let router = router!(index: get "/" => handler,
    router.get("/:query", handler, "query");  //                      query: get "/:query" => handler);
    router.post("/post/:queryPost", handler_post, "queryPost");  //                      query: get "/:query" => handler);

    Iron::new(router).http("localhost:3001").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
    
    fn handler_post(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("queryPost").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
}
