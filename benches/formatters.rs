#![feature(test)]

extern crate knossos;
extern crate test;

macro_rules! maze {
    ($width:expr, $height:expr) => {
        OrthogonalMazeBuilder::new()
            .height($height)
            .width($width)
            .algorithm(Box::new(RecursiveBacktracking))
            .build()
    };
}

mod formatters {
    use test::Bencher;
    use knossos::{maze::*};

    mod image {
        use super::*;

        #[bench]
        fn format_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                let maze = maze!(10, 10);
                maze.save("output/maze.png", Image::new())
            });
        }

        #[bench]
        fn format_50_x_50(b: &mut Bencher) {
            b.iter(|| {
                let maze = maze!(50, 50);
                maze.save("output/maze.png", Image::new())
            });
        }
    }

    mod ascii_default {
        use super::*;

        #[bench]
        fn format_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                let maze = maze!(10, 10);
                maze.save("output/maze.txt", Ascii::<formatters::Default>::new())
            });
        }

        #[bench]
        fn format_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                let maze = maze!(100, 100);
                maze.save("output/maze.txt", Ascii::<formatters::Default>::new())
            });
        }
    }

    mod ascii_enhanced {
        use super::*;

        #[bench]
        fn format_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                let maze = maze!(10, 10);
                maze.save("output/maze.txt", Ascii::<formatters::Enhanced>::new())
            });
        }

        #[bench]
        fn format_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                let maze = maze!(100, 100);
                maze.save("output/maze.txt", Ascii::<formatters::Enhanced>::new())
            });
        }
    }

    mod game_map {
        use super::*;

        #[bench]
        fn format_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                let maze = maze!(10, 10);
                maze.save("output/maze.txt", GameMap::new().span(5))
            });
        }

        #[bench]
        fn format_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                let maze = maze!(100, 100);
                maze.save("output/maze.txt", GameMap::new().span(5))
            });
        }
    }
}
