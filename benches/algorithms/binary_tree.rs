use criterion::{criterion_group, criterion_main, Criterion};
use knossos::maze::*;

mod binary_tree {
    use super::*;

    pub fn generate_10_x_10(c: &mut Criterion) {
        c.bench_function("binary_tree/generate_10_x_10", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(BinaryTree::new(Bias::NorthEast)))
                    .build();
            })
        });
    }

    pub fn generate_100_x_100(c: &mut Criterion) {
        c.bench_function("binary_tree/generate_100_x_100", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(BinaryTree::new(Bias::NorthEast)))
                    .build();
            })
        });
    }
}

criterion_group!(
    benches,
    binary_tree::generate_10_x_10,
    binary_tree::generate_100_x_100,
);
criterion_main!(benches);
