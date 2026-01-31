# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/) and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - yyyy-mm-dd

Here we write upgrading notes for brands. It's a team effort to make them as straightforward as possible.

### Added

### Changed

### Fixed

# [1.2.0] - 2026-01-31

### Added

- Added Windows ARM (aarch64) support: build and CI now handle Windows arm64 targets (see PR #24).
- Added Windows (arm64) to the list of precompiled TDLib binaries in the README.
- Added Windows CI coverage (windows-11-arm) to the CI matrices.

### Changed

- CI/build: Updated the `build-tdlib` workflow to support multiple architectures and set the minimum CMake policy to 3.5.
- CI: Removed unnecessary Rust installer on the `windows-11-arm` runner and adjusted workflow steps for cross-arch downloads.
- Examples: Updated `examples/get_me.rs` and enabled `tokio`'s `time` feature in `tdlib-rs/Cargo.toml`.
- Code: Modernized string formatting and applied small refactors across the codebase.

### Fixed

- Fixed README formatting and maintainers list indentation.
- Addressed Clippy warnings and small CI/workflow issues.

## [1.1.0] - 2025-04-17

### Added

- Added support for Linux ARM architecture.

### Fixed

- Fixed some small issues in the continuous integration pipeline.

## [1.0.5] - 2024-08-08

### Added

- Add `CHANGELOG.md` file to the project.

### Changed

- Now windows is linked correctly with `tdjson.lib` and `tdjson.dll`. But need to be fixed because the solution is not the best; we need to find the same solution for all platforms.

### Fixed

- Now the documentation is generated correctly. All functions in `src/build.rs` are documented.

## [1.0.4] - 2024-06-24

### Added

- Feature `local-tdlib` for local usage of TDLib.
- New build functions for enhanced customization and control.

### Changed

### Fixed

## [1.0.3] - 2024-06-20

### Added

### Changed

- Upgraded TDLib to version `1.8.29`.

### Fixed

## [1.0.2] - 2024-05-20

### Added

- CI/CD pipeline for the project.
- Feature `download-tdlib` to download TDLib directly.
- Ability to link TDLib without using `pkgconfig`.

### Changed

### Fixed
