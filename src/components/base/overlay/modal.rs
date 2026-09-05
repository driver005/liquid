use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct Modal {}
impl Modal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn modal(
        self,
        open: RwSignal<bool>,
        title: &str,
        content: impl View + 'static,
        on_close: impl Fn() + 'static,
        theme: Theme,
    ) -> impl View {
        let title_text = title.to_string();
        let on_close = std::sync::Arc::new(on_close);

        let overlay = floem::views::Empty::new().style(move |s| {
            s.width_full()
                .height_full()
                .background(theme.overlay)
                .absolute()
                .inset_left(0.0)
                .inset_top(0.0)
                .z_index(50)
        });

        let on_close_clone = on_close.clone();
        let on_close_clone3 = on_close.clone();

        let modal_content = floem::views::Stack::vertical((
            floem::views::Stack::horizontal((
                floem::views::Label::new(title_text).style(move |s| {
                    s.font_size(18.0)
                        .font_weight(floem::text::FontWeight::BOLD)
                        .color(theme.foreground)
                }),
                floem::views::Empty::new().style(|s| s.flex_grow(1.0)),
                floem::views::Label::new("✕")
                    .style(move |s| {
                        s.font_size(18.0)
                            .color(theme.foreground_secondary)
                            .cursor(floem::style::CursorStyle::Pointer)
                            .transition_colors()
                            .hover(|s| s.color(theme.foreground))
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| {
                        open.set(false);
                        on_close_clone3();
                    }),
            ))
            .style(|s| s.flex_row().items_center().width_full()),
            Self::divider_line(theme),
            content,
        ))
        .style(move |s| {
            s.background(theme.background_elevated)
                .border_radius(theme.radius_lg)
                .border(1.0)
                .border_color(theme.border)
                .padding(24.0)
                .width(480.0)
                .box_shadow(crate::theme::make_shadow(
                    8.0,
                    32.0,
                    theme.shadow_color,
                    16.0,
                ))
                .flex_col()
                .gap(12.0)
        });

        floem::views::Stack::new((
            overlay.on_event_stop(floem::event::listener::Click, move |_, _| {
                open.set(false);
                on_close_clone();
            }),
            modal_content.style(move |s| {
                let is_open = open.get();
                s.z_index(51).overlay_scale_in(is_open)
            }),
        ))
        .style(move |s| {
            let is_open = open.get();
            s.width_full()
                .height_full()
                .absolute()
                .inset_left(0.0)
                .inset_top(0.0)
                .z_index(50)
                .flex_row()
                .items_center()
                .justify_center()
                .overlay_fade(is_open)
        })
    }

    fn divider_line(theme: Theme) -> impl View {
        floem::views::Empty::new()
            .style(move |s| s.width_full().height(1.0).background(theme.divider))
    }
}
