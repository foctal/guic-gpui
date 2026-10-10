# Native file clipboard

Use `App::supports_file_clipboard(operation)` to check backend support and
`App::write_files_to_clipboard(paths, operation)` to publish a file list.
`FileClipboardOperation::{Copy, Move}` is separate from `ExternalPaths`.
The list must be nonempty, with absolute paths and no NUL bytes. Validation does
not access the filesystem. macOS additionally rejects non-UTF-8 paths.

`FileClipboardError` distinguishes `Unsupported`, `InvalidPaths`, and
`Unavailable`. Unsupported operations leave the clipboard alone. Native
failures can leave partially replaced contents. Success means the write was
accepted locally, not that another application pasted or moved anything.
Wayland ownership requests are asynchronous and still subject to compositor
policy. The application/recipient owns file operations; GPUI never deletes
source files and exposes no paste-completion acknowledgement.

| Backend | Copy | Move | Native representation |
| --- | --- | --- | --- |
| macOS | Yes | Unsupported | NSURL pasteboard items plus legacy filenames |
| Windows | Yes | Yes | CF_HDROP, UTF-16 double-NUL path list, Preferred DropEffect |
| X11 | Yes | Yes | text/uri-list, GNOME copied-files, KDE cut-selection |
| Wayland | Yes | Yes | Same MIME formats; requires a data device, input focus and an input serial |
| Test, headless, web, other default backends | Unsupported | Unsupported | No native file writer |

Legacy `write_to_clipboard` remains a best-effort API: file entries request copy
and failures are logged. Prefer the explicit file API for portable behavior and
errors. File lists take precedence over other entries in Linux/Windows legacy
writes. macOS also retains explicit string entries. The read API does not
currently report native move intent; consumer-owned internal cut state must
remain separate from the returned path list.

The URI serializer percent-encodes reserved characters, Unicode, and newlines;
URI lists use CRLF and GNOME lists use LF with a copy/cut prefix. Native formats
are based on [Windows Shell clipboard formats](https://learn.microsoft.com/en-us/windows/win32/shell/clipboard),
[Apple pasteboard writing](https://developer.apple.com/documentation/appkit/nspasteboard/writeobjects(_:)),
and [GNOME's clipboard implementation](https://mail.gnome.org/archives/commits-list/2022-January/msg01597.html).

## Validation

Automated coverage checks URI encoding, copy/move intent, invalid paths,
unsupported writes preserving existing clipboard contents, and macOS native
pasteboard items containing multiple Unicode file URLs. macOS tests use a
unique OS pasteboard and inspect its native items directly; they do not depend
on an in-process clipboard cache. Replacement by externally serialized data is
also covered.

Windows native tests exercise the headless platform's clipboard owner, read back
multiple Unicode paths through CF_HDROP, inspect Preferred DropEffect for copy
and move, and verify that invalid writes preserve the existing contents. They
also cover legacy file writes and replacement by a separate text writer. These
checks run within the existing text clipboard test to avoid competing test
owners. Run them on Windows with:

```sh
cargo test -p guic-gpui-windows --features test-support --lib platform::tests::test_clipboard --locked
```

Linux tests cover Wayland MIME payload delivery through file descriptors,
including copy/move intent and replacement with text. The native X11 test uses
a separate client connection to request all three file formats, checks that
invalid writes preserve the selection, and replaces it with an external text
selection. Run it on an isolated X server because it changes the clipboard:

```sh
cargo test -p guic-gpui-linux --features test-support --lib --locked
xvfb-run -a cargo test -p guic-gpui-linux --features test-support --lib native_file_clipboard_round_trip --locked -- --ignored
```

The Wayland payload test does not exercise compositor selection policy, focus,
or input serials; those still require native validation below.

Before release, verify actual file-manager paste with Finder, Explorer,
Nautilus and Dolphin (X11 and Wayland), including multiple Unicode paths,
copy/move behavior, clipboard replacement by another application, and missing
Wayland input focus/serial. Native file-manager interoperability is not implied
by serialization or in-process tests. Native validation must run on each target
platform; cross-compilation alone does not validate clipboard interoperability.
