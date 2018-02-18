use std::thread;
use std::sync::Arc;

fn main() {
    let some_resource = Arc::new("Hello World".to_string());
    let thread_a = {
        let some_resource = some_resource.clone();
        thread::spawn(move || {
            println!("Thread A says: {}", some_resource);
        })
    };
    let thread_b = {
        let some_resource = some_resource.clone();
        thread::spawn(move || {
            println!("Thread B says: {}", some_resource);
        })
    };

    thread_a.join().expect("Thread A panicked");
    thread_b.join().expect("Thread B panicked");
}
