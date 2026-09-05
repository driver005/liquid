use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

pub struct Drawer {
    pub side: DrawerSide,
    pub width: f32,
    pub bordered: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DrawerSide {
    Left,
    Right,
    Top,
    Bottom,
}

impl Drawer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Drawer {
    fn default() -> Self {
        Self {
            side: DrawerSide::Right,
            width: 360.0,
            bordered: true,
        }
    }
}

impl Drawer {
    pub fn drawer(
        self,
        open: RwSignal<bool>,
        title: &str,
        content: impl View + 'static,
        on_close: impl Fn() + 'static,
        theme: Theme,
    ) -> impl View {
        let title_text = title.to_string();
        let on_close = std::sync::Arc::new(on_close);

        let overlay =
            floem::views::Empty::new().style(move |s| s.apply(theme.drawer_overlay_style()));

        let on_close_clone = on_close.clone();
        let on_close_clone2 = on_close.clone();

        let panel = floem::views::Stack::vertical((
            floem::views::Stack::horizontal((
                floem::views::Label::new(title_text).style(move |s| {
                    s.font_size(16.0)
                        .font_weight(floem::text::FontWeight::BOLD)
                        .color(theme.foreground)
                }),
                floem::views::Empty::new().style(|s| s.flex_grow(1.0)),
                floem::views::Label::new("✕")
                    .style(move |s| {
                        s.font_size(16.0)
                            .color(theme.foreground_secondary)
                            .cursor(floem::style::CursorStyle::Pointer)
                            .transition_colors()
                            .hover(|s| s.color(theme.foreground))
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| {
                        open.set(false);
                        on_close_clone2();
                    }),
            ))
            .style(|s| s.flex_row().items_center().width_full()),
            floem::views::Empty::new()
                .style(move |s| s.width_full().height(1.0).background(theme.divider)),
            content,
        ))
        .style({
            move |s| {
                let is_open = open.get();
                let mut s = s.apply(theme.drawer_panel_style(self.bordered, &self.side));
                match self.side {
                    DrawerSide::Left | DrawerSide::Right => {
                        s = s
                            .width(self.width)
                            .height_full()
                            .overlay_slide_x(is_open, self.width);
                    }
                    DrawerSide::Top | DrawerSide::Bottom => {
                        s = s
                            .width_full()
                            .height(self.width)
                            .overlay_slide_y(is_open, self.width);
                    }
                }
                s
            }
        });

        floem::views::Stack::new((
            overlay.on_event_stop(floem::event::listener::Click, move |_, _| {
                open.set(false);
                on_close_clone();
            }),
            panel,
        ))
        .style({
            move |s| {
                let is_open = open.get();
                s.apply(theme.drawer_container_style())
                    .overlay_fade(is_open)
            }
        })
    }
}
