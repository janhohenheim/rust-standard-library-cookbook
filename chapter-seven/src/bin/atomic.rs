use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, ATOMIC_BOOL_INIT, ATOMIC_USIZE_INIT};
use std::thread;
use std::ops::{Deref, DerefMut};
use std::cell::UnsafeCell;

// NaiveMutex is an easy, albeit very suboptimal,
// implementation of a Mutex, similar to std::sync::Mutex
pub struct NaiveMutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct NaiveMutexGuard<'a, T: 'a> {
    naive_mutex: &'a NaiveMutex<T>,
}

impl<T> NaiveMutex<T> {
    pub fn new(data: T) -> Self {
        NaiveMutex {
            locked: ATOMIC_BOOL_INIT,
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> NaiveMutexGuard<T> {
        // The following algorithm is called a "spinlock", because it keeps
        // the current thread blocked by doing nothing (it keeps it "spinning")
        while self.locked.compare_and_swap(false, true, Ordering::SeqCst) {}
        NaiveMutexGuard { naive_mutex: self }
    }
}

unsafe impl<T: Send> Sync for NaiveMutex<T> {}

// Automatically unlock the underlying resource on drop
impl<'a, T> Drop for NaiveMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.naive_mutex.locked.store(false, Ordering::SeqCst);
    }
}

impl<'a, T> Deref for NaiveMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &mut *self.naive_mutex.data.get() }
    }
}

impl<'a, T> DerefMut for NaiveMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.naive_mutex.data.get() }
    }
}

unsafe impl<'a, T: Sync> Sync for NaiveMutexGuard<'a, T> {}

fn main() {
    let naive_mutex = Arc::new(NaiveMutex::new(1));
    let updater = {
        let naive_mutex = naive_mutex.clone();
        thread::spawn(move || {
            let mut val = naive_mutex.lock();
            *val = 2;
        })
    };
    let printer = {
        let naive_mutex = naive_mutex.clone();
        thread::spawn(move || {
            let val = naive_mutex.lock();
            println!("The value in the naive mutex is: {}", *val);
        })
    };
    updater.join().expect("The updater thread panicked");
    printer.join().expect("The printer thread panicked");
}
