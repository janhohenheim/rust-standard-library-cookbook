#![feature(test)]
extern crate test;

pub fn fibonacci_recursive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
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
    fn bench_fibonacci_recursive(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(20);
            fibonacci_recursive(n)
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