extern crate futures;
extern crate hyper;

use hyper::{Method, StatusCode};
use hyper::server::{const_service, service_fn, Http, Request, Response};
use hyper::header::{ContentLength, ContentType};
use hyper::mime;
use futures::Future;
use futures::sync::oneshot;
use std::net::SocketAddr;
use std::thread;
use std::fs::File;
use std::io::{self, copy};

fn main() {
    let addr = "[::1]:3000".parse().expect("Failed to parse address");
    run_echo_server(&addr).expect("Failed to run webserver");
}

fn run_echo_server(addr: &SocketAddr) -> Result<(), hyper::Error> {
    let echo = const_service(service_fn(|req: Request| {
        match (req.method(), req.path()) {
            (&Method::Get, "/") => handle_root(),
            (&Method::Get, path) => handle_get_file(path),
            _ => handle_invalid_method(),
        }
    }));

    let server = Http::new().bind(addr, echo)?;
    server.run()
}

type ResponseFuture = Box<Future<Item = Response, Error = hyper::Error>>;
fn handle_root() -> ResponseFuture {
    send_file_or_404("index.html")
}

fn handle_get_file(file: &str) -> ResponseFuture {
    send_file_or_404(file)
}

fn handle_invalid_method() -> ResponseFuture {
    let response_future = send_file_or_404("invalid_method.html")
        .and_then(|response| Ok(response.with_status(StatusCode::MethodNotAllowed)));
    Box::new(response_future)
}

fn send_file_or_404(path: &str) -> ResponseFuture {
    let path = sanitize_path(path);
    let response_future = try_to_send_file(&path)
        .and_then(|response_result| response_result.map_err(|error| error.into()))
        .or_else(|_| send_404());
    Box::new(response_future)
}

fn sanitize_path(path: &str) -> String {
    path.replace("\\", "/")
        .replace("../", "")
        .trim_left_matches(|c| c == '/')
        .trim_right_matches(|c| c == '/')
        .to_string()
}

type ResponseResultFuture = Box<Future<Item = Result<Response, io::Error>, Error = hyper::Error>>;
fn try_to_send_file(path: &str) -> ResponseResultFuture {
    let path = path_on_disk(path);
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to find file: {}", path);
                tx.send(Err(err)).expect("Send error on file not found");
                return;
            }
        };
        let mut buf: Vec<u8> = Vec::new();
        match copy(&mut file, &mut buf) {
            Ok(_) => {
                println!("Sending file: {}", path);
                let content_type = get_content_type(&path).unwrap_or(ContentType::plaintext());
                let res = Response::new()
                    .with_header(ContentLength(buf.len() as u64))
                    .with_header(content_type)
                    .with_body(buf);
                tx.send(Ok(res))
                    .expect("Send error on successful file read");
            }
            Err(err) => {
                tx.send(Err(err)).expect("Send error on error reading file");
            }
        };
    });
    Box::new(rx.map_err(|error| io::Error::new(io::ErrorKind::Other, error).into()))
}

fn path_on_disk(path_to_file: &str) -> String {
    "files/".to_string() + path_to_file
}

fn send_404() -> ResponseFuture {
    let response_future = try_to_send_file("not_found.html").and_then(|response_result| {
        Ok(response_result.unwrap_or_else(|_| {
            const ERROR_MSG: &str = "Failed to find \"File not found\" page. How ironic\n";
            Response::new()
                .with_status(StatusCode::NotFound)
                .with_header(ContentLength(ERROR_MSG.len() as u64))
                .with_body(ERROR_MSG)
        }))
    });
    Box::new(response_future)
}

fn get_content_type(file: &str) -> Option<ContentType> {
    let pos = file.rfind('.')? + 1;
    let mime_type = match &file[pos..] {
        "txt" => mime::TEXT_PLAIN_UTF_8,
        "html" => mime::TEXT_HTML_UTF_8,
        "css" => mime::TEXT_CSS,
        _ => return None,
    };
    Some(ContentType(mime_type))
}
