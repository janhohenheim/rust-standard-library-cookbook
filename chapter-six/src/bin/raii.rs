use std::ops::Deref;

struct Mutex<T> {
    // We keep a reference to our data: T here.
    //...
    data: T,
}

struct MutexGuard<'a, T: 'a> {
    lock: &'a Mutex<T>,
    //...
}

// Locking the mutex is explicit.
impl<T> Mutex<T> {
    fn new(data: T) -> Self {
        Mutex { data }
    }
    fn lock(&self) -> MutexGuard<T> {
        // Lock the underlying OS mutex.
        //...

        // MutexGuard keeps a reference to self
        MutexGuard { lock: self }
    }
}

// Destructor for unlocking the mutex.
impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // Unlock the underlying OS mutex.
        //...
    }
}

// Implementing Deref means we can treat MutexGuard like a pointer to T.
impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.lock.data
    }
}

fn main() {
    let foo = Mutex::new("Hello World");
    let bar = foo.lock();
    // len is a method on str.
    let len = bar.len();
    // The borrow checker ensures we can't store a reference to the underlying
    // Foo which will outlive the guard xx.

    // x is unlocked when we exit this function and xx's destructor is executed.
}
