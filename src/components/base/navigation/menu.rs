use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

pub struct Menu {
    pub label: String,
    pub icon: Option<&'static str>,
    pub on_click: Option<Box<dyn Fn() + 'static>>,
    pub danger: bool,
    pub separator: bool,
}

impl Menu {
    pub fn item(self, label: impl Into<String>, on_click: impl Fn() + 'static) -> Self {
        Self {
            label: label.into(),
            icon: None,
            on_click: Some(Box::new(on_click)),
            danger: false,
            separator: false,
        }
    }

    pub fn separator() -> Self {
        Self {
            label: String::new(),
            icon: None,
            on_click: None,
            danger: false,
            separator: true,
        }
    }

    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }
}

/// A generic click-to-open dropdown menu, distinct from `context_menu` (which
/// opens on right-click): `trigger` opens `items` on a plain left click.
impl Menu {
    pub fn menu(
        trigger: impl View + 'static,
        items: Vec<Menu>,
        open: RwSignal<bool>,
        theme: Theme,
    ) -> impl View {
        let menu_items: Vec<_> = items
            .into_iter()
            .map(|item| {
                let open_sig = open;

                if item.separator {
                    return floem::views::Empty::new()
                        .style(move |s| s.apply(theme.menu_separator_style()))
                        .into_any();
                }

                let label = item.label.clone();
                let icon = item.icon.map(|s| s.to_string());
                let on_click = item.on_click;
                let danger = item.danger;
                let text_color = if danger {
                    theme.danger.d500
                } else {
                    theme.foreground
                };

                let row = floem::views::Stack::horizontal((
                    icon.map(|ic| {
                        let color = text_color;
                        floem::views::Label::new(ic).style({
                            let color = color.clone();
                            move |s| s.apply(theme.menu_icon_style(color))
                        })
                    })
                    .map(|v| v.into_any())
                    .unwrap_or_else(|| floem::views::Empty::new().into_any()),
                    floem::views::Label::new(label).style({
                        let text_color = text_color.clone();
                        move |s| s.apply(theme.menu_label_style(text_color))
                    }),
                ))
                .style({
                    move |s| {
                        s.apply(theme.menu_row_style())
                            .hover(move |s| s.apply(theme.menu_row_hover_style(danger)))
                    }
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    if let Some(on_click) = &on_click {
                        on_click();
                    }
                    open_sig.set(false);
                });

                crate::components::base::button::ripple::Ripple::ripple_target(row, text_color, theme.radius_md).into_any()
            })
            .collect();

        let panel = floem::views::Stack::vertical_from_iter(menu_items).style({
            move |s| {
                let is_open = open.get();
                s.apply(theme.menu_panel_style())
                    .overlay_fade(is_open)
                    .overlay_scale_in(is_open)
            }
        });

        let trigger_with_toggle = trigger
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                open.update(|o| *o = !*o)
            });

        floem::views::Stack::new((trigger_with_toggle, panel))
    }
}
