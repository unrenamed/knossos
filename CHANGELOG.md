# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

## [Unreleased]

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

[unreleased]: https://github.com/unrenamed/knossos/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/unrenamed/knossos/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/unrenamed/knossos/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/unrenamed/knossos/releases/tag/v0.1.2
