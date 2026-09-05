use crate::theme::Theme;
use floem::prelude::*;

#[derive(Default, Clone)]
pub struct SegmentedControl {}
impl SegmentedControl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn segmented_control(
        self,
        options: Vec<String>,
        selected: RwSignal<usize>,
        theme: Theme,
    ) -> impl View {
        let segments: Vec<_> = options
            .into_iter()
            .enumerate()
            .map(move |(i, label)| {
                let sel_sig = selected;
                let ripple_color = theme.foreground_secondary;
                let btn = floem::views::Label::new(label)
                    .style(move |s| {
                        let is_sel = sel_sig.get() == i;
                        s.apply(theme.segmented_control_segment_style(is_sel))
                            .hover({
                                move |s| {
                                    s.apply(theme.segmented_control_segment_hover_style(is_sel))
                                }
                            })
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| selected.set(i));
                crate::components::base::button::ripple::Ripple::ripple_target(btn, ripple_color, theme.radius_md)
            })
            .collect();

        let container_style = theme.segmented_control_container_style();
        floem::views::Stack::horizontal(segments).style(move |s| s.apply(container_style.clone()))
    }

    pub fn segmented_control_pills(
        self,
        options: Vec<String>,
        selected: RwSignal<usize>,
        theme: Theme,
    ) -> impl View {
        let segments: Vec<_> = options
            .into_iter()
            .enumerate()
            .map(move |(i, label)| {
                let sel_sig = selected;
                let ripple_color = theme.primary.d500;
                let btn = floem::views::Label::new(label)
                    .style(move |s| {
                        let is_sel = sel_sig.get() == i;
                        s.apply(theme.segmented_control_pills_segment_style(is_sel))
                            .hover({
                                move |s| {
                                    s.apply(
                                        theme.segmented_control_pills_segment_hover_style(is_sel),
                                    )
                                }
                            })
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| selected.set(i));
                crate::components::base::button::ripple::Ripple::ripple_target(btn, ripple_color, theme.radius_md)
            })
            .collect();

        let container_style = theme.segmented_control_pills_container_style();
        floem::views::Stack::horizontal(segments).style(move |s| s.apply(container_style.clone()))
    }
}
