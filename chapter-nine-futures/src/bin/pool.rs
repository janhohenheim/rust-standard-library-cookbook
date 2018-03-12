extern crate futures;

use futures::prelude::*;
use futures::task::Context;
use futures::channel::oneshot;
use futures::future::{
    FutureResult,
    lazy,
    ok,
};
use futures::executor::{
    Executor,
    LocalPool,
    ThreadPool,
    ThreadPoolBuilder,
};

use std::cell::Cell;
use std::rc::Rc;
use std::sync::mpsc;

#[derive(Clone, Copy, Debug)]
enum Status {
  Loading,
  Loaded,
}

#[derive(Clone, Copy, Debug)]
struct Container {
    name: &'static str,
    status: Status,
    ticks: usize,
}

impl Container {
    fn new(name: &'static str) -> Self {
        Container {
            name: name,
            status: Status::Loading,
            ticks: 10,
        }
    }
}

impl Future for Container {
    type Item = Container;
    type Error = Never;

    fn poll(&mut self, _cx: &mut Context) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready(*self))
    }
}

const FINISHED: Result<(), Never> = Ok(());

fn new_status(unit: &'static str, status: Status) {
    println!("{}: new status: {:?}", unit, status);
}

fn local_until() {
    let mut container = Container::new("acme");

    let mut pool = LocalPool::new();
    let mut exec = pool.executor();

    let f = lazy(move || -> FutureResult<Container, Never> {
        container.status = Status::Loaded;
        ok(container)
    });

    container = pool.run_until(f, &mut exec).unwrap();
    new_status("local_until", container.status);
}

fn local_spawns_completed() {
    let (tx, rx) = oneshot::channel();
    let mut container = Container::new("acme");

    let mut pool = LocalPool::new();
    let mut exec = pool.executor();

    exec.spawn_local(lazy(move || {
        container.status = Status::Loaded;
        tx.send(container).unwrap();
        FINISHED
    })).unwrap();

    container = pool.run_until(rx, &mut exec).unwrap();
    new_status("local_spanws_completed", container.status);
}

fn local_nested() {
    let mut container = Container::new("acme");
    let cnt = Rc::new(Cell::new(container));
    let cnt_2 = cnt.clone();

    let mut pool = LocalPool::new();
    let mut exec = pool.executor();
    let mut exec_2 = pool.executor();

    let _ = exec.spawn_local(lazy(move || {
        exec_2.spawn_local(lazy(move || {
            let mut container = cnt_2.get();
            container.status = Status::Loaded;
            cnt_2.set(container);
            FINISHED
        })).unwrap();
        FINISHED
    }));

    let _ = pool.run(&mut exec);

    container = cnt.get();
    new_status("local_nested", container.status);
}

fn thread_pool() {
    let (tx, rx) = mpsc::sync_channel(2);
    let tx_2 = tx.clone();
    let mut thread_pool = ThreadPool::new();

    let _ = thread_pool.spawn(Box::new(lazy(move || {
        tx.send(1).unwrap();
        FINISHED
    })));

    let f = lazy(move || {
        tx_2.send(1).unwrap();
        FINISHED
    });

    let _ = thread_pool.run(f);

    let cnt = rx.into_iter().count();
    println!("Count should be 2: {:?}", cnt);
}

fn thread_pool_2_cpus() {
    let (tx, rx) = mpsc::sync_channel(2);
    let tx_2 = tx.clone();

    let mut cpu_pool = ThreadPoolBuilder::new()
        .pool_size(2).create();

    let _ = cpu_pool.spawn(Box::new(lazy(move || {
        tx.send(1).unwrap();
        FINISHED
    })));

    let f = lazy(move || {
        tx_2.send(1).unwrap();
        FINISHED
    });

    let _ = cpu_pool.run(f);

    let cnt = rx.into_iter().count();
    println!("Count should be 2: {:?}", cnt);
}

fn main() {
    println!("local_until():");
    local_until();

    println!("\nlocal_spawns_completed():");
    local_spawns_completed();

    println!("\nlocal_nested():");
    local_nested();

    println!("\nthread_pool():");
    thread_pool();

    println!("\nthread_pool_2_cpus():");
    thread_pool_2_cpus();
}
