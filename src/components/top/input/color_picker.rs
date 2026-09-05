use crate::style::StyleExt;
use floem::peniko::Color;
use floem::prelude::*;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct ColorPicker {}
impl ColorPicker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color_picker(self, selected: RwSignal<Color>, theme: Theme) -> impl View {
        let open = floem::reactive::RwSignal::new(false);

        let swatches = vec![
            Color::from_rgb8(239, 68, 68),
            Color::from_rgb8(245, 158, 11),
            Color::from_rgb8(16, 185, 129),
            Color::from_rgb8(99, 102, 241),
            Color::from_rgb8(168, 85, 247),
            Color::from_rgb8(236, 72, 153),
            Color::from_rgb8(34, 197, 94),
            Color::from_rgb8(14, 165, 233),
            Color::from_rgb8(100, 116, 139),
            Color::from_rgb8(0, 0, 0),
            Color::from_rgb8(255, 255, 255),
            Color::from_rgb8(120, 120, 120),
        ];

        let trigger = floem::views::Empty::new()
            .style(move |s| s.apply(theme.color_picker_trigger_style(selected.get())))
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                open.update(|o| *o = !*o)
            });

        let swatch_views: Vec<_> = swatches
            .into_iter()
            .map(|color| {
                let selected_sig = selected;
                floem::views::Empty::new()
                    .style(move |s| {
                        let is_sel = selected_sig.get() == color;
                        s.apply(theme.color_picker_swatch_style(color, is_sel))
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| {
                        selected.set(color);
                        open.set(false);
                    })
            })
            .collect();

        let panel = floem::views::Stack::vertical((
            floem::views::Label::new("Choose color")
                .style(move |s| s.apply(theme.color_picker_header_style())),
            floem::views::Stack::vertical_from_iter({
                let mut rows = Vec::new();
                let mut iter = swatch_views.into_iter();
                loop {
                    let chunk: Vec<_> = iter.by_ref().take(6).collect();
                    if chunk.is_empty() {
                        break;
                    }
                    rows.push(
                        floem::views::Stack::horizontal_from_iter(chunk)
                            .style(move |s| s.apply(theme.color_picker_row_style())),
                    );
                }
                rows
            })
            .style(move |s| s.apply(theme.color_picker_swatch_grid_style())),
        ))
        .style({
            move |s| {
                let is_open = open.get();
                s.apply(theme.color_picker_panel_style())
                    .overlay_fade(is_open)
                    .overlay_scale_in(is_open)
            }
        });

        floem::views::Stack::new((trigger, panel))
    }
}
