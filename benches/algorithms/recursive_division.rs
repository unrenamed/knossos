use criterion::{criterion_group, criterion_main, Criterion};
use knossos::maze::*;

mod aldous_broder {
    use super::*;

    pub fn generate_10_x_10(c: &mut Criterion) {
        c.bench_function("aldous_broder/generate_10_x_10", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(AldousBroder))
                    .build();
            })
        });
    }

    pub fn generate_100_x_100(c: &mut Criterion) {
        c.bench_function("aldous_broder/generate_100_x_100", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(AldousBroder))
                    .build();
            })
        });
    }
}

criterion_group!(
    benches,
    aldous_broder::generate_10_x_10,
    aldous_broder::generate_100_x_100
);
criterion_main!(benches);
