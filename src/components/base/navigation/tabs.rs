use floem::prelude::*;
use crate::theme::{Theme, ColorRole};

#[derive(Default, Clone)]
pub struct Tabs {}

impl Tabs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tabs(
        self,
        labels: Vec<String>,
        active_tab: RwSignal<usize>,
        view_fn: impl Fn(usize) -> Box<dyn View> + 'static,
        theme: Theme,
        color: ColorRole,
    ) -> impl View {
        let scale = *theme.scale_for(color);
        let border_color = theme.border;
        let fg = theme.foreground;
        let fg_muted = theme.foreground_secondary;
        let bg_elevated = theme.background_elevated;

        let labels_for_stack = labels.clone();
        let headers = floem::views::dyn_stack(
            move || labels_for_stack.clone(),
            |item| item.clone(),
            move |label| {
                let labels_clone = labels.clone();
                let idx = labels_clone.iter().position(|l| l == &label).unwrap_or(0);
                let is_active = move || active_tab.get() == idx;
                
                floem::views::Label::new(label)
                    .style(move |s| {
                        s.padding_vert(8.0)
                         .padding_horiz(16.0)
                         .font_size(14.0)
                         .color(if is_active() { scale.d500 } else { fg_muted })
                         .border_bottom(2.0)
                         .border_color(if is_active() { scale.d500 } else { floem::peniko::Color::TRANSPARENT })
                         .cursor(floem::style::CursorStyle::Pointer)
                         .hover(move |s| {
                            if is_active() { s } else { s.color(fg).background(bg_elevated) }
                         })
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| {
                        active_tab.set(idx);
                    })
            }
        ).style(move |s| {
            s.flex_row()
             .width_full()
             .border_bottom(1.0)
             .border_color(border_color)
        });

        let content = floem::views::dyn_view(move || view_fn(active_tab.get()))
            .style(|s| s.width_full().padding_vert(16.0));

        floem::views::Stack::vertical((headers, content))
            .style(|s| s.flex_col().width_full())
    }
}
