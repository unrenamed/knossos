use criterion::{criterion_group, criterion_main, Criterion};
use knossos::maze::*;

mod eller {
    use super::*;

    pub fn generate_10_x_10(c: &mut Criterion) {
        c.bench_function("eller/generate_10_x_10", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(Eller))
                    .build();
            })
        });
    }

    pub fn generate_100_x_100(c: &mut Criterion) {
        c.bench_function("eller/generate_100_x_100", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(Eller))
                    .build();
            })
        });
    }
}

criterion_group!(
    benches,
    eller::generate_10_x_10,
    eller::generate_100_x_100
);
criterion_main!(benches);
