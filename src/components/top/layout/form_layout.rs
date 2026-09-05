use floem::prelude::*;
use floem::AnyView;

use crate::theme::Theme;

pub struct FormLayout {
    pub label: String,
    pub field: AnyView,
    pub error: Option<String>,
    pub hint: Option<String>,
}

impl FormLayout {
    pub fn new(label: impl Into<String>, field: impl View + 'static) -> Self {
        Self {
            label: label.into(),
            field: field.into_any(),
            error: None,
            hint: None,
        }
    }
    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn form_layout(fields: Vec<FormLayout>, theme: Theme) -> impl View {
        let field_views: Vec<_> = fields
            .into_iter()
            .map(|field| {
                let label = field.label.clone();
                let error = field.error.clone();
                let hint = field.hint.clone();

                floem::views::Stack::vertical((
                    floem::views::Label::new(label).style(move |s| {
                        s.font_size(13.0)
                            .font_weight(floem::text::FontWeight::BOLD)
                            .color(theme.foreground)
                            .margin_bottom(6.0)
                    }),
                    field.field,
                    error
                        .map(|e| {
                            floem::views::Label::new(e).style(move |s| {
                                s.font_size(12.0).color(theme.danger.d500).margin_top(4.0)
                            })
                        })
                        .map(|v| v.into_any())
                        .unwrap_or_else(|| floem::views::Empty::new().into_any()),
                    hint.map(|h| {
                        floem::views::Label::new(h).style(move |s| {
                            s.font_size(12.0)
                                .color(theme.foreground_secondary)
                                .margin_top(4.0)
                        })
                    })
                    .map(|v| v.into_any())
                    .unwrap_or_else(|| floem::views::Empty::new().into_any()),
                ))
                .style(|s| s.flex_col().gap(0.0).width_full())
            })
            .collect();

        floem::views::Stack::vertical_from_iter(field_views)
            .style(move |s| s.flex_col().gap(16.0).width_full())
    }

    pub fn form_section(self, title: &str, fields: Vec<FormLayout>, theme: Theme) -> impl View {
        let title_text = title.to_string();

        floem::views::Stack::vertical((
            floem::views::Label::new(title_text).style(move |s| {
                s.font_size(16.0)
                    .font_weight(floem::text::FontWeight::BOLD)
                    .color(theme.foreground)
                    .margin_bottom(16.0)
            }),
            Self::form_layout(fields, theme),
        ))
        .style(move |s| {
            s.flex_col()
                .gap(0.0)
                .padding(24.0)
                .border(1.0)
                .border_color(theme.border)
                .border_radius(theme.radius_md)
                .background(theme.background_elevated)
                .width_full()
        })
    }
}
