extern crate hyper;

use hyper::header::{ContentLength, ContentType};
use hyper::server::{const_service, service_fn, Http, Response};
use std::net::Ipv6Addr;

fn main() {
    run().expect("Failed to run webserver");
}

fn run() -> Result<(), hyper::Error> {
    let localhost = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    let port = 3000;
    let addr = (localhost, port).into();

    let hello_world = const_service(service_fn(|req| {
        println!("Got a connection!");
        static BODY: &'static str = "Hello, World!";
        Ok(Response::<hyper::Body>::new()
            .with_header(ContentLength(BODY.len() as u64))
            .with_header(ContentType::plaintext())
            .with_body(BODY))
    }));

    let server = Http::new().bind(&addr, hello_world)?;
    server.run()
}
