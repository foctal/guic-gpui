# Asynchronous text clipboard

`App::read_text_from_clipboard_async()` returns a `Task<Option<String>>`. Await
it in a foreground task and apply the result through the application context.
Do not block the foreground executor waiting for the task.

The desktop backends perform text transfer and decoding outside the UI thread.
The API requests plain text only: it does not load images, file lists, or
application metadata. `None` means no nonempty plain text was available, or the
native read failed. Line endings are preserved. Use the existing
`read_from_clipboard()` API when the complete clipboard item is needed;
that API retains its synchronous behavior.

Applications must retain their own target document, selection, and request
identity. Before applying a result, check whether the target still exists and
whether intervening input should cancel the paste or be processed after it.
The runtime does not choose an editor's input-order or undo policy.

An asynchronous read is not an atomic snapshot at the moment of the key press.
Another application can change the clipboard before the native request is
serviced. Dropping the task discards the result, but a native read already in
progress may finish. No clipboard content is written or cleared by a read.

## Backends

| Backend | Read behavior |
| --- | --- |
| macOS | An OS-supplied `osascript` helper reads plain text with AppKit on the helper's main thread and returns UTF-8 bytes. The host's UI thread and pasteboard objects are not used for the transfer. Helper startup has a cost; if it cannot run or access text, the task returns `None`. A helper still running after five seconds is terminated and reaped. |
| Windows | A background task uses a dedicated message-only window to read `CF_UNICODETEXT`, with bounded retries for a temporarily held clipboard lock. It does not enumerate or request image formats or metadata. |
| X11 | A background task requests text targets through the clipboard connection. Both local ownership and external selection transfers use the worker. |
| Wayland | The event-loop thread captures the current offer and sends a receive request. A background task reads its pipe with the existing transfer timeout and decodes UTF-8. In-process clipboard contents are shared until the worker reads them. |

Custom `Platform` implementations inherit a synchronous compatibility fallback.
Override `Platform::read_text_from_clipboard_async` to provide nonblocking native
acquisition. Unsupported/headless backends return no text. Browser clipboard
permission flows are outside this desktop API change.

`ClipboardItem::plain_text()` extracts only plain text entries from an existing
item. Unlike `text()`, it does not fall back to a file-path representation.

## Validation

Core tests cover asynchronous completion, replacement of clipboard contents,
cancellation, empty text, and exclusion of non-text entries. Native tests cover
Unicode transfer on macOS, Windows clipboard contention, and local/external X11
selection ownership. Wayland pipe tests cover deferred transfer, UTF-8 decoding,
line-ending preservation, and unsupported offers.

The macOS test uses the system clipboard and restores the supported item it
read beforehand. Run it in an isolated desktop session:

```sh
cargo test -p guic-gpui-macos --features font-kit --lib system_helper_reads_unicode_text_off_thread --locked -- --ignored
```

The Windows test runs with the existing native clipboard suite to avoid
competing test owners:

```sh
cargo test -p guic-gpui-windows --features test-support --lib platform::tests::test_clipboard --locked
```

Run the X11 test on an isolated server:

```sh
xvfb-run -a cargo test -p guic-gpui-linux --features test-support --lib native_text_clipboard_reads_on_worker --locked -- --ignored
```

Real applications should also test slow clipboard owners, clipboard replacement
during a pending paste, focus/document changes, and restricted helper execution.
