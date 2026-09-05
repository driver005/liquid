use floem::prelude::*;
use crate::theme::{Theme, ColorRole};
use floem::style::CursorStyle;

#[derive(Default, Clone)]
pub struct TextInput {}

impl TextInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text_input(self, rw_signal: RwSignal<String>, role: ColorRole, theme: Theme) -> impl View {
        let accent = theme.scale_for(role);
        let bg_color = theme.background;
        let fg_color = theme.foreground;
        let scale_d500 = accent.d500;
        let scale_d600 = accent.d600;

        floem::views::text_input(rw_signal).style(move |s| {
            s.width_full()
                .color(fg_color)
                .background(bg_color)
                .border(1.0)
                .border_color(fg_color.with_alpha(0.5))
                .border_radius(5.0)
                .font_size(14.0)
                .cursor(CursorStyle::Text)
                .cursor_color(fg_color.with_alpha(0.5))
                .height(36.0)
                .padding_horiz(15.0)
                .items_center().justify_center()
                .disabled(|s| {
                    s.background(scale_d500.with_alpha(0.3))
                     .color(fg_color.with_alpha(0.5))
                     .cursor(CursorStyle::Default)
                })
                .hover(move |s| {
                    s.border_color(scale_d500)
                })
                .focus(move |s| {
                    s.border_color(scale_d600)
                })
        })
    }
}
