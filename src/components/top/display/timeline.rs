use floem::prelude::*;

use crate::theme::{ColorRole, Theme};

pub struct Timeline {
    pub title: String,
    pub description: Option<String>,
    pub color: ColorRole,
    pub timestamp: Option<String>,
}
impl Default for Timeline {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: None,
            color: crate::theme::ColorRole::Primary,
            timestamp: None,
        }
    }
}

impl Timeline {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            color: ColorRole::Primary,
            timestamp: None,
        }
    }
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
    pub fn color(mut self, color: ColorRole) -> Self {
        self.color = color;
        self
    }
    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    pub fn timeline(self, items: Vec<Timeline>, theme: Theme) -> impl View {
        let total = items.len();
        let views: Vec<_> = items
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let is_last = i == total - 1;
                let scale = *theme.scale_for(item.color);
                let title = item.title.clone();
                let desc = item.description.clone();
                let timestamp = item.timestamp.clone();

                let dot = floem::views::Empty::new().style({
                    let scale = scale.clone();
                    move |s| s.apply(theme.timeline_dot_style(scale.d500))
                });

                let line = floem::views::Empty::new()
                    .style(move |s| s.apply(theme.timeline_line_style(is_last)));

                let left = floem::views::Stack::vertical((dot, line))
                    .style(move |s| s.apply(theme.timeline_left_container_style()));

                let right = floem::views::Stack::vertical((
                    floem::views::Label::new(title)
                        .style(move |s| s.apply(theme.timeline_title_style())),
                    desc.map(|d| {
                        floem::views::Label::new(d)
                            .style(move |s| s.apply(theme.timeline_desc_style()))
                    })
                    .map(|v| v.into_any())
                    .unwrap_or_else(|| floem::views::Empty::new().into_any()),
                    timestamp
                        .map(|ts| {
                            floem::views::Label::new(ts)
                                .style(move |s| s.apply(theme.timeline_timestamp_style()))
                        })
                        .map(|v| v.into_any())
                        .unwrap_or_else(|| floem::views::Empty::new().into_any()),
                ))
                .style(move |s| s.apply(theme.timeline_right_container_style()));

                floem::views::Stack::horizontal((left, right))
                    .style(move |s| s.apply(theme.timeline_item_container_style()))
            })
            .collect();

        floem::views::Stack::vertical_from_iter(views)
            .style(move |s| s.apply(theme.timeline_list_container_style()))
    }
}
