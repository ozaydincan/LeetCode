use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use itoa::Buffer;
use std::fmt::Write;

fn stoi_write(s: &str) -> String {
    let mut num_str = String::new();
    for c in s.chars() {
        let value = c as u8 - 96;
        write!(&mut num_str, "{}", value).expect("Write failed");
    }
    num_str
}
fn sum_ascii_values(s: &str) -> u64 {
    s.chars().map(|ch| ch as u64).sum()
}

fn sum_digits(x: u64) -> u64 {
    (x % 10)
        + (0..)
            .scan(x, |num, _| {
                *num /= 10;
                Some(*num)
            })
            .take_while(|num| *num > 0)
            .map(|num| num % 10)
            .sum::<u64>()
}

fn stoi_itoa(s: &str) -> String {
    let mut num_str = String::new();
    let mut buffer = Buffer::new();
    for c in s.chars() {
        let value = c as u8 - 96;
        num_str.push_str(buffer.format(value));
    }
    num_str
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("benchmark_group");

    group.bench_with_input(
        BenchmarkId::new("sum_ascii_values", "abcdefghij"),
        &"abcdefghij",
        |b, s| b.iter(|| sum_ascii_values(black_box(s))),
    );

    group.bench_with_input(BenchmarkId::new("sum_digits", 123456), &123456, |b, &x| {
        b.iter(|| sum_digits(black_box(x)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
