use floem::prelude::*;

use crate::theme::Theme;

pub struct Table {
    pub label: String,
    pub width: Option<f32>,
}
impl Default for Table {
    fn default() -> Self {
        Self {
            label: String::new(),
            width: None,
        }
    }
}

impl Table {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            width: None,
        }
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn table(self, columns: Vec<Table>, rows: Vec<Vec<String>>, theme: Theme) -> impl View {
        let header_cells: Vec<_> = columns
            .iter()
            .map(|col| {
                let label = col.label.clone();
                let width = col.width;
                floem::views::Label::new(label).style(move |s| {
                    let mut s = s
                        .font_size(12.0)
                        .font_weight(floem::text::FontWeight::BOLD)
                        .color(theme.foreground_secondary)
                        .padding(12.0)
                        .flex_grow(1.0);
                    if let Some(w) = width {
                        s = s.width(w).flex_shrink(0.0);
                    }
                    s
                })
            })
            .collect();

        let header = floem::views::Stack::horizontal_from_iter(header_cells)
            .style(move |s| s.flex_row().width_full().background(theme.content2));

        let row_views: Vec<_> = rows
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                let cells: Vec<_> = row
                    .into_iter()
                    .enumerate()
                    .map(|(j, cell)| {
                        let width = columns.get(j).and_then(|c| c.width);
                        floem::views::Label::new(cell).style(move |s| {
                            let mut s = s
                                .font_size(14.0)
                                .color(theme.foreground)
                                .padding(12.0)
                                .flex_grow(1.0);
                            if let Some(w) = width {
                                s = s.width(w).flex_shrink(0.0);
                            }
                            s
                        })
                    })
                    .collect();

                floem::views::Stack::horizontal_from_iter(cells).style(move |s| {
                    let bg = if i % 2 == 0 {
                        theme.background_elevated
                    } else {
                        theme.content2
                    };
                    s.flex_row()
                        .width_full()
                        .background(bg)
                        .border_bottom(1.0)
                        .border_bottom_color(theme.divider)
                })
            })
            .collect();

        floem::views::Stack::vertical((
            header,
            floem::views::Stack::vertical_from_iter(row_views).style(|s| s.flex_col().width_full()),
        ))
        .style(move |s| {
            s.flex_col()
                .width_full()
                .border(1.0)
                .border_color(theme.border)
                .border_radius(theme.radius_md)
                .overflow_x(floem::taffy::style::Overflow::Hidden)
                .overflow_y(floem::taffy::style::Overflow::Hidden)
        })
    }
}
