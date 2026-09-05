use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::{ColorRole, Theme};

#[derive(Default, Clone)]
pub struct StatsCard {}
impl StatsCard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stats_card(
        self,
        label: &str,
        value: &str,
        trend: Option<(&str, bool)>,
        theme: Theme,
    ) -> impl View {
        let label_text = label.to_string();
        let value_text = value.to_string();

        let trend_view = trend.map(|(trend_str, is_positive)| {
            let arrow = if is_positive { "↑" } else { "↓" };
            let color = if is_positive {
                theme.success.d500
            } else {
                theme.danger.d500
            };
            floem::views::Label::new(format!("{} {}", arrow, trend_str)).style(move |s| {
                s.font_size(12.0)
                    .font_weight(floem::text::FontWeight::BOLD)
                    .color(color)
                    .padding_xy(6.0, 2.0)
                    .border_radius(6.0)
                    .background(if is_positive {
                        theme.success.d50
                    } else {
                        theme.danger.d50
                    })
            })
        });

        floem::views::Stack::vertical((
            floem::views::Label::new(label_text)
                .style(move |s| s.font_size(13.0).color(theme.foreground_secondary)),
            floem::views::Stack::horizontal((
                floem::views::Label::new(value_text).style(move |s| {
                    s.font_size(28.0)
                        .font_weight(floem::text::FontWeight::BOLD)
                        .color(theme.foreground)
                }),
                floem::views::Empty::new().style(|s| s.flex_grow(1.0)),
                trend_view
                    .map(|t| t.into_any())
                    .unwrap_or_else(|| floem::views::Empty::new().into_any()),
            ))
            .style(|s| s.flex_row().items_center().gap(8.0).margin_top(4.0)),
        ))
        .style(move |s| {
            s.flex_col()
                .gap(0.0)
                .padding(20.0)
                .border_radius(theme.radius_md)
                .border(1.0)
                .border_color(theme.border)
                .background(theme.background_elevated)
                .width(240.0)
        })
    }

    pub fn kpi_card(
        self,
        label: &str,
        value: &str,
        icon: &str,
        color: ColorRole,
        theme: Theme,
    ) -> impl View {
        let scale = *theme.scale_for(color);
        let label_text = label.to_string();
        let value_text = value.to_string();
        let _icon_text = icon.to_string();

        floem::views::Stack::horizontal((
            floem::views::Empty::new().style(move |s| {
                s.size(48.0, 48.0)
                    .border_radius(theme.radius_md)
                    .background(scale.d50)
                    .flex_row()
                    .items_center()
                    .justify_center()
            }),
            floem::views::Stack::vertical((
                floem::views::Label::new(label_text)
                    .style(move |s| s.font_size(13.0).color(theme.foreground_secondary)),
                floem::views::Label::new(value_text).style(move |s| {
                    s.font_size(22.0)
                        .font_weight(floem::text::FontWeight::BOLD)
                        .color(theme.foreground)
                }),
            ))
            .style(|s| s.flex_col().gap(2.0)),
        ))
        .style(move |s| {
            s.flex_row()
                .items_center()
                .gap(16.0)
                .padding(20.0)
                .border_radius(theme.radius_md)
                .border(1.0)
                .border_color(theme.border)
                .background(theme.background_elevated)
        })
    }
}
