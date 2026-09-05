use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct Navbar {}
impl Navbar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn navbar(
        self,
        brand: &str,
        links: Vec<(&str, Box<dyn Fn() + 'static>)>,
        theme: Theme,
    ) -> impl View {
        let brand_text = brand.to_string();

        let nav_links: Vec<_> = links
            .into_iter()
            .map(|(label, on_click)| {
                let label = label.to_string();
                floem::views::Label::new(label)
                    .style(move |s| {
                        s.color(theme.foreground_secondary)
                            .font_size(14.0)
                            .padding_xy(12.0, 6.0)
                            .border_radius(theme.radius_sm)
                            .cursor(floem::style::CursorStyle::Pointer)
                            .hover(|s| s.color(theme.foreground).background(theme.content2))
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| on_click())
            })
            .collect();

        floem::views::Stack::horizontal((
            floem::views::Label::new(brand_text).style(move |s| {
                s.font_size(18.0)
                    .font_weight(floem::text::FontWeight::BOLD)
                    .color(theme.foreground)
            }),
            floem::views::Stack::horizontal_from_iter(nav_links)
                .style(|s| s.flex_row().gap(4.0).margin_left(24.0)),
            floem::views::Empty::new().style(|s| s.flex_grow(1.0)),
        ))
        .style(move |s| {
            s.flex_row()
                .items_center()
                .width_full()
                .padding_xy(24.0, 12.0)
                .background(theme.background)
                .border_bottom(1.0)
                .border_bottom_color(theme.border)
        })
    }
}
