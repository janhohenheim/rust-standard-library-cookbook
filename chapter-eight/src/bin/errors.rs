extern crate futures;

use futures::prelude::*;
use futures::executor::block_on;
use futures::stream;
use futures::task::Context;
use futures::future::{
    FutureResult,
    err,
};

struct MyFuture {}
impl MyFuture {
    fn new() -> Self {
        MyFuture {}
    }

    fn map_error_example(self) -> FutureResult<(), &'static str> {
        err::<(), &'static str>("map_error has occurred")
    }

    fn err_into_example(self) -> FutureResult<(), u8> {
        err::<(), u8>(1)
    }

    fn or_else_example(self) -> FutureResult<(), &'static str> {
        err::<(), &'static str>("or_else error has occurred")
    }
}

impl Future for MyFuture {
    type Item = ();
    type Error = Box<&'static str>;

    fn poll(&mut self, _cx: &mut Context) -> Poll<Self::Item, Self::Error> {
        Err(Box::new("A generic error goes here"))
    }
}

struct FuturePanic {}

impl Future for FuturePanic {
    type Item = ();
    type Error = Box<&'static str>;

    fn poll(&mut self, _cx: &mut Context) -> Poll<Self::Item, Self::Error> {
        panic!("It seems like there was a major issue with catch_unwind_example")
    }
}

fn using_recover() {
    let f = MyFuture::new();

    let f_recover = f.recover::<Box<&'static str>, _>(|err| {
        println!("An error has occurred: {}", err);
        ()
    });

    block_on(f_recover).unwrap();
}

fn map_error() {
    let f = MyFuture::new();
    let map_fn = |err| {
        let mut s = String::from(err);
        s.insert_str(0, "map_error_example: ");
        s
    };

    match block_on(f.map_error_example().map_err(map_fn)) {
        Err(e) => {
            println!("block_on error: {}", e)
        }
        _ => {},
    }
}

fn err_into() {
    let f = MyFuture::new();

    match block_on(f.err_into_example().err_into::<u32>()) {
        Err(e) => {
            println!("block_on error: {:?}", e)
        }
        _ => {},
    }
}

fn or_else() {
    let f = MyFuture::new();

    match block_on(f.or_else_example().or_else(|_| -> Result<_, &'static str> {
        Err("changed or_else's error message")
    })) {
        Err(e) => {
            println!("block_on error: {}", e)
        },
        _ => {},
    }
}

fn catch_unwind() {
    let f = FuturePanic {};

    match block_on(f.catch_unwind()) {
        Err(e) => {
            let err: Box<&'static str> = e.downcast().unwrap();
            println!("block_on error: {:?}", err)
        },
        _ => {},
    }
}

fn stream_panics() {
    let stream_ok = stream::iter_ok::<_, bool>(vec![Some(1), Some(7), None, Some(20)]);
    // We should panic on the "None" value since we're explicitly calling for Ok
    // values from stream::iter_ok
    let stream_map = stream_ok.map(|o| o.unwrap());

    // We can use catch_unwind() for catching panics
    let stream = stream_map.catch_unwind().then(|r| Ok::<_, ()>(r));
    let stream_results = block_on(stream.collect()).unwrap();

    // Here we can use the partition() function to separate the Ok and Err values
    let (oks, errs): (Vec<_>, Vec<_>) = stream_results.into_iter().partition(Result::is_ok);
    let ok_values: Vec<_> = oks.into_iter().map(Result::unwrap).collect();
    let err_values: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();

    println!("Panic's Ok values: {:?}", ok_values);
    println!("Panuc's Err values: {:?}", err_values);
}

fn main() {
    println!("using_recover():");
    using_recover();

    println!("\nmap_error():");
    map_error();

    println!("\nerr_into():");
    err_into();

    println!("\nor_else():");
    or_else();

    println!("\ncatch_unwind():");
    catch_unwind();

    println!("\nstream_panics():");
    stream_panics();
}
