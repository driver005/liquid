use floem::{reactive::{SignalGet, SignalUpdate},
    event::EventPropagation,
    reactive::{ReadSignal, WriteSignal},
    style::{Style, StyleValue},
    views::{slider::{slider as base_slider, SliderChanged}, Decorators},
    IntoView,
};

use crate::theme::Theme;

pub fn slider(
    value: ReadSignal<f64>,
    set_value: WriteSignal<f64>,
    theme: Theme,
) -> impl IntoView {
    let scale_d500 = theme.scale_for(crate::theme::ColorRole::Primary).d500;
    let scale_d600 = theme.scale_for(crate::theme::ColorRole::Primary).d600;

    base_slider(move || floem::unit::Pct(value.get()))
        .on_event(SliderChanged::listener(), move |_, event| {
            set_value.set(event.pct.0);
            EventPropagation::Continue
        })
        .slider_style(move |s| {
            s.handle_color(scale_d600)
             // .bar_color(...) might be .accent_bar_color(...) or something, let's just do handle_color for now
        })
}
