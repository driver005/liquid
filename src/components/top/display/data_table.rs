use floem::peniko::Color;
use floem::prelude::*;
use floem::AnyView;

use crate::prelude::Table;
use crate::theme::Theme;

#[derive(Default)]
pub struct DataTable {
    pub selectable: bool,
    pub sortable: bool,
    pub page_size: Option<usize>,
}

impl DataTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn data_table(
        self,
        columns: Vec<Table>,
        rows: Vec<Vec<String>>,
        selected_row: RwSignal<Option<usize>>,
        sort_column: RwSignal<Option<usize>>,
        sort_ascending: RwSignal<bool>,
        theme: Theme,
    ) -> impl View {
        let header_cells: Vec<_> = columns
            .iter()
            .enumerate()
            .map(|(i, col)| {
                let label = col.label.clone();
                let width = col.width;
                let sort_col = sort_column;
                let sort_asc = sort_ascending;

                let label_view = floem::views::Label::new(label).style({
                    move |s| {
                        let mut s = s
                            .apply(theme.data_table_header_label_style())
                            .flex_grow(1.0);
                        if let Some(w) = width {
                            s = s.width(w).flex_shrink(0.0);
                        }
                        s
                    }
                });

                let sort_indicator = floem::views::Label::derived(move || {
                    if sort_col.get() == Some(i) {
                        if sort_asc.get() {
                            "▲"
                        } else {
                            "▼"
                        }
                    } else {
                        ""
                    }
                    .to_string()
                })
                .style(move |s| {
                    s.font_size(10.0)
                        .color(theme.foreground_secondary)
                        .padding_left(4.0)
                });

                let cell = floem::views::Stack::horizontal((label_view, sort_indicator))
                    .style(move |s| s.apply(theme.data_table_cell_container_style()));

                if self.sortable {
                    cell.on_event_stop(floem::event::listener::Click, move |_, _| {
                        if sort_col.get() == Some(i) {
                            sort_asc.update(|a| *a = !*a);
                        } else {
                            sort_col.set(Some(i));
                            sort_asc.set(true);
                        }
                    })
                } else {
                    cell
                }
            })
            .collect();

        let select_header = if self.selectable {
            let _all_selected = floem::reactive::RwSignal::new(false);
            Some(
                floem::views::Empty::new()
                    .style(move |s| {
                        s.size(16.0, 16.0)
                            .border(2.0)
                            .border_color(theme.border)
                            .border_radius(4.0)
                            .cursor(floem::style::CursorStyle::Pointer)
                    })
                    .style(move |s| s.apply(theme.data_table_checkbox_container_style())),
            )
        } else {
            None
        };

        let mut header_items: Vec<AnyView> = Vec::new();
        if let Some(sh) = select_header {
            header_items.push(sh.into_any());
        }
        for hc in header_cells {
            header_items.push(hc.into_any());
        }

        let header = floem::views::Stack::horizontal_from_iter(header_items)
            .style(move |s| s.flex_row().width_full().background(theme.content2));

        let row_views: Vec<_> = rows
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                let selected_sig = selected_row;

                let mut cells: Vec<AnyView> = Vec::new();

                if self.selectable {
                    let selected_sig2 = selected_row;
                    cells.push(
                        floem::views::Empty::new()
                            .style(move |s| {
                                let is_sel = selected_sig2.get() == Some(i);
                                s.size(16.0, 16.0)
                                    .border(2.0)
                                    .border_color(if is_sel {
                                        theme.primary.d500
                                    } else {
                                        theme.border
                                    })
                                    .border_radius(4.0)
                                    .background(if is_sel {
                                        theme.primary.d500
                                    } else {
                                        Color::TRANSPARENT
                                    })
                                    .cursor(floem::style::CursorStyle::Pointer)
                            })
                            .style({
                                move |s| s.apply(theme.data_table_checkbox_container_style())
                            })
                            .into_any(),
                    );
                }

                for (j, cell_text) in row.into_iter().enumerate() {
                    let width = columns.get(j).and_then(|c| c.width);
                    cells.push(
                        floem::views::Label::new(cell_text)
                            .style(move |s| {
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
                            .into_any(),
                    );
                }

                floem::views::Stack::horizontal_from_iter(cells)
                    .style(move |s| {
                        let is_sel = selected_sig.get() == Some(i);
                        let bg = if is_sel {
                            theme.primary.d50
                        } else if i % 2 == 0 {
                            theme.background_elevated
                        } else {
                            theme.content2
                        };
                        s.flex_row()
                            .width_full()
                            .background(bg)
                            .border_bottom(1.0)
                            .border_bottom_color(theme.divider)
                            .cursor(floem::style::CursorStyle::Pointer)
                            .hover(|s| s.background(theme.primary.d50))
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| {
                        selected_row.update(|s| {
                            if *s == Some(i) {
                                *s = None
                            } else {
                                *s = Some(i)
                            }
                        });
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
