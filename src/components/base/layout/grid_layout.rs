use floem::prelude::*;
use floem::AnyView;

use crate::theme::Theme;

pub struct GridLayout {
    pub columns: usize,
    pub gap: f32,
    pub min_column_width: Option<f32>,
}

impl Default for GridLayout {
    fn default() -> Self {
        Self {
            columns: 3,
            gap: 16.0,
            min_column_width: None,
        }
    }
}

fn chunk_owned(children: Vec<AnyView>, cols: usize) -> Vec<Vec<AnyView>> {
    let mut rows = Vec::new();
    let mut iter = children.into_iter();
    loop {
        let chunk: Vec<_> = iter.by_ref().take(cols).collect();
        if chunk.is_empty() {
            break;
        }
        rows.push(chunk);
    }
    rows
}

impl GridLayout {
    pub fn new() -> Self {
        Self::default()
    }
}
impl GridLayout {
    pub fn grid_layout(self, children: Vec<AnyView>) -> impl View {
        let cols = self.columns;
        let gap = self.gap;

        let rows: Vec<_> = chunk_owned(children, cols)
            .into_iter()
            .map(|chunk| {
                floem::views::Stack::horizontal_from_iter(chunk)
                    .style(move |s| s.flex_row().gap(gap).width_full())
            })
            .collect();

        floem::views::Stack::vertical_from_iter(rows)
            .style(move |s| s.flex_col().gap(gap).width_full())
    }

    pub fn responsive_grid(self, children: Vec<AnyView>, _theme: Theme, gap: f32) -> impl View {
        let rows: Vec<_> = chunk_owned(children, 3)
            .into_iter()
            .map(|chunk| {
                floem::views::Stack::horizontal_from_iter(chunk)
                    .style(move |s| s.flex_row().gap(gap).width_full())
            })
            .collect();

        floem::views::Stack::vertical_from_iter(rows)
            .style(move |s| s.flex_col().gap(gap).width_full())
    }

    pub fn auto_grid(self, children: Vec<AnyView>, _min_width: f32, gap: f32) -> impl View {
        let rows: Vec<_> = chunk_owned(children, 3)
            .into_iter()
            .map(|chunk| {
                floem::views::Stack::horizontal_from_iter(chunk).style(move |s| {
                    s.flex_row()
                        .gap(gap)
                        .width_full()
                        .flex_wrap(floem::taffy::style::FlexWrap::Wrap)
                })
            })
            .collect();

        floem::views::Stack::vertical_from_iter(rows)
            .style(move |s| s.flex_col().gap(gap).width_full())
    }
}
