use std::hint::black_box;
use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};
use carr::arr::Arr;

fn carr_small() {
    let arr = Arr::<Vec<i32>, i32, { &[5; 1] }>::new_ones();
    let arr2 = Arr::<Vec<i32>, i32, { &[5; 1] }>::new_ones();
    let _ = black_box(arr2) + black_box(arr);
}

fn carr_medium() {
    let arr = Arr::<Vec<i32>, i32, { &[5; 3] }>::new_ones();
    let arr2 = Arr::<Vec<i32>, i32, { &[5; 3] }>::new_ones();
    let _ = black_box(arr2) + black_box(arr);
}

fn carr_large() {
    let arr = Arr::<Vec<i32>, i32, { &[5; 5] }>::new_ones();
    let arr2 = Arr::<Vec<i32>, i32, { &[5; 5] }>::new_ones();
    let _ = black_box(arr2) + black_box(arr);
}



fn carr_huge() {
    let arr = Arr::<Vec<i32>, i32, { &[5; 8] }>::new_ones();
    let arr2 = Arr::<Vec<i32>, i32, { &[5; 8] }>::new_ones();
    let _ = black_box(arr2) + black_box(arr);
}

fn ndarray_small() {
    let arr = ndarray::Array1::<i32>::ones(5);
    let arr2 = ndarray::Array1::<i32>::ones(5);
    let _ = black_box(arr2) + black_box(arr);
}


fn ndarray_medium() {
    let arr = ndarray::Array3::<i32>::ones([5, 5, 5]);
    let arr2 = ndarray::Array3::<i32>::ones([5, 5, 5]);
    let _ = black_box(arr2) + black_box(arr);
}

fn ndarray_large() {
    let arr = ndarray::Array5::<i32>::ones([5, 5, 5, 5, 5]);
    let arr2 = ndarray::Array5::<i32>::ones([5, 5, 5, 5, 5]);
    let _ = black_box(arr2) + black_box(arr);
}

fn ndarray_huge() {
    let arr = ndarray::ArrayD::<i32>::ones(&[5, 5, 5, 5, 5, 5, 5, 5][..]);
    let arr2 = ndarray::ArrayD::<i32>::ones(&[5, 5, 5, 5, 5, 5, 5, 5][..]);
    let _ = black_box(arr2) + black_box(arr);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut c_carr = c.benchmark_group("Carr");
    c_carr.bench_function("small", |b| b.iter(carr_small));
    c_carr.bench_function("medium", |b| b.iter(carr_medium));
    c_carr.bench_function("large", |b| b.iter(carr_large));
    c_carr.bench_function("huge", |b| b.iter(carr_huge));
    c_carr.finish();

    let mut c_ndarray = c.benchmark_group("Ndarray");
    c_ndarray.bench_function("small", |b| b.iter(ndarray_small));
    c_ndarray.bench_function("medium", |b| b.iter(ndarray_medium));
    c_ndarray.bench_function("large", |b| b.iter(ndarray_large));
    c_ndarray.bench_function("huge", |b| b.iter(ndarray_huge));
    c_ndarray.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
