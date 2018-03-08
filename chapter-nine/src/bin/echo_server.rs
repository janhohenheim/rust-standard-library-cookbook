extern crate hyper;

use hyper::{Method, StatusCode};
use hyper::server::{Http, Request, Response};

use hyper::server::{const_service, service_fn};

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let echo = const_service(service_fn(|req: Request| {
        let mut response: Response<hyper::Body> = Response::new();

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

        Ok(response)
    }));
    let server = Http::new().bind(&addr, echo).unwrap();
    server.run().unwrap();
}
