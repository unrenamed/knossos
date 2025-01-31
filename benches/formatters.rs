use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! maze {
    ($width:expr, $height:expr) => {
        OrthogonalMazeBuilder::new()
            .height($height)
            .width($width)
            .algorithm(Box::new(RecursiveBacktracking))
            .build()
    };
}

criterion_group!(
    benches,
    game_map::format_10_x_10,
    game_map::format_100_x_100,
    ascii_broad::format_10_x_10,
    ascii_broad::format_100_x_100,
    ascii_narrow::format_10_x_10,
    ascii_narrow::format_100_x_100,
    image::format_10_x_10,
    image::format_50_x_50,
);
criterion_main!(benches);

use assert_fs::fixture::TempDir;
use bevy_knossos::maze::*;

mod image {
    use super::*;

    pub fn format_10_x_10(c: &mut Criterion) {
        c.bench_function("image/format_10_x_10", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.png", output_dir.path().display());
                let maze = maze!(10, 10).unwrap();
                maze.save(&file_path, Image::new())
            })
        });
    }

    pub fn format_50_x_50(c: &mut Criterion) {
        c.bench_function("image/format_50_x_50", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.png", output_dir.path().display());
                let maze = maze!(50, 50).unwrap();
                maze.save(&file_path, Image::new())
            })
        });
    }
}

mod ascii_narrow {
    use super::*;

    pub fn format_10_x_10(c: &mut Criterion) {
        c.bench_function("ascii_narrow/format_10_x_10", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(10, 10).unwrap();
                maze.save(&file_path, AsciiNarrow)
            })
        });
    }

    pub fn format_100_x_100(c: &mut Criterion) {
        c.bench_function("ascii_narrow/format_100_x_100", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(100, 100).unwrap();
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
                let maze = maze!(10, 10).unwrap();
                maze.save(&file_path, AsciiBroad)
            })
        });
    }

    pub fn format_100_x_100(c: &mut Criterion) {
        c.bench_function("ascii_broad/format_100_x_100", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(100, 100).unwrap();
                maze.save(&file_path, AsciiBroad)
            })
        });
    }
}

mod game_map {
    use super::*;

    pub fn format_10_x_10(c: &mut Criterion) {
        c.bench_function("game_map/format_10_x_10", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(10, 10).unwrap();
                maze.save(&file_path, GameMap::new().span(5))
            })
        });
    }

    pub fn format_100_x_100(c: &mut Criterion) {
        c.bench_function("game_map/format_100_x_100", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(100, 100).unwrap();
                maze.save(&file_path, GameMap::new().span(5))
            })
        });
    }
}
