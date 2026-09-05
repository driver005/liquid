use crate::theme::Theme;
use floem::prelude::*;

#[derive(Default, Clone)]
pub struct Rating {}
impl Rating {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rating(value: RwSignal<usize>, max: usize, theme: Theme) -> impl View {
        let stars: Vec<_> = (0..max)
            .map(move |i| {
                let val_sig = value;
                let star_index = i + 1;

                floem::views::Label::derived(move || {
                    {
                        if val_sig.get() >= star_index {
                            "★"
                        } else {
                            "☆"
                        }
                    }
                    .to_string()
                })
                .style(move |s| {
                    let is_active = val_sig.get() >= star_index;
                    s.font_size(24.0)
                        .color(if is_active {
                            theme.warning.d500
                        } else {
                            theme.content3
                        })
                        .cursor(floem::style::CursorStyle::Pointer)
                        .hover(move |s| s.color(theme.warning.d400))
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    value.set(star_index)
                })
            })
            .collect();

        floem::views::Stack::horizontal(stars).style(move |s| s.flex_row().gap(2.0))
    }

    pub fn rating_with_label(self, value: RwSignal<usize>, max: usize, theme: Theme) -> impl View {
        floem::views::Stack::horizontal((
            Self::rating(value, max, theme),
            floem::views::Label::derived(move || {
                let v = value.get();
                format!("{}/{}", v, max)
            })
            .style(move |s| {
                s.font_size(13.0)
                    .color(theme.foreground_secondary)
                    .margin_left(8.0)
            }),
        ))
        .style(move |s| s.flex_row().items_center().gap(4.0))
    }

    pub fn half_rating(self, value: RwSignal<f32>, max: usize, theme: Theme) -> impl View {
        let stars: Vec<_> = (0..max)
            .map(move |i| {
                let val_sig = value;
                let star_index = (i + 1) as f32;

                floem::views::Label::derived(move || {
                    {
                        let v = val_sig.get();
                        if v >= star_index {
                            "★"
                        } else if v >= star_index - 0.5 {
                            "⯨"
                        } else {
                            "☆"
                        }
                    }
                    .to_string()
                })
                .style(move |s| {
                    let v = val_sig.get();
                    let is_half = v >= star_index - 0.5 && v < star_index;
                    let is_full = v >= star_index;
                    s.font_size(24.0)
                        .color(if is_full {
                            theme.warning.d500
                        } else if is_half {
                            theme.warning.d500
                        } else {
                            theme.content3
                        })
                        .cursor(floem::style::CursorStyle::Pointer)
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    value.set(star_index)
                })
            })
            .collect();

        floem::views::Stack::horizontal(stars).style(move |s| s.flex_row().gap(2.0))
    }
}
