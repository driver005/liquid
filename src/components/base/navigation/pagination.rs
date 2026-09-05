use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct Pagination {}
impl Pagination {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pagination(
        self,
        current: RwSignal<usize>,
        total_pages: usize,
        theme: Theme,
    ) -> impl View {
        let ripple_color = theme.foreground;

        let prev_btn = floem::views::Label::new("‹")
            .style(move |s| {
                s.font_size(16.0)
                    .color(theme.foreground_secondary)
                    .padding_xy(10.0, 6.0)
                    .border_radius(theme.radius_sm)
                    .border(1.0)
                    .border_color(theme.border)
                    .cursor(floem::style::CursorStyle::Pointer)
                    .transition_colors()
                    .hover(|s| s.background(theme.content2))
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                current.update(|c| {
                    if *c > 0 {
                        *c -= 1
                    }
                });
            });
        let prev_btn = crate::components::base::button::ripple::Ripple::ripple_target(prev_btn, ripple_color, theme.radius_md);

        let next_btn = floem::views::Label::new("›")
            .style(move |s| {
                s.font_size(16.0)
                    .color(theme.foreground_secondary)
                    .padding_xy(10.0, 6.0)
                    .border_radius(theme.radius_sm)
                    .border(1.0)
                    .border_color(theme.border)
                    .cursor(floem::style::CursorStyle::Pointer)
                    .transition_colors()
                    .hover(|s| s.background(theme.content2))
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                current.update(|c| {
                    if *c < total_pages.saturating_sub(1) {
                        *c += 1
                    }
                });
            });
        let next_btn = crate::components::base::button::ripple::Ripple::ripple_target(next_btn, ripple_color, theme.radius_md);

        let page_buttons: Vec<_> = (0..total_pages)
            .map(|page| {
                let current_sig = current;
                let btn = floem::views::Label::new((page + 1).to_string())
                    .style(move |s| {
                        let is_active = current_sig.get() == page;
                        s.font_size(13.0)
                            .padding_xy(10.0, 6.0)
                            .border_radius(theme.radius_sm)
                            .cursor(floem::style::CursorStyle::Pointer)
                            .background(if is_active {
                                theme.primary.d500
                            } else {
                                Color::TRANSPARENT
                            })
                            .color(if is_active {
                                Color::WHITE
                            } else {
                                theme.foreground_secondary
                            })
                            .border(1.0)
                            .border_color(if is_active {
                                theme.primary.d500
                            } else {
                                theme.border
                            })
                            .transition_colors()
                            .hover(|s| {
                                if is_active {
                                    s.background(theme.primary.d600)
                                } else {
                                    s.background(theme.content2)
                                }
                            })
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| current.set(page));
                crate::components::base::button::ripple::Ripple::ripple_target(btn, ripple_color, theme.radius_md)
            })
            .collect();

        floem::views::Stack::horizontal((
            prev_btn,
            floem::views::Stack::horizontal_from_iter(page_buttons)
                .style(|s| s.flex_row().gap(4.0)),
            next_btn,
        ))
        .style(|s| s.flex_row().items_center().gap(4.0))
    }

    pub fn simple_pagination(
        self,
        current: RwSignal<usize>,
        total: usize,
        theme: Theme,
    ) -> impl View {
        floem::views::Stack::horizontal((
            floem::views::Label::new("‹")
                .style(move |s| {
                    s.font_size(16.0)
                        .color(theme.foreground_secondary)
                        .padding_xy(10.0, 6.0)
                        .border_radius(theme.radius_sm)
                        .cursor(floem::style::CursorStyle::Pointer)
                        .hover(|s| s.color(theme.foreground))
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    current.update(|c| {
                        if *c > 0 {
                            *c -= 1
                        }
                    });
                }),
            floem::views::Label::derived(move || format!("{} / {}", current.get() + 1, total))
                .style(move |s| {
                    s.font_size(13.0)
                        .color(theme.foreground)
                        .padding_xy(12.0, 6.0)
                }),
            floem::views::Label::new("›")
                .style(move |s| {
                    s.font_size(16.0)
                        .color(theme.foreground_secondary)
                        .padding_xy(10.0, 6.0)
                        .border_radius(theme.radius_sm)
                        .cursor(floem::style::CursorStyle::Pointer)
                        .hover(|s| s.color(theme.foreground))
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    current.update(|c| {
                        if *c < total.saturating_sub(1) {
                            *c += 1
                        }
                    });
                }),
        ))
        .style(move |s| {
            s.flex_row()
                .items_center()
                .gap(4.0)
                .border(1.0)
                .border_color(theme.border)
                .border_radius(theme.radius_md)
                .padding(4.0)
        })
    }
}
