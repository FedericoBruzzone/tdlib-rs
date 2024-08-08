# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/) and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - yyyy-mm-dd
Here we write upgrading notes for brands. It's a team effort to make them as straightforward as possible.
### Added
### Changed
### Fixed

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

