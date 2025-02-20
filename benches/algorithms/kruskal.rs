use criterion::{criterion_group, criterion_main, Criterion};
use knossos::maze::*;

mod kruskal {
    use super::*;

    pub fn generate_10_x_10(c: &mut Criterion) {
        c.bench_function("kruskal/generate_10_x_10", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(Kruskal))
                    .build();
            })
        });
    }

    pub fn generate_100_x_100(c: &mut Criterion) {
        c.bench_function("kruskal/generate_100_x_100", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(Kruskal))
                    .build();
            })
        });
    }
}

criterion_group!(
    benches,
    kruskal::generate_10_x_10,
    kruskal::generate_100_x_100
);
criterion_main!(benches);
