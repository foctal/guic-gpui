# guic-gpui

`guic-gpui` is a GPU-accelerated application framework maintained for GUIC and
general use. This project started as a fork of Zed's GPUI. It is not
an official Zed or GPUI release by Zed Industries.

## Installation

```toml
[dependencies]
gpui = { package = "guic-gpui", version = "0.1.0" }

[target.'cfg(any(target_os = "linux", target_os = "freebsd"))'.dependencies]
gpui_platform = { package = "guic-gpui-platform", version = "0.1.0", features = ["font-kit", "wayland", "x11", "runtime_shaders"] }

[target.'cfg(target_os = "macos")'.dependencies]
gpui_platform = { package = "guic-gpui-platform", version = "0.1.0", features = ["font-kit"] }

[target.'cfg(target_os = "windows")'.dependencies]
gpui_platform = { package = "guic-gpui-platform", version = "0.1.0" }
```

## License

The source is distributed under Apache-2.0. Bundled IBM Plex Sans and Lilex
fonts are distributed under SIL Open Font License 1.1.
