# Changelog

All notable user-visible changes are recorded here.

## [0.3.1] - 2026-09-06

### Fixed

- macOS: Compare font variation and descriptor dictionaries using Core Foundation
  equality, restoring `font-kit` backend compatibility with `core-foundation`
  0.10.0 while preserving font equivalence checks.

## [0.3.0] - 2026-09-06

### Fixed

- Preserve explicitly configured image aspect ratios during layout.
- Snap recomputed padding to the device pixel grid to avoid spurious scrolling.
- Prevent multi-modifier gestures from triggering standalone modifier bindings.
- Cancel pending input when a window is removed and prevent stale timeouts from
  affecting replacement input sequences or windows.
- Return an unsupported-handle error instead of panicking when requesting raw
  window or display handles from test windows.
- Preserve caller metadata and non-empty targets in error logging.
- macOS: Distinguish equivalent font aliases from conflicting font registrations.
- Windows: Clear the render target before compositing color emoji to prevent
  artifacts in uncovered pixels.
- Windows: Use the destination monitor's DPI when calculating initial window
  placement.
- Windows: Handle IME attribute read errors and validate cursor adjacency against
  the returned attributes.
- Windows: Correct clipboard bitmap pixel offsets for extended DIB headers.
- Windows: Improve shader compiler discovery and track shader and environment
  changes that require recompilation.
- Linux: Append the missing NUL terminator to the X11 `WM_CLASS` property.
- Linux: Release X11 client state before invoking window-close callbacks to avoid
  borrow conflicts during callback re-entry.
- Linux: Prevent inactive Wayland windows from updating the IME cursor position.
- Linux: Handle XKB context initialization failures in X11 and Wayland.
- Linux: Route Wayland backend errors through the logger to avoid panics when
  standard error is unwritable.
- Linux: Forward the X11 feature to the core GPUI crate.

### Added

- Add `AsyncWindowContext::update_if_present` for updates that tolerate a
  destroyed window while preserving other update errors.
- Add regression tests for the fixes and a manual Windows placement example.

## [0.2.0] - 2026-08-11

- Clarify the purpose of each crate in its README and package metadata.
- Correct the workspace example instructions.

## [0.1.0] - 2026-08-11

- Fork GPUI and its required support crates from Zed commit
  `5e1fd392f67e27fa1da91bad43eef7db1a5dec23`.
- Introduce the namespaced, lockstep `guic-gpui` crate family.
- Remove GPL-licensed Zed-specific tracing and logging packages and unrelated utility chains.
- Replace runtime and build Git dependencies with crates.io releases.
- Move bundled fonts into the licensed `guic-gpui-assets` package.
- Add dependency, package, and release checks.
