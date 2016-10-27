extern crate iron;
extern crate rustc_serialize;
extern crate router;

use std::io::Read;

use iron::prelude::*;
use iron::status;
use router::Router;

use rustc_serialize::json;

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

impl Greeting {
    fn new(msg: String) -> Greeting {
        Greeting {
            msg: msg
        }
    }
}

fn main() {
    let mut router = Router::new();

    router.get("/", hello_world, "hello_world");
    router.post("/set", set_greeting, "set_greeting");

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let greeting = Greeting::new("Hello, World".to_string());
        let payload = json::encode(&greeting).unwrap();

        Ok(Response::with((status::Ok, payload)))
    }

    fn set_greeting(request: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let request: Greeting = json::decode(&payload).unwrap();
        
        let greeting = Greeting::new(request.msg);
        let payload = json::encode(&greeting).unwrap();

        Ok(Response::with((status::Ok, payload)))
    }

    Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}