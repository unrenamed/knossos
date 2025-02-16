use assert_fs::TempDir;
use std::time::Duration;
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

mod image {
    use super::*;

    pub fn format_10_x_10(c: &mut Criterion) {
        c.bench_function("image/format_10_x_10", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.png", output_dir.path().display());
                let maze = maze!(10, 10);
                maze.save(&file_path, Image::new())
            })
        });
    }

    pub fn format_100_x_100(c: &mut Criterion) {
        let mut group = c.benchmark_group("image");
        group.measurement_time(Duration::from_secs(80));
        group.bench_function("format_100_x_100", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.png", output_dir.path().display());
                let maze = maze!(100, 100);
                maze.save(&file_path, Image::new())
            })
        });
        group.finish();
    }
}

criterion_group!(benches, image::format_10_x_10, image::format_100_x_100,);
criterion_main!(benches);
