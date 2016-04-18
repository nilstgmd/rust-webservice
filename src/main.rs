extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use std::sync::{Mutex, Arc};

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    message: String
}

fn main() {
    let greeting = Arc::new(Mutex::new(Greeting { message: "Hello World".to_string()}));
    let greeting_clone = greeting.clone();

    let mut router = Router::new();

    router.get("/", move |r: &mut Request | hello_world(r, &greeting.lock().unwrap()));
    router.post("/set", move |r: &mut Request | set_greeting(r, &mut greeting_clone.lock().unwrap()));

    fn hello_world(_: &mut Request, greeting: &Greeting) -> IronResult<Response> {
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    fn set_greeting(request: &mut Request, greeting: &mut Greeting) -> IronResult<Response> {
        let mut request_payload = String::new();
        request.body.read_to_string(&mut request_payload).unwrap();
        *greeting = json::decode(&request_payload).unwrap();

        Ok(Response::with((status::Created)))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
