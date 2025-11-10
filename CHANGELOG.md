# Changelog

All notable changes to this project will be documented in this file.

The format is based on https://keepachangelog.com/ and this project follows Semantic Versioning.


## [Unreleased]

### Added
- better error logging for compile time errors
- builtin methods for datatypes like `.length()`

### Changed
- Internal refactor of CLI
- `build` and `run` commands now create `.mluva` directory, `init` command no longer creates it
- Removed `uninit` command from CLI

### Fixed
- N/A

### Security
- N/A

---

## [0.1.0] - 2025-11-01
### Added
- Initial public release of Mluva.
- Basic datatypes: Int, Float, String, Bool, Void.
- Syntax: statements, expressions, functions, modules, if/while, builtin functions.
- CLI: init/uninit/build/run commands.
- Bytecode serializer/deserializer.
- docs/: language reference and examples.
- examples/: basic control-flow, functions.
- CI: GitHub Actions workflow for fmt, clippy, and tests.

### Changed
- N/A

### Fixed
- N/A

### Security
- N/A

---

[Unreleased]: https://github.com/MikulasBar/mluva/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/MikulasBar/mluva/releases/tag/v0.1.0