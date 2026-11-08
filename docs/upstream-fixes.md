# Platform integration

## Windows native controls

Set `WindowOptions::windows_composition` when creating a window:

- `Auto` follows the platform default and `GPUI_DISABLE_DIRECT_COMPOSITION`.
- `DirectComposition` enables DirectComposition regardless of that environment
  setting. GPUI content is placed below composition-backed child HWNDs.
- `RedirectionBitmap` disables DirectComposition for the window and supports
  native menus and GDI child controls.

The mode cannot change after creation. Embedded WebViews occupy a separate
native layer; GPUI overlays cannot cover them. Other platforms ignore the setting.

The native rendering tests require an unobscured interactive Windows desktop
and a Direct3D device:

```sh
cargo test -p guic-gpui-windows --features test-support --lib native_ --locked -- --ignored --test-threads=1
```

## Wayland activation

Activation requests use the latest key or mouse press and its original surface.
Each token stays associated with its request. Compositors may still reject a
focus request according to their focus-stealing policy.

## Upstream references

Adapted fixes include [list insets](https://github.com/zed-industries/zed/commit/78fde14d124d),
[drag cursors](https://github.com/zed-industries/zed/commit/c7bea343838f),
[menu equivalents](https://github.com/zed-industries/zed/commit/d01f1258376e),
[Wayland activation](https://github.com/zed-industries/zed/commit/4d847859f550),
and [child-window composition](https://github.com/zed-industries/zed/commit/bc6fc117403d).
