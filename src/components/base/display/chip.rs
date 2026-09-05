use floem::peniko::Color;
use floem::prelude::*;

use crate::theme::{ColorRole, Theme, Variant};

#[derive(Default, Clone)]
pub struct Chip {}
impl Chip {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn chip(
        self,
        label: impl Into<String>,
        on_close: impl Fn() + 'static,
        theme: Theme,
        color: ColorRole,
        variant: Variant,
    ) -> impl View {
        let scale = *theme.scale_for(color);
        let label = label.into();
        let radius = theme.radius_sm;

        let (bg, fg, border_color) = match variant {
            Variant::Solid => {
                let bg = if color == ColorRole::Default {
                    theme.default
                } else {
                    scale.d500
                };
                let fg = if color == ColorRole::Default {
                    theme.default_foreground
                } else {
                    Color::WHITE
                };
                (bg, fg, Color::TRANSPARENT)
            }
            Variant::Bordered => {
                let bc = if color == ColorRole::Default {
                    theme.border
                } else {
                    scale.d500
                };
                let fg = if color == ColorRole::Default {
                    theme.foreground
                } else {
                    scale.d700
                };
                (Color::TRANSPARENT, fg, bc)
            }
            _ => {
                let bg = if color == ColorRole::Default {
                    theme.content2
                } else {
                    scale.d100
                };
                let fg = if color == ColorRole::Default {
                    theme.foreground
                } else {
                    scale.d700
                };
                (bg, fg, Color::TRANSPARENT)
            }
        };

        let close_fg = fg;

        let close_svg = r#"<svg viewBox="0 0 24 24"><path fill="currentColor" d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>"#;
        let close_btn = crate::components::base::display::icon::SvgIcon::new().icon(close_svg, 14.0, theme.clone())
            .style({
                let close_fg = close_fg.clone();
                move |s| {
                    s.apply(theme.chip_close_button_style(close_fg)).color(close_fg)
                        .hover(move |s| s.apply(theme.chip_close_button_hover_style()))
                }
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| on_close());

        floem::views::Stack::horizontal((
            floem::views::Label::new(label).style({
                let fg = fg.clone();
                move |s| s.apply(theme.chip_label_style(fg))
            }),
            crate::components::base::button::ripple::Ripple::ripple_target(close_btn, close_fg, theme.radius_md),
        ))
        .style({
            let bg = bg.clone();
            let border_color = border_color.clone();
            move |s| s.apply(theme.chip_container_style(bg, border_color, radius)).justify_around()
        })
    }

    pub fn chip_group(
        self,
        chips: RwSignal<Vec<String>>,
        theme: Theme,
        color: ColorRole,
    ) -> impl View {
        let list = floem::views::dyn_stack(
            move || chips.get(),
            |item: &String| item.clone(),
            move |label| {
                let chips_sig = chips;
                let label_clone = label.clone();
                Self::default().chip(
                    label,
                    move || {
                        chips_sig.update(|c| c.retain(|x| x != &label_clone));
                    },
                    theme,
                    color,
                    Variant::Flat,
                )
            },
        );

        list.style(move |s| s.apply(theme.chip_group_container_style()))
    }
}
