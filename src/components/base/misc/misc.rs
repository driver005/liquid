use floem::prelude::*;

use crate::theme::{ColorRole, Size, Theme};

#[derive(Default, Clone)]
pub struct Misc {}
impl Misc {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn divider(theme: Theme) -> impl View {
        floem::views::Empty::new().style(move |s| {
            s.width_full()
                .height(1.0)
                .background(theme.divider)
                .margin(8.0)
        })
    }

    pub fn vertical_divider(self, theme: Theme, height: f32) -> impl View {
        floem::views::Empty::new().style(move |s| {
            s.width(1.0)
                .height(height)
                .background(theme.divider)
                .margin_vert(0.0)
                .margin_horiz(8.0)
        })
    }

    pub fn link(
        self,
        label: impl Into<String>,
        on_click: impl Fn() + 'static,
        theme: Theme,
    ) -> impl View {
        let label = label.into();

        floem::views::Label::new(label)
            .style(move |s| {
                s.color(theme.primary.d500)
                    .font_size(14.0)
                    .cursor(floem::style::CursorStyle::Pointer)
                    .hover(|s| s.color(theme.primary.d600))
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| on_click())
    }

    pub fn spinner(self, theme: Theme, color: ColorRole, size: Size) -> impl View {
        let dim = match size {
            Size::Sm => 14.0,
            Size::Md => 20.0,
            Size::Lg => 28.0,
        };
        let scale = *theme.scale_for(color);

        floem::views::Empty::new().style(move |s| {
            s.width(dim)
                .height(dim)
                .border_radius(9999.0)
                .border(2.0)
                .border_color(theme.border)
                .border_top(2.0)
                .border_top_color(scale.d500)
        })
    }

    pub fn progress(
        self,
        value: RwSignal<f32>,
        max: f32,
        theme: Theme,
        color: ColorRole,
        size: Size,
    ) -> impl View {
        let scale = *theme.scale_for(color);
        let height = match size {
            Size::Sm => 4.0,
            Size::Md => 8.0,
            Size::Lg => 12.0,
        };

        let track = floem::views::Empty::new().style(move |s| {
            let _pct = (value.get() / max).clamp(0.0, 1.0);
            s.width_full()
                .height(height)
                .border_radius(9999.0)
                .background(theme.content3)
        });

        track.style(move |s| {
            let pct = (value.get() / max).clamp(0.0, 1.0);
            s.width(pct * 100.0)
                .height(height)
                .border_radius(9999.0)
                .background(scale.d500)
                .transition(
                    floem::style::Width,
                    floem::style::Transition::ease_in_out(std::time::Duration::from_secs_f64(0.3)),
                )
        })
    }

    pub fn skeleton(self, theme: Theme, width: f32, height: f32, radius: f32) -> impl View {
        floem::views::Empty::new().style(move |s| {
            s.width(width)
                .height(height)
                .border_radius(radius)
                .background(theme.content2)
        })
    }

    pub fn skeleton_text(self, theme: Theme, lines: usize) -> impl View {
        let views: Vec<_> = (0..lines)
            .map(|i| {
                let w = if i == lines - 1 { 60.0 } else { 100.0 };
                crate::components::base::misc::misc::Misc::default().skeleton(theme, w * 3.0, 12.0, 4.0)
            })
            .collect();

        floem::views::Stack::vertical_from_iter(views).style(|s| s.flex_col().gap(8.0).width_full())
    }
}
