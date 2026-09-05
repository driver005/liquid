use floem::prelude::*;
use crate::theme::{Theme, ColorRole};
use floem::style::CursorStyle;

impl Theme {
    pub fn text_input_uikit(&self, rw_signal: RwSignal<String>, role: ColorRole) -> impl View {
        let accent = self.scale_for(role);
        let bg_color = self.background;
        let fg_color = self.foreground;
        let scale_d500 = accent.d500;
        let scale_d600 = accent.d600;

        let input = floem::views::text_input(rw_signal).style(move |s| {
            s.width_full()
                .color(fg_color)
                .background(floem::peniko::Color::TRANSPARENT)
                .border(0.0)
                .font_size(14.0)
                .cursor(CursorStyle::Text)
                .cursor_color(fg_color.with_alpha(0.5))
        });

        floem::views::Stack::horizontal((input,)).style(move |s| {
            s.background(bg_color)
                .border(1.0)
                .border_color(fg_color.with_alpha(0.5))
                .border_radius(5.0)
                .padding_horiz(15.0)
                .height(36.0)
                .items_center() // Physically center the text_input child
                .disabled(|s| {
                    s.background(scale_d500.with_alpha(0.3))
                        .cursor(CursorStyle::Default)
                })
                .hover(move |s| {
                    s.border_color(scale_d500)
                })
                .focus_within(move |s| {
                    s.border_color(scale_d600)
                })
        })
    }
}
