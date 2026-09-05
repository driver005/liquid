use floem::prelude::*;
use crate::theme::{Theme, ColorRole};

#[derive(Default, Clone)]
pub struct SelectItem {}

impl SelectItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item(
        self,
        label: impl Into<String>,
        is_selected: bool,
        theme: Theme,
        color: ColorRole,
    ) -> impl View {
        let scale = *theme.scale_for(color);
        let text_color = theme.foreground;
        let bg_hover = theme.content2;

        floem::views::Label::new(label.into())
            .style(move |s| {
                s.width_full()
                 .padding(8.0)
                 .border_radius(theme.radius_md)
                 .cursor(floem::style::CursorStyle::Pointer)
                 .background(if is_selected { scale.d500.with_alpha(0.15) } else { floem::peniko::Color::TRANSPARENT })
                 .color(if is_selected { scale.d400 } else { text_color })
                 .hover(move |s| {
                     if is_selected {
                         s
                     } else {
                         s.background(bg_hover)
                     }
                 })
            })
    }
}
