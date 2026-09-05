use floem::prelude::*;

use crate::theme::Theme;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScrollShadowDirection {
    Vertical,
    Horizontal,
}

/// Wraps `content` in a scroll view with a fade-out shadow overlay at the
/// scrollable edges, like HeroUI's `ScrollShadow`.
///
/// The leading edge (top/left) shadow fades in and out reactively as you
/// scroll away from and back to the start. The trailing edge (bottom/right)
/// shadow is shown at a constant strength whenever wrapped — floem doesn't
/// publicly expose the scrolled content's total extent, so "scrolled all the
/// way to the end" can't be detected from outside `Scroll` the way the
/// leading edge can (that only needs the current offset).
#[derive(Default, Clone)]
pub struct ScrollShadow {}
impl ScrollShadow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scroll_shadow(
        content: impl View + 'static,
        theme: Theme,
        direction: ScrollShadowDirection,
    ) -> impl View {
        let offset = floem::reactive::RwSignal::new(0.0f64);

        let scroll = floem::views::Scroll::new(content)
            .on_event_stop(
                floem::views::scroll::ScrollChanged::listener(),
                move |_cx, evt| {
                    offset.set(match direction {
                        ScrollShadowDirection::Vertical => evt.offset.y,
                        ScrollShadowDirection::Horizontal => evt.offset.x,
                    });
                },
            )
            .style(|s| s.width_full().height_full());

        let shadow_size = 16.0;
        let shadow_color = theme.shadow_color;

        let leading = floem::views::Empty::new().style(move |s| {
            let visible = (offset.get() / 16.0).clamp(0.0, 1.0) as f32;
            let mut s = s
                .absolute()
                .pointer_events_none()
                .background(shadow_color)
                .opacity(visible)
                .transition(
                    floem::style::Opacity,
                    floem::style::Transition::ease_in_out(std::time::Duration::from_millis(120)),
                );
            s = match direction {
                ScrollShadowDirection::Vertical => s
                    .width_full()
                    .height(shadow_size)
                    .inset_top(0.0)
                    .inset_left(0.0),
                ScrollShadowDirection::Horizontal => s
                    .height_full()
                    .width(shadow_size)
                    .inset_left(0.0)
                    .inset_top(0.0),
            };
            s
        });

        let trailing = floem::views::Empty::new().style(move |s| {
            let mut s = s.absolute().pointer_events_none().background(shadow_color);
            s = match direction {
                ScrollShadowDirection::Vertical => s
                    .width_full()
                    .height(shadow_size)
                    .inset_bottom(0.0)
                    .inset_left(0.0),
                ScrollShadowDirection::Horizontal => s
                    .height_full()
                    .width(shadow_size)
                    .inset_right(0.0)
                    .inset_top(0.0),
            };
            s
        });

        floem::views::Stack::new((scroll, leading, trailing)).style(|s| {
            s.width_full()
                .height_full()
                .overflow_x(floem::taffy::style::Overflow::Hidden)
                .overflow_y(floem::taffy::style::Overflow::Hidden)
        })
    }
}
