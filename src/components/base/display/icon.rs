use floem::prelude::*;
use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct SvgIcon {}

impl SvgIcon {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn icon(self, svg_str: impl Into<String>, size: f32, theme: Theme) -> impl View {
        floem::views::svg(svg_str.into())
            .style(move |s| {
                s.width(size)
                    .height(size)
                    .color(theme.foreground).items_center().justify_center()
            })
    }
}
