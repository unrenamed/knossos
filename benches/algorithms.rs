#![feature(test)]

extern crate knossos;
extern crate test;

mod algorithms {
    use test::Bencher;
    use knossos::{maze::*};

    mod aldous_broder {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(AldousBroder))
                    .build();
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(AldousBroder))
                    .build();
            });
        }
    }

    mod binary_tree {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(BinaryTree::new(Bias::NorthEast)))
                    .build();
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(BinaryTree::new(Bias::NorthEast)))
                    .build();
            });
        }
    }

    mod eller {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(Eller))
                    .build();
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(Eller))
                    .build();
            });
        }
    }

    mod growing_tree_method_random {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(GrowingTree::new(Method::Random)))
                    .build();
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(GrowingTree::new(Method::Random)))
                    .build();
            });
        }
    }

    mod growing_tree_method_oldest {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(GrowingTree::new(Method::Oldest)))
                    .build();
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(GrowingTree::new(Method::Oldest)))
                    .build();
            });
        }
    }

    mod growing_tree_method_newest {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(GrowingTree::new(Method::Newest)))
                    .build();
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(GrowingTree::new(Method::Newest)))
                    .build();
            });
        }
    }

    mod growing_tree_method_middle {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(GrowingTree::new(Method::Middle)))
                    .build();
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(GrowingTree::new(Method::Middle)))
                    .build();
            });
        }
    }

    mod hunt_and_kill {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(HuntAndKill::new()))
                    .build();
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(HuntAndKill::new()))
                    .build();
            });
        }
    }

    mod kruskal {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(Kruskal))
                    .build();
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(Kruskal))
                    .build();
            });
        }
    }

    mod prim {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(Prim::new()))
                    .build();
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(Prim::new()))
                    .build();
            });
        }
    }

    mod recursive_backtracking {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(RecursiveBacktracking))
                    .build();
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(RecursiveBacktracking))
                    .build();
            });
        }
    }

    mod recursive_division {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(RecursiveDivision))
                    .build();
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(RecursiveDivision))
                    .build();
            });
        }
    }

    mod sidewinder {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(10)
                    .width(10)
                    .algorithm(Box::new(Sidewinder))
                    .build();
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                OrthogonalMazeBuilder::new()
                    .height(100)
                    .width(100)
                    .algorithm(Box::new(Sidewinder))
                    .build();
            });
        }
    }
}
