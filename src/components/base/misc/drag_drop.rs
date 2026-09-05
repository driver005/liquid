use crate::style::StyleExt;
use crate::theme::Theme;
use floem::prelude::*;

#[derive(Default, Clone)]
pub struct DragDrop {}
impl DragDrop {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn drag_drop_list(items: RwSignal<Vec<String>>, theme: Theme) -> impl View {
        let dragging = floem::reactive::RwSignal::new(None::<usize>);

        let list = floem::views::dyn_stack(
            move || items.get(),
            move |item| item.clone(),
            move |item| {
                let items_sig = items;
                let drag_sig = dragging;
                let item_text = item.clone();
                let index = items_sig
                    .get_untracked()
                    .iter()
                    .position(|x| x == &item)
                    .unwrap_or(0);

                floem::views::Stack::horizontal((
                    floem::views::Label::new("≡").style(move |s| {
                        s.font_size(16.0)
                            .color(theme.foreground_secondary)
                            .cursor(floem::style::CursorStyle::Grab)
                            .opacity(0.5)
                    }),
                    floem::views::Label::new(item_text)
                        .style(move |s| s.font_size(14.0).color(theme.foreground).flex_grow(1.0)),
                ))
                .style(move |s| {
                    let is_dragging = drag_sig.get() == Some(index);
                    s.apply(theme.drag_drop_item_style(is_dragging))
                })
                .on_event_stop(floem::event::listener::PointerDown, move |_, _| {
                    dragging.set(Some(index));
                })
                .on_event_stop(floem::event::listener::PointerUp, move |_, _| {
                    dragging.set(None);
                })
            },
        );

        list.style(move |s| s.flex_col().gap(4.0).width_full())
    }

    pub fn reorderable_list(self, items: RwSignal<Vec<String>>, theme: Theme) -> impl View {
        let list = floem::views::dyn_stack(
            move || items.get(),
            move |item| item.clone(),
            move |item| {
                let items_sig = items;
                let item_text = item.clone();
                let index = items_sig
                    .get_untracked()
                    .iter()
                    .position(|x| x == &item)
                    .unwrap_or(0);

                floem::views::Stack::horizontal((
                    floem::views::Label::new("▲")
                        .style(move |s| {
                            s.font_size(14.0)
                                .color(theme.foreground_secondary)
                                .cursor(floem::style::CursorStyle::Pointer)
                                .hover(move |s| s.color(theme.foreground))
                        })
                        .on_event_stop(floem::event::listener::Click, move |_, _| {
                            if index > 0 {
                                items_sig.update(|v| v.swap(index, index - 1));
                            }
                        }),
                    floem::views::Label::new("▼")
                        .style(move |s| {
                            s.font_size(14.0)
                                .color(theme.foreground_secondary)
                                .cursor(floem::style::CursorStyle::Pointer)
                                .hover(move |s| s.color(theme.foreground))
                        })
                        .on_event_stop(floem::event::listener::Click, move |_, _| {
                            items_sig.update(|v| {
                                if index < v.len().saturating_sub(1) {
                                    v.swap(index, index + 1);
                                }
                            });
                        }),
                    floem::views::Label::new(item_text).style(move |s| {
                        s.font_size(14.0)
                            .color(theme.foreground)
                            .flex_grow(1.0)
                            .margin_left(8.0)
                    }),
                ))
                .style(move |s| {
                    s.flex_row()
                        .items_center()
                        .gap(4.0)
                        .width_full()
                        .padding_xy(12.0, 10.0)
                        .border(1.0)
                        .border_color(theme.border)
                        .border_radius(theme.radius_sm)
                        .background(theme.background_elevated)
                        .hover(move |s| s.background(theme.content2))
                })
            },
        );

        list.style(move |s| s.flex_col().gap(4.0).width_full())
    }
}
