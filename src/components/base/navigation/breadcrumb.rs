use floem::prelude::*;
use floem::AnyView;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct Breadcrumb {}
impl Breadcrumb {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn breadcrumb(
        self,
        items: Vec<(&str, Box<dyn Fn() + 'static>)>,
        theme: Theme,
    ) -> impl View {
        let last_index = Self::items_len(&items);
        let mut views: Vec<AnyView> = Vec::new();
        for (i, (label, on_click)) in items.into_iter().enumerate() {
            let label = label.to_string();
            let is_last = i == last_index;

            views.push(
                floem::views::Label::new(label)
                    .style({
                        move |s| {
                            s.apply(theme.breadcrumb_item_style(is_last))
                                .hover(move |s| s.apply(theme.breadcrumb_item_hover_style()))
                        }
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| on_click())
                    .into_any(),
            );

            if !is_last {
                views.push(
                    floem::views::Label::new("/")
                        .style(move |s| s.apply(theme.breadcrumb_separator_style()))
                        .into_any(),
                );
            }
        }

        floem::views::Stack::horizontal_from_iter(views)
            .style(move |s| s.apply(theme.breadcrumb_container_style()))
    }

    fn items_len<T>(v: &[T]) -> usize {
        v.len().saturating_sub(1)
    }

    pub fn breadcrumb_simple(self, items: Vec<&str>, theme: Theme) -> impl View {
        let total = items.len();

        let mut views: Vec<AnyView> = Vec::new();
        for (i, label) in items.into_iter().enumerate() {
            let is_last = i == total - 1;
            let label = label.to_string();

            views.push(
                floem::views::Label::new(label)
                    .style(move |s| s.apply(theme.breadcrumb_item_style(is_last)))
                    .into_any(),
            );

            if !is_last {
                views.push(
                    floem::views::Label::new("/")
                        .style(move |s| s.apply(theme.breadcrumb_separator_style()))
                        .into_any(),
                );
            }
        }

        floem::views::Stack::horizontal_from_iter(views)
            .style(move |s| s.apply(theme.breadcrumb_container_style()))
    }
}
