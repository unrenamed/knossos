# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

## [Unreleased]

## [0.6.1] - 2025-02-11

### Added
- Added `Cell::to_bits` helper API for developer experience returns the `Cell::bits` representation.

### Updated
- Improved examples `bevy_pathfinding` and `bevy_ecs_tilemap` to use `Cell::to_bits` instead of `to_bits_str`.
- Cargo update

### Breaking
- `to_bits_str` now returns `&'static str` whereas `to_bits_string` now returns a `String`.

## [0.6.0] - 2025-02-8

### Added 
- Introduced feature `pathfinding` as default feature.
- Added A*Pathfinding for `OrthogonalMaze`. 
- Added `Start` and `Goal` components for `OrthogonalMaze`. Support for Pathfinding.
- Added extra functions to `CoordsComponent`:
    - `new` creates new `CoordsComponent` from `x` and `y` type usize
    - `xy` returns `(usize, usize)` of `CoordsComponent`. Where `(x, y)`
- Added maze `CellSize` Resource as type f32.
- Added examples:
    - bevy_ecs_tilemap
    - bevy_pathfinding 

### Updated
- Implemented `fmt::Display` for `CoordsComponent`.

## [0.5.2] - 2025-02-7

### Updated
- Cargo update

## [0.5.1] - 2025-01-31

### Added
- Added Bevy Plugin to support `Reflect` types.

### Coverage
- Increase test coverage for type `CooordsComponent`.

## [0.5.0] - 2025-01-30

### Breaking Change
- Forked to focus on Bevy Compatibility
- Add start coords for algorithms that support initial coords. Trait function changed from `Algorithm::generate(&mut self, grid: &mut Grid)` to `Algorithm::generate(&mut self, grid: &mut Grid, start_coords: Option<Coords>)`. **None** preserves previous behaviour.

### Added
- Add `Index` trait to `OrthogonalMaze` and `Grid` returning, now public, `Cell`.
- `OrthogonalMaze` can be a Bevy Resource.
- `Cell` can be a Bevy Component.
- `Cell` auxiliary methods (`iter, into_inter, to_bits_str`).
- `CoordsComponent` to map `Coords` as Bevy Component. Multiple `From` trait implemented for `CoordsComponent`.

## [0.4.1] - 2025-01-29

### Changed
- Apply more modern Rust code styling, including `rustfmt` and `cargo clippy`.
- Replace test bencher with `criterion`.
- Move non-release dependencies to `dev-dependencies`.

### Updated
- Run `cargo update` to update dependencies.

## [0.4.0] - 2023-11-01

### Added

- Implement an option to randomly place start `S` and goal `G` points along the borders ensuring a viable path between the two points for the [GameMap](./src/maze/formatters/game_map.rs) formatter.

- Add the new option `--with-start-goal` to the `game-map` command on CLI.

## [0.3.0] - 2023-05-06

### Added

- New `AsciiNarrow` and `AsciiNarrow` formatters replacing `Ascii::narrow()` and `Ascii::broad()` calls.

### Fixed

- Fix usage of old Ascii output types in code and docs.
- Move lib examples to the `examples` dir. `cargo run --example name` to run the specified example.

## [0.2.0] - 2023-04-02

### Added

- Implement knossos CLI.
- Add new narrow and broad ASCII formatters.

### Fixed

- Fix method to validate if a maze is valid.

### Changed

- Use bitflags to optimize and speed up maze generation process.

## [0.1.2] - 2022-04-11

### Added

- Orthogonal maze builder with 10 optional generation algorithms.
- Ascii, game map and image formatters to save the generated maze to files.

[unreleased]: https://github.com/unrenamed/knossos/compare/v0.5.1...HEAD
[0.5.1]: https://github.com/unrenamed/knossos/compare/v0.4.0...v0.5.1
[0.4.0]: https://github.com/unrenamed/knossos/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/unrenamed/knossos/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/unrenamed/knossos/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/unrenamed/knossos/releases/tag/v0.1.2
