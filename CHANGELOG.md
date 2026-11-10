# Changelog

All notable user-visible changes are recorded here.

## [0.6.1] - 2026-11-10

### Fixed

- Preserve window resize and move notifications that arrive reentrantly while
  the application or window is borrowed. Apply the latest bounds and scale
  factor before the next draw to prevent stale viewport sizes and layout.

## [0.6.0] - 2026-11-08

### Compatibility

- Add `windows_composition` to `WindowOptions` and `WindowParams`. Exhaustive
  struct literals must supply the field; `WindowOptions` literals using
  `..Default::default()` remain compatible. Use `WindowsCompositionMode::Auto`
  to preserve the previous behavior. The setting is ignored outside Windows.

### Added

- Add `App::read_text_from_clipboard_async()` for background plain-text reads
  on macOS, Windows, X11, and Wayland. Custom platform implementations retain a
  synchronous fallback. See [text clipboard integration](docs/text-clipboard.md).
- Add `ClipboardItem::plain_text()` to extract text without file-path fallback.
- Add per-window Windows composition modes. `RedirectionBitmap` supports native
  menus and GDI child controls; `DirectComposition` overrides the process-wide
  environment setting.

### Fixed

- Include padding and borders in list item measurement, placement, reported
  bounds, and scrollbar limits. Remeasure items when the content width changes.
- Wake blocked queue receivers when the sender disconnects and keep exhausted
  iterators exhausted. Stop idle Linux dispatcher threads on shutdown.
- Apply drag cursors immediately after mouse listeners run.
- Preserve native window tab order and existing groups when reconciling tabs,
  and make group navigation deterministic.
- Distinguish file-dialog cancellation from failures on Windows and Linux.
  Preserve native Windows paths and release Shell-allocated path strings.
- macOS: Correct Backspace and Delete menu equivalents.
- macOS: Reject noncontiguous Core Media buffers before constructing a byte
  slice that would extend past the first memory block.
- Wayland: Pair activation tokens with their original requests and input
  surfaces, including keyboard activation and serial rollover.
- Wayland: Keep clipboard writes nonblocking when recipients stop reading.
- Windows: Place DirectComposition content below native child windows.
- Windows: Pass restart paths through the helper's environment to preserve
  native strings and prevent PowerShell interpretation.
- Windows: Ignore unsupported hide/unhide-other-applications requests.
- FreeBSD: Expose the task queue required by the platform dispatcher.
- Web: Fix example dependency aliases and exclude native path conversion from
  targets without filesystem path support.
- Require Tokio 1.53.3 or later, addressing RUSTSEC-2026-0386, and replace the
  yanked `async-executor` lockfile entry.

## [0.5.0] - 2026-11-02

- Keep the element arena in a separate allocation to avoid aliasing it through
  a raw pointer while the application is mutably borrowed.
- Release pooled text line wrappers without retaining the text system in a
  reference cycle.
- Defer native window-close cleanup to avoid reentrant application borrows.
- Redraw cached views when inherited opacity changes, including hover styles.
- Clarify UTF-16 input range units, window-local input geometry, and frame-scoped
  input-handler registration.
- Add regression coverage for bordered scroll panes at fractional display scales.
- Linux: Align backend feature gates and remove obsolete coverage attributes.

## [0.4.0] - 2026-10-10

- Avoid a panic when hit testing beyond a wrapped line's measured width and keep
  closing punctuation attached to the preceding word when wrapping.
- Exclude disabled key bindings from next-keystroke suggestions.
- Center and constrain windows within the display's visible work area.
- Preserve debug selectors when reusing cached views, and support simulated
  display scale changes in test windows.
- macOS: Release the accessibility adapter on window teardown and stop replaying
  synthetic drag events after the original mouse button is released.
- Linux: Avoid local XKB include paths when loading server-provided keymaps.
- WGPU: Preserve fonts without Latin glyphs for fallback text rendering.
- Windows: Preserve default DirectWrite ligatures, replace embedded clipboard NUL
  characters with spaces while preserving metadata, and fix manifest resource
  include paths for cross-compilation.

- Add a fallible native file clipboard API with separate copy/move intent and
  capability reporting. Implement macOS copy, Windows CF_HDROP, and X11/Wayland
  file MIME offers. macOS move and unsupported backends return explicit errors.
  See [the contract and native validation checklist](docs/file-clipboard.md).
- Include both nodes and their element provenance in debug duplicate-focus
  diagnostics, preserving the accessibility invariant.
- Add `Window::enable_a11y_for_testing` under `test-support`, with corrected and
  invalid nested-focus component fixtures.

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
