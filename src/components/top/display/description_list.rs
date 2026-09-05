use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

pub struct DescriptionList {
    pub term: String,
    pub details: String,
}
impl Default for DescriptionList {
    fn default() -> Self {
        Self {
            term: String::new(),
            details: String::new(),
        }
    }
}

impl DescriptionList {
    pub fn new(term: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            term: term.into(),
            details: details.into(),
        }
    }

    pub fn description_list(self, entries: Vec<DescriptionList>, theme: Theme) -> impl View {
        let rows: Vec<_> = entries
            .into_iter()
            .map(|entry| {
                floem::views::Stack::horizontal((
                    floem::views::Label::new(entry.term).style(move |s| {
                        s.font_size(13.0)
                            .font_weight(floem::text::FontWeight::BOLD)
                            .color(theme.foreground)
                            .width(140.0)
                            .flex_shrink(0.0)
                    }),
                    floem::views::Label::new(entry.details).style(move |s| {
                        s.font_size(13.0)
                            .color(theme.foreground_secondary)
                            .flex_grow(1.0)
                    }),
                ))
                .style(move |s| {
                    s.flex_row()
                        .items_start()
                        .gap(16.0)
                        .padding_xy(0.0, 8.0)
                        .border_bottom(1.0)
                        .border_bottom_color(theme.divider)
                })
            })
            .collect();

        floem::views::Stack::vertical_from_iter(rows).style(move |s| s.flex_col().width_full())
    }
}
