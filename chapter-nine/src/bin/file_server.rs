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
        match (req.method(), req.path()) {
            (&Method::Get, "/") => handle_root(req),
            (&Method::Get, _) => handle_get_file(req),
            _ => handle_invalid_method(req),
        }
    }));

    let server = Http::new().bind(addr, echo)?;
    server.run()
}

type ResponseFuture = Box<Future<Item = Response, Error = hyper::Error>>;
fn handle_root(_: Request) -> ResponseFuture {
    send_html("index.html")
}

fn handle_get_file(req: Request) -> ResponseFuture {
    send_html(req.path())
}

fn handle_invalid_method(_: Request) -> ResponseFuture {
    let response_future = send_html("invalid_method.html")
        .and_then(|response| Ok(response.with_status(StatusCode::MethodNotAllowed)));
    Box::new(response_future)
}

fn send_html(path: &str) -> Box<Future<Item = Response, Error = hyper::Error>> {
    let path = path_on_disk(path);
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                const ERROR_MSG: &[u8] = b"Error 404 (File Not Found)";
                tx.send(
                    Response::new()
                        .with_status(StatusCode::NotFound)
                        .with_header(ContentLength(ERROR_MSG.len() as u64))
                        .with_body(ERROR_MSG),
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
    Box::new(rx.map_err(|error| io::Error::new(io::ErrorKind::Other, error).into()))
}

fn path_on_disk(path_to_file: &str) -> String {
    "files/".to_string() + path_to_file
}
