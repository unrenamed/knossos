use criterion::{criterion_group, criterion_main, Criterion};
use knossos::maze::*;

mod recursive_division {
    use super::*;

    pub fn generate_10_x_10(c: &mut Criterion) {
        c.bench_function("recursive_division/generate_10_x_10", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(RecursiveDivision))
                    .build();
            })
        });
    }

    pub fn generate_100_x_100(c: &mut Criterion) {
        c.bench_function("recursive_division/generate_100_x_100", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(RecursiveDivision))
                    .build();
            })
        });
    }
}

criterion_group!(
    benches,
    recursive_division::generate_10_x_10,
    recursive_division::generate_100_x_100
);
criterion_main!(benches);
