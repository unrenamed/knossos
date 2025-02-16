use criterion::criterion_main;

mod ascii;
mod game_map;
mod image;

criterion_main!(ascii::benches, game_map::benches, image::benches);
