use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct Popover {}
impl Popover {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn popover(
        self,
        trigger: impl View + 'static,
        content: impl View + 'static,
        open: RwSignal<bool>,
        theme: Theme,
    ) -> impl View {
        let trigger_view = trigger.on_event_stop(floem::event::listener::Click, move |_, _| {
            open.update(|o| *o = !*o)
        });

        let panel = floem::views::Stack::vertical((content,)).style({
            move |s| {
                s.apply(theme.popover_panel_style())
                    .overlay_fade(open.get())
                    .overlay_scale_in(open.get())
            }
        });

        floem::views::Stack::new((trigger_view, panel))
    }

    pub fn popover_header(self, title: &str, theme: Theme) -> impl View {
        let title = title.to_string();
        floem::views::Label::new(title).style(move |s| s.apply(theme.popover_header_style()))
    }
}
