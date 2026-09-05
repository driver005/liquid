use floem::peniko::Color;
use floem::prelude::*;

use crate::theme::Theme;

#[derive(Clone)]
pub struct Gallery {
    pub bg: Color,
    pub label: String,
}

impl Gallery {
    pub fn new(bg: Color, label: impl Into<String>) -> Self {
        Self {
            bg,
            label: label.into(),
        }
    }

    pub fn gallery(items: Vec<Gallery>, theme: Theme) -> impl View {
        let rows: Vec<_> = items
            .chunks(3)
            .map(|chunk| {
                let cells: Vec<_> = chunk
                    .iter()
                    .map(|item| {
                        let bg = item.bg;
                        let label = item.label.clone();
                        floem::views::Stack::vertical((
                            floem::views::Empty::new().style(move |s| {
                                s.width_full()
                                    .height(120.0)
                                    .background(bg)
                                    .border_radius(theme.radius_sm)
                            }),
                            floem::views::Label::new(label).style(move |s| {
                                s.font_size(12.0)
                                    .color(theme.foreground_secondary)
                                    .margin_top(6.0)
                            }),
                        ))
                        .style(|s| s.flex_col().gap(0.0).flex_grow(1.0))
                    })
                    .collect();
                floem::views::Stack::horizontal_from_iter(cells)
                    .style(|s| s.flex_row().gap(12.0).width_full())
            })
            .collect();

        floem::views::Stack::vertical_from_iter(rows)
            .style(move |s| s.flex_col().gap(12.0).width_full())
    }

    pub fn masonry_gallery(self, items: Vec<Gallery>, theme: Theme) -> impl View {
        let col1: Vec<_> = items.iter().step_by(2).cloned().collect();
        let col2: Vec<_> = items.iter().skip(1).step_by(2).cloned().collect();

        let make_col = |col_items: Vec<Gallery>, theme: Theme| {
            let views: Vec<_> = col_items
                .into_iter()
                .map(|item| {
                    let bg = item.bg;
                    let label = item.label;
                    floem::views::Stack::vertical((
                        floem::views::Empty::new().style(move |s| {
                            s.width_full()
                                .height(120.0)
                                .background(bg)
                                .border_radius(theme.radius_sm)
                        }),
                        floem::views::Label::new(label).style(move |s| {
                            s.font_size(12.0)
                                .color(theme.foreground_secondary)
                                .margin_top(6.0)
                        }),
                    ))
                    .style(|s| s.flex_col().gap(0.0).width_full())
                })
                .collect();
            floem::views::Stack::vertical_from_iter(views)
                .style(|s| s.flex_col().gap(12.0).flex_grow(1.0))
        };

        floem::views::Stack::horizontal((make_col(col1, theme), make_col(col2, theme)))
            .style(|s| s.flex_row().gap(12.0).width_full().items_start())
    }
}
