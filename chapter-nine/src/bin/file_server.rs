extern crate futures;
extern crate hyper;

use hyper::{Method, StatusCode};
use hyper::server::{const_service, service_fn, Http, Request, Response};
use hyper::header::{ContentLength, ContentType};
use std::net::SocketAddr;
use std::thread;
use futures::Future;
use futures::sync::oneshot;
use std::fs::File;
use std::io::{self, copy};

fn main() {
    let addr = "[::1]:3000".parse().expect("Failed to parse address");
    run_echo_server(&addr).expect("Failed to run webserver");
}

fn run_echo_server(addr: &SocketAddr) -> Result<(), hyper::Error> {
    let echo = const_service(service_fn(|req: Request| {
        // An easy way to implement routing is
        // to simply match the request's path
        /*
        match (req.method(), req.path()) {
            (&Method::Get, "/") => handle_root(req),
            (&Method::Post, "/echo") => handle_echo(req),
            _ => handle_not_found(req),
        }
        */
        simple_file_send("files/index.html")
    }));

    let server = Http::new().bind(addr, echo)?;
    server.run()
}

type ResponseResult = Result<Response<hyper::Body>, hyper::Error>;
fn handle_root(_: Request) -> ResponseResult {
    const MSG: &str = "Try doing a POST at /echo";
    Ok(Response::new()
        .with_header(ContentType::plaintext())
        .with_header(ContentLength(MSG.len() as u64))
        .with_body(MSG))
}

fn handle_echo(req: Request) -> ResponseResult {
    // The echoing is implemented by setting the response's
    // body to the request's body
    Ok(Response::new().with_body(req.body()))
}

fn handle_not_found(_: Request) -> ResponseResult {
    // Return a 404 for every unsupported route
    Ok(Response::new().with_status(StatusCode::NotFound))
}

fn simple_file_send(f: &str) -> Box<Future<Item = Response, Error = hyper::Error>> {
    let filename = f.to_string(); // we need to copy for lifetime issues
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        let not_found: &[u8] = b"not found";
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                tx.send(
                    Response::new()
                        .with_status(StatusCode::NotFound)
                        .with_header(ContentLength(not_found.len() as u64))
                        .with_body(not_found),
                ).expect("Send error on open");
                return;
            }
        };
        let mut buf: Vec<u8> = Vec::new();
        match copy(&mut file, &mut buf) {
            Ok(_) => {
                let res = Response::new()
                    .with_header(ContentType::html())
                    .with_header(ContentLength(buf.len() as u64))
                    .with_body(buf);
                tx.send(res).expect("Send error on successful file read");
            }
            Err(_) => {
                tx.send(Response::new().with_status(StatusCode::InternalServerError))
                    .expect("Send error on error reading file");
            }
        };
    });
    Box::new(rx.map_err(|e| hyper::Error::from(io::Error::new(io::ErrorKind::Other, e))))
}
