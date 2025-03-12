# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

## [Unreleased]

## [1.2.0] - 2025-03-12

### Added

- Library: Make start & goal positions optionally seedable in `GameMap`.

## [1.1.0] - 2025-02-24

### Added

- Library: Implement optional random seeding for maze algorithms to enable deterministic outputs.
- CLI: Introduce an optional `--seed` argument for reproducible maze generation.

### Fixed

- Resolve duplicate algorithm benchmarks.

### Refactored

- Re-order imports for better readability.

## [1.0.0] - 2025-02-16

### Breaking Changes

- `get_grid_mut` is removed from the library's public interface.
- Maze builder now enforces positive width and height values.
- Image formatter now enforces positive passage and wall values.

### Added

- New method to format maze without saving to file.
- Example for maze formatting to the crate docs.

### Fixed

- Resolve margin(0) and right shift bugs in maze rendering.
- Correct typos.

### Changed

- Apply aggressive `rustfmt` and `clippy` suggestions for improved code quality by [@naomijub](https://github.com/naomijub).
- Replace test bencher with `criterion` by [@naomijub](https://github.com/naomijub).
- Update all crate dependencies to the latest versions by [@naomijub](https://github.com/naomijub).

### Refactored

- Benchmark code into modular files.

## [0.4.0] - 2023-11-01

### Added

- Implement an option to randomly place start `S` and goal `G` points along the borders ensuring a viable path between the two points for the [GameMap](./src/maze/formatters/game_map.rs) formatter

- Add the new option `--with-start-goal` to the `game-map` command on CLI

## [0.3.0] - 2023-05-06

### Added

- New `AsciiNarrow` and `AsciiNarrow` formatters replacing `Ascii::narrow()` and `Ascii::broad()` calls

### Fixed

- Fix usage of old Ascii output types in code and docs
- Move lib examples to the `examples` dir. `cargo run --example name` to run the specified example

## [0.2.0] - 2023-04-02

### Added

- Implement knossos CLI
- Add new narrow and broad ASCII formatters

### Fixed

- Fix method to validate if a maze is valid

### Changed

- Use bitflags to optimize and speed up maze generation process

## [0.1.2] - 2022-04-11

### Added

- Orthogonal maze builder with 10 optional generation algorithms
- Ascii, game map and image formatters to save the generated maze to files

[unreleased]: https://github.com/unrenamed/knossos/compare/v1.2.0...HEAD
[1.2.0]: https://github.com/unrenamed/knossos/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/unrenamed/knossos/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/unrenamed/knossos/compare/v0.4.0...v1.0.0
[0.4.0]: https://github.com/unrenamed/knossos/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/unrenamed/knossos/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/unrenamed/knossos/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/unrenamed/knossos/releases/tag/v0.1.2
