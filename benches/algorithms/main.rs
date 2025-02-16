use criterion::criterion_main;

mod aldous_broder;
mod binary_tree;
mod sidewinder;
mod growing_tree;
mod kruskal;
mod prim;
mod hunt_and_kill;
mod recursive_backtracking;
mod recursive_division;

criterion_main!(
    aldous_broder::benches,
    binary_tree::benches,
    sidewinder::benches,
    growing_tree::benches,
    kruskal::benches,
    prim::benches,
    hunt_and_kill::benches,
    recursive_backtracking::benches,
    recursive_division::benches,
);
