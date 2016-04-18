extern crate hyper;

use hyper::*;
use std::io::Read;

fn main() {
    let client = Client::new();
    let res = client.post("http://localhost:3000/set").body(r#"{ "message": "What comes around goes around." }"#).send().unwrap();
    assert_eq!(res.status, hyper::status::StatusCode::Created);

    let mut res = client.get("http://localhost:3000/").send().unwrap();
    assert_eq!(res.status, hyper::status::StatusCode::Ok);

    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
