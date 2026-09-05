use floem::peniko::Color;
use floem::prelude::*;

/// Wraps `child` with a HeroUI-style click ripple: a translucent circle that
/// grows from the center and fades out on every pointer-down. Wrap the fully
/// styled interactive view (button, chip, list row, ...) with this last.
///
/// The ripple always grows from the element's center rather than the exact
/// click point, since that avoids needing the click position in local
/// (element-relative) coordinates.
#[derive(Default, Clone)]
pub struct Ripple {}
impl Ripple {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ripple_target(child: impl View + 'static, color: Color, radius: f32) -> impl View {
        let ripples: RwSignal<Vec<(u64, RwSignal<bool>)>> =
            floem::reactive::RwSignal::new(Vec::new());
        let next_id: RwSignal<u64> = floem::reactive::RwSignal::new(0);

        let layer = floem::views::dyn_stack(
            move || ripples.get(),
            |(id, _grown)| *id,
            move |(_id, grown)| {
                floem::views::Empty::new().style(move |s| {
                    let is_grown = grown.get();
                    let d: f32 = if is_grown { 1500.0 } else { 0.0 };
                    let alpha = if is_grown { 0.0 } else { 0.35 };
                    let dur = std::time::Duration::from_millis(500);
                    s.width(d)
                        .height(d)
                        .border_radius(9999.0)
                        .background(color.with_alpha(alpha))
                        .transition(
                            floem::style::Width,
                            floem::style::Transition::ease_in_out(dur),
                        )
                        .transition(
                            floem::style::Height,
                            floem::style::Transition::ease_in_out(dur),
                        )
                        .transition(
                            floem::style::Background,
                            floem::style::Transition::ease_in_out(dur),
                        )
                        .pointer_events_none()
                })
            },
        )
        .style(move |s| {
            s.absolute()
                .inset_left(0.0)
                .inset_top(0.0)
                .width_full()
                .height_full()
                .flex_row()
                .items_center()
                .justify_center()
                .pointer_events_none()
        });

        floem::views::Stack::new((child, layer))
            .style(move |s| {
                s.overflow_x(floem::taffy::style::Overflow::Hidden)
                    .overflow_y(floem::taffy::style::Overflow::Hidden)
                    .border_radius(radius)
            })
            .on_event_stop(floem::event::listener::PointerDown, move |_cx, _evt| {
                let id = next_id.get_untracked();
                next_id.set(id + 1);
                let grown = floem::reactive::RwSignal::new(false);
                ripples.update(|v| v.push((id, grown)));

                floem::action::exec_after(std::time::Duration::from_millis(16), move |_| {
                    grown.set(true);
                });
                floem::action::exec_after(std::time::Duration::from_millis(560), move |_| {
                    ripples.update(|v| v.retain(|(rid, _)| *rid != id));
                });
            })
    }
}
