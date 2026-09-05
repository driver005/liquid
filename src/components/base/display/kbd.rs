use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

/// A single keyboard key/shortcut badge, e.g. `Self::kbd("⌘", theme)`.
#[derive(Default, Clone)]
pub struct Kbd {}
impl Kbd {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn kbd(key: &str, theme: Theme) -> impl View {
        let key = key.to_string();

        floem::views::Label::new(key).style(move |s| {
            s.font_family("monospace")
                .font_size(12.0)
                .font_weight(floem::text::FontWeight::MEDIUM)
                .color(theme.foreground_secondary)
                .background(theme.content2)
                .border(1.0)
                .border_color(theme.border)
                .border_radius(theme.radius_sm)
                .padding_xy(6.0, 2.0)
        })
    }

    /// A sequence of keys shown together, e.g. `kbd_combo(&["⌘", "K"], theme)`.
    pub fn kbd_combo(self, keys: &[&str], theme: Theme) -> impl View {
        let views: Vec<_> = keys.iter().map(|k| Self::kbd(k, theme)).collect();

        floem::views::Stack::horizontal_from_iter(views)
            .style(|s| s.flex_row().items_center().gap(4.0))
    }
}
