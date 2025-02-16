use assert_fs::TempDir;
use criterion::{criterion_group, criterion_main, Criterion};
use knossos::maze::*;

macro_rules! maze {
    ($width:expr, $height:expr) => {
        OrthogonalMazeBuilder::new()
            .height($height)
            .width($width)
            .algorithm(Box::new(RecursiveBacktracking))
            .build()
    };
}

mod ascii_narrow {
    use super::*;

    pub fn format_10_x_10(c: &mut Criterion) {
        c.bench_function("ascii_narrow/format_10_x_10", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(10, 10);
                maze.save(&file_path, AsciiNarrow)
            })
        });
    }

    pub fn format_100_x_100(c: &mut Criterion) {
        c.bench_function("ascii_narrow/format_100_x_100", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(100, 100);
                maze.save(&file_path, AsciiNarrow)
            })
        });
    }
}

mod ascii_broad {
    use super::*;

    pub fn format_10_x_10(c: &mut Criterion) {
        c.bench_function("ascii_broad/format_10_x_10", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(10, 10);
                maze.save(&file_path, AsciiBroad)
            })
        });
    }

    pub fn format_100_x_100(c: &mut Criterion) {
        c.bench_function("ascii_broad/format_100_x_100", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(100, 100);
                maze.save(&file_path, AsciiBroad)
            })
        });
    }
}

criterion_group!(
    benches,
    ascii_narrow::format_10_x_10,
    ascii_narrow::format_100_x_100,
    ascii_broad::format_10_x_10,
    ascii_broad::format_100_x_100,
);
criterion_main!(benches);
