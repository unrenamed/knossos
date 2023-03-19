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
    use assert_fs::fixture::TempDir;
    use test::Bencher;
    use knossos::maze::*;

    mod image {
        use super::*;

        #[bench]
        fn format_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.png", output_dir.path().display());
                let maze = maze!(10, 10);
                maze.save(&file_path, Image::new())
            });
        }

        #[bench]
        fn format_50_x_50(b: &mut Bencher) {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.png", output_dir.path().display());
                let maze = maze!(50, 50);
                maze.save(&file_path, Image::new())
            });
        }
    }

    mod ascii_default {
        use super::*;

        #[bench]
        fn format_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(10, 10);
                maze.save(&file_path, Ascii::<formatters::Default>::new())
            });
        }

        #[bench]
        fn format_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(100, 100);
                maze.save(&file_path, Ascii::<formatters::Default>::new())
            });
        }
    }

    mod ascii_enhanced {
        use super::*;

        #[bench]
        fn format_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(10, 10);
                maze.save(&file_path, Ascii::<formatters::Enhanced>::new())
            });
        }

        #[bench]
        fn format_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(100, 100);
                maze.save(&file_path, Ascii::<formatters::Enhanced>::new())
            });
        }
    }

    mod game_map {
        use super::*;

        #[bench]
        fn format_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(10, 10);
                maze.save(&file_path, GameMap::new().span(5))
            });
        }

        #[bench]
        fn format_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(100, 100);
                maze.save(&file_path, GameMap::new().span(5))
            });
        }
    }
}
