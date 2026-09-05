use floem::peniko::Color;
use floem::prelude::*;

use crate::theme::{ColorRole, Size, Theme};

#[derive(Default, Clone)]
pub struct Switch {}
impl Switch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn switch(
        self,
        checked: RwSignal<bool>,
        theme: Theme,
        color: ColorRole,
        size: Size,
    ) -> impl View {
        let scale = *theme.scale_for(color);

        floem::views::ToggleButton::new_rw(checked)
            .toggle_style(move |s| {
                let (_, _, thumb_size, pad) = match size {
                    Size::Sm => (28.0, 16.0, 12.0, 2.0),
                    Size::Md => (36.0, 20.0, 16.0, 2.0),
                    Size::Lg => (48.0, 26.0, 22.0, 2.0),
                };
                
                s.accent_color(scale.d500)
                 .handle_color(Color::WHITE)
                 .circle_rad(thumb_size / 2.0)
                 .handle_inset(pad)
            })
            .style(move |s| {
                let (track_w, track_h, _, _) = match size {
                    Size::Sm => (28.0, 16.0, 12.0, 2.0),
                    Size::Md => (36.0, 20.0, 16.0, 2.0),
                    Size::Lg => (48.0, 26.0, 22.0, 2.0),
                };
                let is_on = checked.get();
                let bg = if is_on { scale.d500 } else { theme.border };
                s.width(track_w).height(track_h).border_radius(9999.0).background(bg)
            })
    }

    pub fn labeled_switch(
        self,
        label: &str,
        checked: RwSignal<bool>,
        theme: Theme,
        color: ColorRole,
    ) -> impl View {
        let label_text = label.to_string();

        floem::views::Stack::horizontal((
            floem::views::Label::new(label_text)
                .style(move |s| s.apply(theme.switch_label_style())),
            self.switch(checked, theme, color, Size::Md),
        ))
        .style(move |s| s.apply(theme.switch_container_style()))
    }
}
