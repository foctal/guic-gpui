# Changelog

All notable user-visible changes are recorded here.

## [0.1.0] - 2026-08-11

- Fork GPUI and its required support crates from Zed commit
  `5e1fd392f67e27fa1da91bad43eef7db1a5dec23`.
- Introduce the namespaced, lockstep `guic-gpui` crate family.
- Remove GPL-licensed Zed-specific tracing and logging packages and unrelated utility chains.
- Replace runtime and build Git dependencies with crates.io releases.
- Move bundled fonts into the licensed `guic-gpui-assets` package.
- Add dependency, package, and release checks.
