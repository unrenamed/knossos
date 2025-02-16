use criterion::{criterion_group, criterion_main, Criterion};
use knossos::maze::*;

mod sidewinder {
    use super::*;

    pub fn generate_10_x_10(c: &mut Criterion) {
        c.bench_function("sidewinder/generate_10_x_10", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(Sidewinder))
                    .build();
            })
        });
    }

    pub fn generate_100_x_100(c: &mut Criterion) {
        c.bench_function("sidewinder/generate_100_x_100", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(Sidewinder))
                    .build();
            })
        });
    }
}

criterion_group!(
    benches,
    sidewinder::generate_10_x_10,
    sidewinder::generate_100_x_100
);
criterion_main!(benches);
