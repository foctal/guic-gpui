# guic-gpui

[![Crates.io](https://img.shields.io/crates/v/guic-gpui.svg)](https://crates.io/crates/guic-gpui)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/foctal/guic-gpui/blob/main/LICENSE)

`guic-gpui` is a GPU-accelerated UI framework for building interactive
applications in Rust. It is maintained for GUIC and general use. This project
started as a fork of Zed's GPUI.

## Installation

```toml
[dependencies]
gpui = { package = "guic-gpui", version = "0.6.0" }

[target.'cfg(any(target_os = "linux", target_os = "freebsd"))'.dependencies]
gpui_platform = { package = "guic-gpui-platform", version = "0.6.0", features = ["font-kit", "wayland", "x11", "runtime_shaders"] }

[target.'cfg(target_os = "macos")'.dependencies]
gpui_platform = { package = "guic-gpui-platform", version = "0.6.0", features = ["font-kit"] }

[target.'cfg(target_os = "windows")'.dependencies]
gpui_platform = { package = "guic-gpui-platform", version = "0.6.0" }
```

## Building and examples

Use Rust 1.95 or later. This repository pins Rust 1.95.0 for development.

- macOS: Install Xcode and its Metal toolchain. Enable `runtime_shaders` on
  `gpui_platform` to compile Metal shaders at application startup instead.
- Windows: Install the Windows SDK. Release builds require `fxc.exe`, found
  through the SDK installation, `PATH`, or `GPUI_FXC_PATH`.
- Linux: Install the Fontconfig, Wayland, XKB, X11/XCB development libraries for
  the enabled backends. Rendering requires a GPU supported by WGPU.

Run the desktop example with `cargo run -p guic-gpui-examples`.
The [WebAssembly backend](crates/gpui_web/README.md) is experimental.

## Clipboard integration

See [asynchronous text clipboard](docs/text-clipboard.md) for nonblocking desktop
text reads and [native file clipboard](docs/file-clipboard.md) for file copy/move
integration.

## Compatibility

See [the changelog](CHANGELOG.md) for API changes and migration notes. Windows
native controls and compositor behavior are described in
[platform integration](docs/upstream-fixes.md).

## License

The source is distributed under Apache-2.0. Bundled IBM Plex Sans and Lilex
fonts are distributed under SIL Open Font License 1.1.
