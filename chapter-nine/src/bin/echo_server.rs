extern crate hyper;

use hyper::{Method, StatusCode};
use hyper::server::{const_service, service_fn, Http, Request, Response};
use hyper::header::{ContentLength, ContentType};
use std::net::SocketAddr;
fn main() {
    let addr = "[::1]:3000".parse().expect("Failed to parse address");
    run_echo_server(&addr).expect("Failed to run webserver");
}

fn run_echo_server(addr: &SocketAddr) -> Result<(), hyper::Error> {
    let echo = const_service(service_fn(|req: Request| {
        let mut response: Response<hyper::Body> = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                const ERR_MSG: &str = "Try doing a POST at /echo";
                {
                    let headers = response.headers_mut();
                    headers.set(ContentType::plaintext());
                    headers.set(ContentLength(ERR_MSG.len() as u64));
                }
                response.set_body(ERR_MSG);
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

    let server = Http::new().bind(&addr, echo)?;
    server.run()
}
