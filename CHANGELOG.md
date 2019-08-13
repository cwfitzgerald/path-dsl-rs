# Changelog

All notable changes to this project will be documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

- [Unreleased](#unreleased)
- [v0.3.0](#v030)
- [v0.2.0](#v020)
- [v0.1.1](#v011)
- [v0.1.0](#v010)
- [Diffs](#diffs)

## Unreleased

## v0.4.0

#### Added
- `PathDSL::into_pathbuf`
- Missing `Into<PathBuf> for PathDSL`
- Primary Documentation
- This changelog
- README.md

#### Changed
- All functions marked `inline(always)`
- PathBuf is now `repr(transparent)` over `PathBuf`

#### Fixed
- Macro namespacing using `$crate` except where blocked by [rust-lang/rust#63460](https://github.com/rust-lang/rust/issues/63460).

## v0.3.0

Released 2019-08-09

#### Added
- Filesystem DSL macro with literal combining.

#### Changed
- All functions marked inline

## v0.2.0

Released 2019-08-08

#### Added
- All missing traits needed for feature-parity with `PathBuf`

## v0.1.1

Released 2019-08-08

#### Added
- Added link docs.rs link to crates.io page

## v0.1.0

Released 2019-08-08

#### Added
- First release of `path-dsl`, a rust library for path construction.

## Diffs

- [Unreleased](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.4.0...HEAD)
- [v0.4.0](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.3.0...v0.4.0)
- [v0.3.0](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.2.0...v0.3.0)
- [v0.2.0](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.1.1...v0.2.0)
- [v0.1.1](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.1.0...v0.1.1)
