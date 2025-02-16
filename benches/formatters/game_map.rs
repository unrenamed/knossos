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

mod game_map {
    use super::*;

    pub fn format_10_x_10(c: &mut Criterion) {
        c.bench_function("game_map/format_10_x_10", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(10, 10);
                maze.save(&file_path, GameMap::new().span(5))
            })
        });
    }

    pub fn format_100_x_100(c: &mut Criterion) {
        c.bench_function("game_map/format_100_x_100", |b| {
            b.iter(|| {
                let output_dir = TempDir::new().unwrap();
                let file_path = format!("{}/maze.txt", output_dir.path().display());
                let maze = maze!(100, 100);
                maze.save(&file_path, GameMap::new().span(5))
            })
        });
    }
}

criterion_group!(
    benches,
    game_map::format_10_x_10,
    game_map::format_100_x_100,
);
criterion_main!(benches);
