#![feature(test)]
extern crate test;

pub fn slow_fibonacci_recursive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => slow_fibonacci_recursive(n - 1) + slow_fibonacci_recursive(n - 2)
    }
}

pub fn fast_fibonacci_recursive(n: u32) -> u32 {
    fn inner(n: u32, acc1: u32, acc2: u32) -> u32 {
        match n {
            0 => acc1,
            _ => inner(n - 1, acc2, acc1 + acc2),
        }
    }
    inner(n - 1, 1, 1)
}

pub fn fibonacci_imperative(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut erelast_fib;
            let mut last_fib = 1;
            let mut fib = 0;
            for _ in 0..n {
                erelast_fib = last_fib;
                last_fib = fib;
                fib = erelast_fib + last_fib;
            }
            fib
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_slow_fibonacci_recursive(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(20);
            slow_fibonacci_recursive(n)
        });
    }

    #[bench]
    fn bench_fast_fibonacci_recursive(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(20);
            fast_fibonacci_recursive(n)
        });
    }

    #[bench]
    fn bench_fibonacci_imperative(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(20);
            fibonacci_imperative(n)
        });
    }
}