use floem::prelude::*;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct LoadingOverlay {}
impl LoadingOverlay {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn loading_overlay(visible: RwSignal<bool>, theme: Theme) -> impl View {
        floem::views::Stack::new((
            floem::views::Empty::new()
                .style(move |s| s.width_full().height_full().background(theme.overlay)),
            floem::views::Stack::vertical((
                floem::views::Empty::new().style(move |s| {
                    s.size(40.0, 40.0)
                        .border_radius(9999.0)
                        .border(3.0)
                        .border_color(theme.border)
                        .border_top(3.0)
                        .border_top_color(theme.primary.d500)
                }),
                floem::views::Label::new("Loading...")
                    .style(move |s| s.font_size(14.0).color(theme.foreground).margin_top(12.0)),
            ))
            .style(|s| s.flex_col().items_center().justify_center()),
        ))
        .style(move |s| {
            let is_visible = visible.get();
            s.width_full()
                .height_full()
                .absolute()
                .inset_left(0.0)
                .inset_top(0.0)
                .z_index(150)
                .apply_if(!is_visible, |s| s.hide())
        })
    }

    pub fn loading_overlay_with_message(
        self,
        visible: RwSignal<bool>,
        message: &str,
        theme: Theme,
    ) -> impl View {
        let msg = message.to_string();

        floem::views::Stack::new((
            floem::views::Empty::new()
                .style(move |s| s.width_full().height_full().background(theme.overlay)),
            floem::views::Stack::vertical((
                floem::views::Empty::new().style(move |s| {
                    s.size(40.0, 40.0)
                        .border_radius(9999.0)
                        .border(3.0)
                        .border_color(theme.border)
                        .border_top(3.0)
                        .border_top_color(theme.primary.d500)
                }),
                floem::views::Label::new(msg)
                    .style(move |s| s.font_size(14.0).color(theme.foreground).margin_top(12.0)),
            ))
            .style(|s| s.flex_col().items_center().justify_center()),
        ))
        .style(move |s| {
            let is_visible = visible.get();
            s.width_full()
                .height_full()
                .absolute()
                .inset_left(0.0)
                .inset_top(0.0)
                .z_index(150)
                .apply_if(!is_visible, |s| s.hide())
        })
    }
}
