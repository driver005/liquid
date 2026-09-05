use floem::prelude::*;

use crate::theme::{ColorRole, Theme};

#[derive(Default, Clone)]
pub struct Heatmap {}
impl Heatmap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn heatmap(
        self,
        data: Vec<Vec<f32>>,
        rows: usize,
        cols: usize,
        color: ColorRole,
        theme: Theme,
    ) -> impl View {
        let scale = *theme.scale_for(color);

        let row_views: Vec<_> = (0..rows)
            .map(|r| {
                let scale = scale;
                let cell_views: Vec<_> = (0..cols)
                    .map(|c| {
                        let scale = scale;
                        let val = data
                            .get(r)
                            .and_then(|row| row.get(c))
                            .copied()
                            .unwrap_or(0.0);
                        let alpha = val.clamp(0.0, 1.0);

                        floem::views::Empty::new().style({
                            move |s| s.apply(theme.heatmap_cell_style(alpha, scale.d500))
                        })
                    })
                    .collect();

                floem::views::Stack::horizontal_from_iter(cell_views)
                    .style(move |s| s.apply(theme.heatmap_row_style()))
            })
            .collect();

        floem::views::Stack::vertical_from_iter(row_views)
            .style(move |s| s.apply(theme.heatmap_container_style()))
    }

    pub fn heatmap_legend(self, color: ColorRole, theme: Theme) -> impl View {
        let scale = *theme.scale_for(color);

        let cells: Vec<_> = (0..=4)
            .map(|i| {
                let scale = scale;
                let alpha = i as f32 / 4.0;
                floem::views::Empty::new().style({
                    let scale = scale.clone();
                    move |s| s.apply(theme.heatmap_cell_style(alpha, scale.d500))
                })
            })
            .collect();

        floem::views::Stack::horizontal((
            floem::views::Label::new("Less")
                .style(move |s| s.apply(theme.heatmap_legend_label_style())),
            floem::views::Stack::horizontal_from_iter(cells)
                .style(move |s| s.apply(theme.heatmap_legend_cells_style())),
            floem::views::Label::new("More")
                .style(move |s| s.apply(theme.heatmap_legend_label_style())),
        ))
        .style(move |s| s.apply(theme.heatmap_legend_container_style()))
    }
}
