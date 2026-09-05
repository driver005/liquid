use floem::prelude::*;

use crate::theme::{ColorRole, Theme, Variant};

#[derive(Default, Clone)]
pub struct Alert {}
impl Alert {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alert(
        self,
        title: &str,
        message: &str,
        theme: Theme,
        color: ColorRole,
        variant: Variant,
    ) -> impl View {
        let style = theme.alert_style(color, variant);
        let title_text = title.to_string();
        let message_text = message.to_string();

        let icon = match color {
            ColorRole::Success => "✓",
            ColorRole::Warning => "⚠",
            ColorRole::Danger => "✕",
            _ => "ℹ",
        };

        floem::views::Stack::vertical((
            floem::views::Stack::horizontal((
                floem::views::Label::new(icon)
                    .style(move |s| s.apply(theme.alert_icon_style(color))),
                floem::views::Label::new(title_text)
                    .style(move |s| s.apply(theme.alert_title_style(color))),
            ))
            .style(move |s| s.apply(theme.alert_header_container_style())),
            floem::views::Label::new(message_text)
                .style(move |s| s.apply(theme.alert_message_style())),
        ))
        .style(move |s| s.apply(style.clone()).flex_col())
    }

    pub fn dismissible_alert(
        self,
        title: &str,
        message: &str,
        theme: Theme,
        color: ColorRole,
        visible: RwSignal<bool>,
    ) -> impl View {
        let style = theme.alert_style(color, Variant::Flat);
        let title_text = title.to_string();
        let message_text = message.to_string();

        let icon = match color {
            ColorRole::Success => "✓",
            ColorRole::Warning => "⚠",
            ColorRole::Danger => "✕",
            _ => "ℹ",
        };

        floem::views::Stack::horizontal((
            floem::views::Stack::vertical((floem::views::Label::new(icon)
                .style(move |s| s.apply(theme.alert_icon_style(color))),))
            .style(move |s| s.apply(theme.alert_dismissible_icon_container_style())),
            floem::views::Stack::vertical((
                floem::views::Label::new(title_text.clone())
                    .style(move |s| s.apply(theme.alert_title_style(color))),
                floem::views::Label::new(message_text.clone())
                    .style(move |s| s.apply(theme.alert_dismissible_message_style())),
            ))
            .style(move |s| s.apply(theme.alert_dismissible_content_container_style())),
            floem::views::Empty::new()
                .style(move |s| s.apply(theme.alert_dismissible_spacer_style())),
            floem::views::Label::new("✕")
                .style(move |s| {
                    s.apply(theme.alert_close_style())
                        .hover(move |s| s.apply(theme.alert_close_hover_style()))
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    visible.set(false)
                }),
        ))
        .style(move |s| s.apply(style.clone()).flex_row().items_start().gap(8.0))
    }
}
