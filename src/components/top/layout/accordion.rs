use crate::style::StyleExt;
use floem::prelude::*;
use floem::AnyView;

use crate::theme::Theme;

pub struct Accordion {
    pub title: String,
    pub content: AnyView,
}
impl Default for Accordion {
    fn default() -> Self {
        Self {
            title: String::new(),
            content: floem::views::Empty::new().into_any(),
        }
    }
}

impl Accordion {
    pub fn new(title: impl Into<String>, content: impl View + 'static) -> Self {
        Self {
            title: title.into(),
            content: content.into_any(),
        }
    }

    pub fn accordion(
        self,
        items: Vec<Accordion>,
        expanded: RwSignal<Option<usize>>,
        theme: Theme,
    ) -> impl View {
        let views: Vec<_> = items
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let title = item.title;
                let content = item.content;
                let expanded_sig = expanded;

                let header = floem::views::Stack::horizontal((
                    floem::views::Label::new(title).style(move |s| {
                        let is_exp = expanded_sig.get() == Some(i);
                        s.font_size(14.0)
                            .font_weight(floem::text::FontWeight::BOLD)
                            .color(if is_exp {
                                theme.foreground
                            } else {
                                theme.foreground_secondary
                            })
                    }),
                    floem::views::Empty::new().style(|s| s.flex_grow(1.0)),
                    floem::views::Label::new("▾").style(move |s| {
                        let is_exp = expanded_sig.get() == Some(i);
                        s.font_size(12.0)
                            .color(theme.foreground_secondary)
                            .rotate(if is_exp { 0.0 } else { -90.0 })
                            .transition(
                                floem::style::Rotation,
                                floem::style::Transition::ease_in_out(
                                    std::time::Duration::from_millis(150),
                                ),
                            )
                    }),
                ))
                .style(move |s| {
                    s.flex_row()
                        .items_center()
                        .width_full()
                        .padding(12.0)
                        .cursor(floem::style::CursorStyle::Pointer)
                        .transition_colors()
                        .hover(|s| s.background(theme.content2))
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    expanded.update(|e| {
                        if *e == Some(i) {
                            *e = None
                        } else {
                            *e = Some(i)
                        }
                    });
                });

                let body = content.style(move |s| {
                    let is_exp = expanded_sig.get() == Some(i);
                    s.padding(12.0)
                        .padding_top(0.0)
                        .color(theme.foreground_secondary)
                        .font_size(13.0)
                        .apply_if(!is_exp, |s| s.hide())
                });

                floem::views::Stack::vertical((header, body)).style(move |s| {
                    s.flex_col()
                        .width_full()
                        .border_bottom(1.0)
                        .border_bottom_color(theme.divider)
                })
            })
            .collect();

        floem::views::Stack::vertical_from_iter(views).style(move |s| {
            s.flex_col()
                .width_full()
                .border(1.0)
                .border_color(theme.border)
                .border_radius(theme.radius_md)
                .background(theme.background_elevated)
                .overflow_x(floem::taffy::style::Overflow::Hidden)
                .overflow_y(floem::taffy::style::Overflow::Hidden)
        })
    }
}
