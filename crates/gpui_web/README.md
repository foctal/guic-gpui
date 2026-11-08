# guic-gpui-web

The experimental WebAssembly backend for `guic-gpui`, connecting canvas
windows, browser input, and Fetch-based HTTP.

The `hello_web` example requires nightly Rust, `rust-src`, the
`wasm32-unknown-unknown` target, and Trunk. From `examples/hello_web`, run
`cargo check --target wasm32-unknown-unknown` or `trunk serve`.
The example's Cargo configuration rebuilds the standard library with atomics;
its server configuration supplies the cross-origin isolation headers required
for shared memory. Use a browser with WebGPU support.

This project started as a fork of Zed's GPUI.

Licensed under Apache-2.0.
