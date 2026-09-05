use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

pub struct ListItem {
    pub icon: Option<&'static str>,
    pub title: String,
    pub subtitle: Option<String>,
    pub trailing: Option<String>,
}

impl ListItem {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            icon: None,
            subtitle: None,
            trailing: None,
        }
    }
    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }
    pub fn trailing(mut self, trailing: impl Into<String>) -> Self {
        self.trailing = Some(trailing.into());
        self
    }

    pub fn list_item(
        data: ListItem,
        on_click: Option<Box<dyn Fn() + 'static>>,
        theme: Theme,
    ) -> impl View {
        let icon = data.icon.map(|s| s.to_string());
        let title = data.title.clone();
        let subtitle = data.subtitle.clone();
        let trailing = data.trailing.clone();

        let row = floem::views::Stack::horizontal((
            icon.map(|ic| {
                floem::views::Label::new(ic).style(move |s| {
                    s.font_size(20.0)
                        .color(theme.foreground_secondary)
                        .width(32.0)
                        .flex_row()
                        .items_center()
                        .justify_center()
                })
            })
            .map(|v| v.into_any())
            .unwrap_or_else(|| floem::views::Empty::new().into_any()),
            floem::views::Stack::vertical((
                floem::views::Label::new(title).style(move |s| {
                    s.font_size(14.0)
                        .font_weight(floem::text::FontWeight::BOLD)
                        .color(theme.foreground)
                }),
                subtitle
                    .map(|sub| {
                        floem::views::Label::new(sub).style(move |s| {
                            s.font_size(12.0)
                                .color(theme.foreground_secondary)
                                .margin_top(2.0)
                        })
                    })
                    .map(|v| v.into_any())
                    .unwrap_or_else(|| floem::views::Empty::new().into_any()),
            ))
            .style(|s| s.flex_col().gap(0.0).flex_grow(1.0)),
            trailing
                .map(|t| {
                    floem::views::Label::new(t).style(move |s| {
                        s.font_size(12.0)
                            .color(theme.foreground_secondary)
                            .padding_left(8.0)
                    })
                })
                .map(|v| v.into_any())
                .unwrap_or_else(|| floem::views::Empty::new().into_any()),
        ))
        .style(move |s| {
            s.flex_row()
                .items_center()
                .gap(12.0)
                .padding_xy(12.0, 10.0)
                .border_radius(theme.radius_sm)
                .transition_colors()
                .hover(|s| s.background(theme.content2))
                .cursor(floem::style::CursorStyle::Pointer)
        });

        if let Some(on_click) = on_click {
            row.on_event_stop(floem::event::listener::Click, move |_, _| on_click())
        } else {
            row
        }
    }

    pub fn list_view(self, items: Vec<ListItem>, theme: Theme) -> impl View {
        let views: Vec<_> = items
            .into_iter()
            .map(|item| Self::list_item(item, None, theme))
            .collect();

        floem::views::Stack::vertical_from_iter(views).style(move |s| {
            s.flex_col()
                .gap(2.0)
                .width_full()
                .border(1.0)
                .border_color(theme.border)
                .border_radius(theme.radius_md)
                .padding(4.0)
                .background(theme.background_elevated)
        })
    }
}
