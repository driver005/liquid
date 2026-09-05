use floem::prelude::*;

use crate::theme::Theme;

pub struct Card {
    pub bordered: bool,
    pub shadow: bool,
    pub radius: f32,
    pub padding: f32,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            bordered: true,
            shadow: false,
            radius: 16.0,
            padding: 16.0,
        }
    }
}

impl Card {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn card(self, content: impl View + 'static, theme: Theme) -> impl View {
        let style = theme.card_style(self.radius, self.bordered, self.shadow);

        content.style(move |s| s.apply(style.clone()).padding(self.padding))
    }

    pub fn card_header(self, title: &str, theme: Theme) -> impl View {
        floem::views::Label::new(title.to_string())
            .style(move |s| s.apply(theme.card_header_style()))
    }

    pub fn card_body(self, content: impl View + 'static, theme: Theme) -> impl View {
        content.style(move |s| s.apply(theme.card_body_style()))
    }

    pub fn card_footer(self, content: impl View + 'static, theme: Theme) -> impl View {
        content.style(move |s| s.apply(theme.card_footer_style()))
    }
}
