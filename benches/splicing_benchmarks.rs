use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use slicing_perf::{splice, splice_parallel, splice_stepped};

fn simple_benchmark(c: &mut Criterion) {
    c.bench_function("splice(4, [0..=6])", |b| {
        b.iter(|| splice(black_box(4), black_box(&[0, 1, 2, 3, 4, 5, 6])))
    });
    c.bench_function("splice_stepped(4, [0..=6])", |b| {
        b.iter(|| splice_stepped(black_box(4), black_box(&[0, 1, 2, 3, 4, 5, 6])))
    });
    c.bench_function("splice_parallel(4, [0..=6])", |b| {
        b.iter(|| splice_parallel(black_box(4), black_box(&[0, 1, 2, 3, 4, 5, 6])))
    });
}

fn throughput_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Throughput");

    // try sizes up to 2^26 = 64MB
    for size in generate_sizes(27) {
        let input: Vec<u8> = (0..size).map(|i| i as u8).collect();
        group.throughput(criterion::Throughput::Elements(size as u64));

        group.bench_with_input(BenchmarkId::new("splice", size), &input, |b, i| {
            b.iter(|| splice(black_box(5), i))
        });
        group.bench_with_input(BenchmarkId::new("splice_stepped", size), &input, |b, i| {
            b.iter(|| splice_stepped(black_box(5), i))
        });
        group.bench_with_input(BenchmarkId::new("splice_parallel", size), &input, |b, i| {
            b.iter(|| splice_parallel(black_box(5), i))
        });
    }
}

fn throughput_benchmark_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("Throughput small");

    // try sizes up to 1KB
    for size in generate_sizes(11) {
        let input: Vec<u8> = (0..size).map(|i| i as u8).collect();
        group.throughput(criterion::Throughput::Elements(size as u64));

        group.bench_with_input(BenchmarkId::new("splice", size), &input, |b, i| {
            b.iter(|| splice(black_box(5), i))
        });
        group.bench_with_input(BenchmarkId::new("splice_stepped", size), &input, |b, i| {
            b.iter(|| splice_stepped(black_box(5), i))
        });
    }
}


fn generate_sizes(pow2: usize) -> Vec<usize> {
    std::iter::successors(Some(1), |&i| Some(i * 2))
        .take(pow2)
        .collect()
}

criterion_group!(benches, simple_benchmark, throughput_benchmark, throughput_benchmark_small);
criterion_main!(benches);
