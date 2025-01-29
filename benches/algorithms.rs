use criterion::{criterion_group, criterion_main, Criterion};

criterion_group!(
    benches,
    aldous_broder::generate_10_x_10,
    aldous_broder::generate_100_x_100,
    binary_tree::generate_10_x_10,
    binary_tree::generate_100_x_100,
    eller::generate_10_x_10,
    eller::generate_100_x_100,
    growing_tree_method_random::generate_10_x_10,
    growing_tree_method_random::generate_100_x_100,
    growing_tree_method_oldest::generate_10_x_10,
    growing_tree_method_oldest::generate_100_x_100,
    growing_tree_method_newest::generate_10_x_10,
    growing_tree_method_newest::generate_100_x_100,
    growing_tree_method_middle::generate_10_x_10,
    growing_tree_method_middle::generate_100_x_100,
    hunt_and_kill::generate_10_x_10,
    hunt_and_kill::generate_100_x_100,
    kruskal::generate_10_x_10,
    kruskal::generate_100_x_100,
    prim::generate_10_x_10,
    prim::generate_100_x_100,
    recursive_backtracking::generate_10_x_10,
    recursive_backtracking::generate_100_x_100,
    recursive_division::generate_10_x_10,
    recursive_division::generate_100_x_100,
    sidewinder::generate_10_x_10,
    sidewinder::generate_100_x_100,
);
criterion_main!(benches);

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

mod eller {
    use super::*;

    pub fn generate_10_x_10(c: &mut Criterion) {
        c.bench_function("eller//generate_10_x_10", |b| {
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
        c.bench_function("eller//generate_100_x_100", |b| {
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

mod hunt_and_kill {
    use super::*;

    pub fn generate_10_x_10(c: &mut Criterion) {
        c.bench_function("hunt_and_kill/generate_10_x_10", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(HuntAndKill::new()))
                    .build();
            })
        });
    }

    pub fn generate_100_x_100(c: &mut Criterion) {
        c.bench_function("hunt_and_kill/generate_100_x_100", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(HuntAndKill::new()))
                    .build();
            })
        });
    }
}

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

mod prim {
    use super::*;

    pub fn generate_10_x_10(c: &mut Criterion) {
        c.bench_function("prim/generate_10_x_10", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(Prim::new()))
                    .build();
            })
        });
    }

    pub fn generate_100_x_100(c: &mut Criterion) {
        c.bench_function("prim/generate_100_x_100", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(Prim::new()))
                    .build();
            })
        });
    }
}

mod recursive_backtracking {
    use super::*;

    pub fn generate_10_x_10(c: &mut Criterion) {
        c.bench_function("recursive_backtracking/generate_10_x_10", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(RecursiveBacktracking))
                    .build();
            })
        });
    }

    pub fn generate_100_x_100(c: &mut Criterion) {
        c.bench_function("recursive_backtracking/generate_100_x_100", |b| {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(RecursiveBacktracking))
                    .build();
            })
        });
    }
}

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
