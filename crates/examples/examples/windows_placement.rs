//! Manual native window placement probe. Run with saved, fallback, or maximized.

use gpui::{
    App, Bounds, Context, Pixels, Render, Window, WindowBounds, WindowOptions, div, point,
    prelude::*, px, rgb, size,
};

struct PlacementProbe {
    label: String,
    expected: Bounds<Pixels>,
}

impl Render for PlacementProbe {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .bg(rgb(0x202020))
            .text_color(rgb(0xffffff))
            .child(self.label.clone())
            .child(format!("Expected restored bounds: {:?}", self.expected))
            .child(format!("Current bounds: {:?}", window.bounds()))
            .child(format!("Current scale: {}", window.scale_factor()))
            .child("Minimize, restore, maximize, and restore again.")
            .child("Check that the title bar and borders remain accessible.")
            .child(
                div()
                    .font_family("Segoe UI Emoji")
                    .text_3xl()
                    .child("🫠 🥹 🧗 🏋 🚀 🥺"),
            )
    }
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "saved".into());
    assert!(
        matches!(mode.as_str(), "saved" | "fallback" | "maximized"),
        "expected saved, fallback, or maximized"
    );
    gpui_platform::application().run(move |cx: &mut App| {
        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        for display in cx.displays() {
            let saved = Bounds::new(
                display.visible_bounds().origin + point(px(80.0), px(80.0)),
                size(px(640.0), px(360.0)),
            );
            let expected = if mode == "fallback" {
                display.default_bounds()
            } else {
                saved
            };
            let requested = if mode == "fallback" {
                Bounds::new(point(px(-100000.0), px(-100000.0)), saved.size)
            } else {
                saved
            };
            let window_bounds = if mode == "maximized" {
                WindowBounds::Maximized(requested)
            } else {
                WindowBounds::Windowed(requested)
            };
            let label = format!("{mode} on display {:?}", display.id());
            println!("{label}: expected restored bounds {expected:?}");
            cx.open_window(
                WindowOptions {
                    display_id: Some(display.id()),
                    window_bounds: Some(window_bounds),
                    focus: false,
                    ..Default::default()
                },
                |_, cx| cx.new(|_| PlacementProbe { label, expected }),
            )
            .unwrap();
        }
    });
}
