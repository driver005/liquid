use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct SearchInput {}
impl SearchInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search_input(self, value: RwSignal<String>, theme: Theme) -> impl View {
        floem::views::Stack::horizontal((
            floem::views::Label::new("🔍").style(move |s| {
                s.font_size(14.0)
                    .color(theme.foreground_secondary)
                    .padding_left(12.0)
            }),
            floem::views::TextInput::new(value)
                .placeholder("Search...")
                .style(move |s| {
                    s.width_full()
                        .padding_xy(8.0, 10.0)
                        .border(0.0)
                        .border_color(Color::TRANSPARENT)
                        .background(Color::TRANSPARENT)
                        .color(theme.foreground)
                        .font_size(14.0)
                }),
            floem::views::Label::new("✕")
                .style(move |s| {
                    let has_text = !value.get().is_empty();
                    s.font_size(14.0)
                        .color(theme.foreground_secondary)
                        .padding_right(12.0)
                        .cursor(floem::style::CursorStyle::Pointer)
                        .apply_if(!has_text, |s| s.hide())
                        .hover(|s| s.color(theme.foreground))
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    value.set(String::new())
                }),
        ))
        .style(move |s| {
            s.flex_row()
                .items_center()
                .apply(theme.search_input_style())
        })
    }

    pub fn search_input_with_results(
        self,
        value: RwSignal<String>,
        results: Vec<String>,
        on_select: impl Fn(&str) + 'static + Clone,
        theme: Theme,
    ) -> impl View {
        let on_select = std::sync::Arc::new(on_select);
        let _open = floem::reactive::RwSignal::new(false);

        let search = Self::default().search_input(value, theme);

        let result_views: Vec<_> = results
            .into_iter()
            .map(|r| {
                let on_select = on_select.clone();
                let val_sig = value;
                floem::views::Label::new(r.clone())
                    .style(move |s| {
                        s.font_size(14.0)
                            .color(theme.foreground)
                            .padding_xy(12.0, 8.0)
                            .width_full()
                            .cursor(floem::style::CursorStyle::Pointer)
                            .hover(|s| s.background(theme.content2))
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| {
                        on_select(&r);
                        val_sig.set(r.clone());
                    })
            })
            .collect();

        let results_panel = floem::views::Stack::vertical_from_iter(result_views).style(move |s| {
            s.flex_col()
                .width_full()
                .border(1.0)
                .border_color(theme.border)
                .border_top(0.0)
                .border_top_color(Color::TRANSPARENT)
                .border_bottom_left_radius(theme.radius_md)
                .border_bottom_right_radius(theme.radius_md)
                .background(theme.background_elevated)
                .box_shadow(crate::theme::make_shadow(
                    4.0,
                    12.0,
                    theme.shadow_color,
                    8.0,
                ))
                .absolute()
                .inset_top(100.0)
                .inset_left(0.0)
                .z_index(40)
        });

        floem::views::Stack::new((search, results_panel))
    }
}
