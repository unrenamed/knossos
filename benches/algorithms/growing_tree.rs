use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use knossos::maze::*;

mod growing_tree_method_oldest {
    use super::*;

    pub fn generate_10_x_10(c: &mut Criterion) {
        c.bench_function("growing_tree_method_oldest/generate_10_x_10", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(GrowingTree::new(Method::Oldest)))
                    .build();
            })
        });
    }

    pub fn generate_100_x_100(c: &mut Criterion) {
        c.bench_function("growing_tree_method_oldest/generate_100_x_100", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(GrowingTree::new(Method::Oldest)))
                    .build();
            })
        });
    }
}

mod growing_tree_method_newest {
    use super::*;

    pub fn generate_10_x_10(c: &mut Criterion) {
        c.bench_function("growing_tree_method_newest/generate_10_x_10", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(GrowingTree::new(Method::Newest)))
                    .build();
            })
        });
    }

    pub fn generate_100_x_100(c: &mut Criterion) {
        c.bench_function("growing_tree_method_newest/generate_100_x_100", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(GrowingTree::new(Method::Newest)))
                    .build();
            })
        });
    }
}

mod growing_tree_method_middle {
    use super::*;

    pub fn generate_10_x_10(c: &mut Criterion) {
        c.bench_function("growing_tree_method_middle/generate_10_x_10", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(GrowingTree::new(Method::Middle)))
                    .build();
            })
        });
    }

    pub fn generate_100_x_100(c: &mut Criterion) {
        c.bench_function("growing_tree_method_middle/generate_100_x_100", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(GrowingTree::new(Method::Middle)))
                    .build();
            })
        });
    }
}

mod growing_tree_method_random {
    use super::*;

    pub fn generate_10_x_10(c: &mut Criterion) {
        c.bench_function("growing_tree_method_random/generate_10_x_10", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(GrowingTree::new(Method::Random)))
                    .build();
            })
        });
    }

    pub fn generate_100_x_100(c: &mut Criterion) {
        c.bench_function("growing_tree_method_random/generate_100_x_100", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(GrowingTree::new(Method::Random)))
                    .build();
            })
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(10));
    targets =
        growing_tree_method_oldest::generate_10_x_10,
        growing_tree_method_oldest::generate_100_x_100,
        growing_tree_method_newest::generate_10_x_10,
        growing_tree_method_newest::generate_100_x_100,
        growing_tree_method_middle::generate_10_x_10,
        growing_tree_method_middle::generate_100_x_100,
        growing_tree_method_random::generate_10_x_10,
        growing_tree_method_random::generate_100_x_100,
}
criterion_main!(benches);
