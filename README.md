# guic-gpui

[![Crates.io](https://img.shields.io/crates/v/guic-gpui.svg)](https://crates.io/crates/guic-gpui)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/foctal/guic-gpui/blob/main/LICENSE)

`guic-gpui` is a GPU-accelerated UI framework for building interactive
applications in Rust. It is maintained for GUIC and general use. This project
started as a fork of Zed's GPUI.

## Installation

```toml
[dependencies]
gpui = { package = "guic-gpui", version = "0.3.1" }

[target.'cfg(any(target_os = "linux", target_os = "freebsd"))'.dependencies]
gpui_platform = { package = "guic-gpui-platform", version = "0.3.1", features = ["font-kit", "wayland", "x11", "runtime_shaders"] }

[target.'cfg(target_os = "macos")'.dependencies]
gpui_platform = { package = "guic-gpui-platform", version = "0.3.1", features = ["font-kit"] }

[target.'cfg(target_os = "windows")'.dependencies]
gpui_platform = { package = "guic-gpui-platform", version = "0.3.1" }
```

## License

The source is distributed under Apache-2.0. Bundled IBM Plex Sans and Lilex
fonts are distributed under SIL Open Font License 1.1.
