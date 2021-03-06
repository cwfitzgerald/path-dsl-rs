# Changelog

All notable changes to this project will be documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

- [Unreleased](#unreleased)
- [v0.6.1](#v061)
- [v0.6.0](#v060)
- [v0.5.4](#v054)
- [v0.5.3](#v053)
- [v0.5.2](#v052)
- [v0.5.1](#v051)
- [v0.5.0](#v050)
- [v0.4.0](#v040)
- [v0.3.0](#v030)
- [v0.2.0](#v020)
- [v0.1.1](#v011)
- [v0.1.0](#v010)
- [Diffs](#diffs)

## Unreleased

## v0.6.1

Released 2020-07-11

#### Fixed
- Updated readme for v0.6.0 release

## v0.6.0

Released 2020-07-11

#### Added
- Converted macro to output PathBuf directly
- Discouraged manual use of PathDSL

## v0.5.4

Released 2019-08-25

#### Added
- Badges to Readme
- Keywords on crates.io
- Categories on crates.io

## v0.5.3

Released 2019-08-25

#### Fixed
- Inconsequential documentation link issue

## v0.5.2

Released 2019-08-25

#### Changed
- Documentation rewrite to make it more user friendly.

## v0.5.1

Released 2019-08-15

#### Fixed
- Polished documentation
- All clippy lints!

## v0.5.0

Released 2019-08-15

#### Added
- Implemented CopylessDSL
- Added missing `From<Cow<OsStr>>` for `PathDSL`
- Documentation for all member functions
- Many tests

#### Fixed
- Properly handle &mut
- Documentation links now always point to newest version

## v0.4.0

Released 2019-08-13

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

Released 2019-08-10

#### Added
- Filesystem DSL macro with literal combining.

#### Changed
- All functions marked inline

## v0.2.0

Released 2019-08-09

#### Added
- All missing traits needed for feature-parity with `PathBuf`

## v0.1.1

Released 2019-08-09

#### Added
- Added link docs.rs link to crates.io page

## v0.1.0

Released 2019-08-09

#### Added
- First release of `path-dsl`, a rust library for path construction.

## Diffs

- [Unreleased](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.6.1...HEAD)
- [v0.6.1](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.6.0...v0.6.1)
- [v0.6.0](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.5.4...v0.6.0)
- [v0.5.4](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.5.3...v0.5.4)
- [v0.5.3](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.5.2...v0.5.3)
- [v0.5.2](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.5.1...v0.5.2)
- [v0.5.1](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.5.0...v0.5.1)
- [v0.5.0](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.4.0...v0.5.0)
- [v0.4.0](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.3.0...v0.4.0)
- [v0.3.0](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.2.0...v0.3.0)
- [v0.2.0](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.1.1...v0.2.0)
- [v0.1.1](https://github.com/cwfitzgerald/path-dsl-rs/compare/v0.1.0...v0.1.1)
