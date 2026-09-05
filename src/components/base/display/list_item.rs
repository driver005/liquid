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
        let mut builder = crate::components::base::input::select_item::SelectItem::new();
        
        if let Some(ic) = data.icon { builder = builder.icon(ic); }
        if let Some(desc) = data.subtitle { builder = builder.description(desc); }
        if let Some(trail) = data.trailing { builder = builder.trailing(trail); }
        
        let view = builder.item(data.title, || false, theme.clone(), crate::theme::ColorRole::Primary);
        
        if let Some(on_click) = on_click {
            view.on_event_stop(floem::event::listener::Click, move |_, _| on_click()).into_any()
        } else {
            view.into_any()
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
