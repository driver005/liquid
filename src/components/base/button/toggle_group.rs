use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct ToggleGroup {}
impl ToggleGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle_group(
        self,
        options: Vec<String>,
        selected: RwSignal<Option<usize>>,
        theme: Theme,
    ) -> impl View {
        let buttons: Vec<_> = options
            .into_iter()
            .enumerate()
            .map(|(i, label)| {
                let selected_sig = selected;
                let ripple_color = theme.foreground_secondary;
                let btn = floem::views::Label::new(label)
                    .style(move |s| {
                        let is_sel = selected_sig.get() == Some(i);
                        s.font_size(13.0)
                            .padding_xy(12.0, 6.0)
                            .color(if is_sel {
                                Color::WHITE
                            } else {
                                theme.foreground_secondary
                            })
                            .background(if is_sel {
                                theme.primary.d500
                            } else {
                                Color::TRANSPARENT
                            })
                            .cursor(floem::style::CursorStyle::Pointer)
                            .transition_colors()
                            .hover(|s| {
                                if is_sel {
                                    s.background(theme.primary.d600)
                                } else {
                                    s.background(theme.content2)
                                }
                            })
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| {
                        selected.update(|s| {
                            if *s == Some(i) {
                                *s = None
                            } else {
                                *s = Some(i)
                            }
                        });
                    });
                crate::components::base::button::ripple::Ripple::ripple_target(btn, ripple_color, theme.radius_md)
            })
            .collect();

        floem::views::Stack::horizontal_from_iter(buttons).style(move |s| {
            s.flex_row()
                .gap(2.0)
                .padding(2.0)
                .border(1.0)
                .border_color(theme.border)
                .border_radius(theme.radius_md)
                .background(theme.content2)
        })
    }

    pub fn toggle_group_multi(
        self,
        options: Vec<String>,
        selected: RwSignal<Vec<usize>>,
        theme: Theme,
    ) -> impl View {
        let buttons: Vec<_> = options
            .into_iter()
            .enumerate()
            .map(|(i, label)| {
                let selected_sig = selected;
                let ripple_color = theme.primary.d500;
                let btn = floem::views::Label::new(label)
                    .style(move |s| {
                        let is_sel = selected_sig.get().contains(&i);
                        s.font_size(13.0)
                            .padding_xy(12.0, 6.0)
                            .color(if is_sel {
                                theme.primary.d700
                            } else {
                                theme.foreground_secondary
                            })
                            .background(if is_sel {
                                theme.primary.d100
                            } else {
                                Color::TRANSPARENT
                            })
                            .border_radius(theme.radius_sm)
                            .cursor(floem::style::CursorStyle::Pointer)
                            .transition_colors()
                            .hover(|s| {
                                s.background(if is_sel {
                                    theme.primary.d200
                                } else {
                                    theme.content2
                                })
                            })
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| {
                        selected_sig.update(|s| {
                            if s.contains(&i) {
                                s.retain(|x| *x != i);
                            } else {
                                s.push(i);
                            }
                        });
                    });
                crate::components::base::button::ripple::Ripple::ripple_target(btn, ripple_color, theme.radius_md)
            })
            .collect();

        floem::views::Stack::horizontal_from_iter(buttons).style(move |s| {
            s.flex_row()
                .gap(2.0)
                .padding(2.0)
                .border(1.0)
                .border_color(theme.border)
                .border_radius(theme.radius_md)
                .background(theme.content2)
        })
    }
}
