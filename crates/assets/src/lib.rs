//! Font assets embedded by guic-gpui backends and renderers.

/// IBM Plex Sans Regular.
pub const IBM_PLEX_SANS_REGULAR: &[u8] =
    include_bytes!("../fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf");
/// IBM Plex Sans Italic.
pub const IBM_PLEX_SANS_ITALIC: &[u8] =
    include_bytes!("../fonts/ibm-plex-sans/IBMPlexSans-Italic.ttf");
/// IBM Plex Sans SemiBold.
pub const IBM_PLEX_SANS_SEMIBOLD: &[u8] =
    include_bytes!("../fonts/ibm-plex-sans/IBMPlexSans-SemiBold.ttf");
/// IBM Plex Sans SemiBold Italic.
pub const IBM_PLEX_SANS_SEMIBOLD_ITALIC: &[u8] =
    include_bytes!("../fonts/ibm-plex-sans/IBMPlexSans-SemiBoldItalic.ttf");
/// Lilex Regular.
pub const LILEX_REGULAR: &[u8] = include_bytes!("../fonts/lilex/Lilex-Regular.ttf");
/// Lilex Bold.
pub const LILEX_BOLD: &[u8] = include_bytes!("../fonts/lilex/Lilex-Bold.ttf");
/// Lilex Italic.
pub const LILEX_ITALIC: &[u8] = include_bytes!("../fonts/lilex/Lilex-Italic.ttf");
/// Lilex Bold Italic.
pub const LILEX_BOLD_ITALIC: &[u8] = include_bytes!("../fonts/lilex/Lilex-BoldItalic.ttf");

/// All fonts bundled for the WebAssembly backend.
pub const BUNDLED_FONTS: &[&[u8]] = &[
    IBM_PLEX_SANS_REGULAR,
    IBM_PLEX_SANS_ITALIC,
    IBM_PLEX_SANS_SEMIBOLD,
    IBM_PLEX_SANS_SEMIBOLD_ITALIC,
    LILEX_REGULAR,
    LILEX_BOLD,
    LILEX_ITALIC,
    LILEX_BOLD_ITALIC,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_fonts_are_non_empty_and_parseable() {
        for font in BUNDLED_FONTS {
            assert!(!font.is_empty());
            ttf_parser::Face::parse(font, 0).expect("bundled font must be loadable");
        }
    }
}
