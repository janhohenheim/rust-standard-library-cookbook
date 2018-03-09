extern crate hyper;

use hyper::{Method, StatusCode};
use hyper::server::{const_service, service_fn, Http, Request, Response};
use hyper::header::{ContentLength, ContentType};

fn main() {
    let addr = "[::1]:3000".parse().expect("Failed to parse address");
    let echo = const_service(service_fn(|req: Request| {
        Ok(match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                const ERR_MSG: &str = "Try doing a POST at /echo";
                Response::<hyper::Body>::new()
                    .with_header(ContentType::plaintext())
                    .with_header(ContentLength(ERR_MSG.len() as u64))
                    .with_body(ERR_MSG)
            }
            (&Method::Post, "/echo") => Response::<hyper::Body>::new().with_body(req.body()),
            _ => Response::<hyper::Body>::new().with_status(StatusCode::NotFound),
        })
    }));
    let server = Http::new().bind(&addr, echo).unwrap();
    server.run().unwrap();
}
