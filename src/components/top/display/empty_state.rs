use floem::prelude::*;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct EmptyState {}
impl EmptyState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn empty_state(
        self,
        icon: &str,
        title: &str,
        description: &str,
        action: Option<(&str, Box<dyn Fn() + 'static>)>,
        theme: Theme,
    ) -> impl View {
        let icon_text = icon.to_string();
        let title_text = title.to_string();
        let desc_text = description.to_string();

        let action_view = action.map(|(label, on_click)| {
            let label = label.to_string();
            theme.button_ui_kit(move || label.clone(), crate::components::base::button::button::ButtonVariant::Regular, crate::theme::ColorRole::Primary).on_click_stop(move |_| { on_click(); })
        });

        floem::views::Stack::vertical((
            floem::views::Label::new(icon_text).style(move |s| {
                s.font_size(48.0)
                    .color(theme.foreground_secondary)
                    .opacity(0.5)
            }),
            floem::views::Label::new(title_text).style(move |s| {
                s.font_size(18.0)
                    .font_weight(floem::text::FontWeight::BOLD)
                    .color(theme.foreground)
            }),
            floem::views::Label::new(desc_text).style(move |s| {
                s.font_size(14.0)
                    .color(theme.foreground_secondary)
                    .max_width(400.0)
                    .text_align(floem::text::Alignment::Center)
            }),
            action_view
                .map(|a| a.into_any())
                .unwrap_or_else(|| floem::views::Empty::new().into_any()),
        ))
        .style(move |s| {
            s.flex_col()
                .items_center()
                .justify_center()
                .gap(12.0)
                .padding(48.0)
                .width_full()
        })
    }
}
