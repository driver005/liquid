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

        floem::views::text_input(rw_signal).style(move |s| {
            s.background(bg_color)
                .border(1.0)
                .color(fg_color)
                .border_color(fg_color.with_alpha(0.5))
                .border_radius(5.0)
                .cursor(CursorStyle::Text)
                .cursor_color(fg_color.with_alpha(0.5))
                .height(36.0) // Give it a fixed, standard height
                .items_center() // Center vertically!
                .padding_horiz(15.0)
                .font_size(14.0)
                .line_height(1.0)
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
