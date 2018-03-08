extern crate futures;
extern crate hyper;

use futures::future::Future;

use hyper::{Method, StatusCode};
use hyper::server::{Http, Request, Response, Service};

struct Echo;
impl Service for Echo {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                response.set_body("Try POSTing data to /echo");
            }
            (&Method::Post, "/echo") => {
                response.set_body(req.body());
            }
            _ => {
                response.set_status(StatusCode::NotFound);
            }
        };

        Box::new(futures::future::ok(response))
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    server.run().unwrap();
}
