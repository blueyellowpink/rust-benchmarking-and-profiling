use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_benchmarking_and_profiling::{read_csv, read_file};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_optimized(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn bench_fibo(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn bench_fibo_opt(c: &mut Criterion) {
    c.bench_function("fib optimized 20", |b| {
        b.iter(|| fibonacci_optimized(black_box(20)))
    });
}

fn bench_read_csv(c: &mut Criterion) {
    c.bench_function("read csv", |b| {
        b.iter(|| {
            let bytes = read_file(&"test.csv".into()).expect("failed to read file");
            black_box(read_csv(&bytes)).expect("benchmark failure");
        })
    });
}

// criterion_group!(benches, bench_fibo, bench_fibo_opt);
criterion_group!(benches, bench_read_csv);
criterion_main!(benches);
