use std::sync::Arc;
use std::sync::atomic::{fence, AtomicBool, AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::thread;

static GLOBAL_THREAD_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

fn main() {
    let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
    println!("live threads: {}", old_thread_count + 1);
}

fn spinlock() {
    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = spinlock.clone();
    let thread = thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // Wait for the other thread to release the lock
    while spinlock.load(Ordering::SeqCst) != 0 {
        println!("Waiting for the other thread to set the counter to zero");
    }

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }

    use std::sync::atomic::{AtomicUsize, Ordering};
}

// A mutual exclusion primitive based on spinlock.
pub struct Mutex {
    flag: AtomicBool,
}

impl Mutex {
    pub fn new() -> Mutex {
        Mutex {
            flag: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while !self.flag.compare_and_swap(false, true, Ordering::Relaxed) {}
        // This fence synchronizes-with store in `unlock`.
        fence(Ordering::Acquire);
    }

    pub fn unlock(&self) {
        self.flag.store(false, Ordering::Release);
    }
}

// To do maybe implement Peterson lock
