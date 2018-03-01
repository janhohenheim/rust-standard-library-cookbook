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
    fn inner(n: u32, penultimate: u32, last: u32) -> u32 {
        match n {
            0 => penultimate,
            1 => last,
            _ => inner(n - 1, last, penultimate + last),
        }
    }
    inner(n, 0, 1)
}

pub fn fibonacci_imperative(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut penultimate;
            let mut last = 1;
            let mut fib = 0;
            for _ in 0..n {
                penultimate = last;
                last = fib;
                fib = penultimate + last;
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