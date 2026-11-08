# Selected upstream fixes

The 0.6.0 development series incorporates focused fixes from Zed's GPUI.
These changes are adapted to this fork rather than replacing its platform
implementations or removing existing APIs.

| Upstream change | Integration |
| --- | --- |
| [List content insets](https://github.com/zed-industries/zed/commit/78fde14d124d) | Include horizontal padding and borders in item measurement, placement, reported bounds, and cache invalidation. Preserve this fork's scroll API and autoscroll behavior. |
| [Drag cursor](https://github.com/zed-industries/zed/commit/c7bea343838f) | Resolve the cursor after mouse listeners and honor a newly started drag before the next paint. Preserve pointer capture and inspector behavior. |
| [macOS menu equivalents](https://github.com/zed-industries/zed/commit/d01f1258376e) | Map Backspace and Delete to AppKit menu equivalents through a separate helper. Keep native event character conversion unchanged. |
| [Wayland activation](https://github.com/zed-industries/zed/commit/4d847859f550) | Keep each activation token paired with its own request and use the latest press and its original surface. Preserve clipboard serial tracking and no-token fallbacks. |
| [Windows child windows](https://github.com/zed-industries/zed/commit/bc6fc117403d) | Place the DirectComposition visual below child HWNDs. Keep per-window composition settings and the native-menu redirection fallback. |

Regression tests cover list layout and remeasurement, cursor behavior before
painting, menu versus native-event key mappings, and Wayland token request data,
serial rollover, input-surface tracking, and destroyed surfaces. Wayland tests
use client proxies over a private socket; they do not prove that a compositor
will grant focus, which remains subject to desktop focus-stealing policy.

The Windows native tests exercise mixed composition modes and child-window
visibility on a desktop with a Direct3D device.
An additional opt-in Windows test renders a blue GPUI scene and a white native
child, then checks their actual desktop pixels. Run the following on an
unobscured interactive desktop to exercise both native rendering checks:

```sh
cargo test -p guic-gpui-windows --features test-support --lib native_ --locked -- --ignored --test-threads=1
```
