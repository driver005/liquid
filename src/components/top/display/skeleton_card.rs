use floem::prelude::*;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct SkeletonCard {}
impl SkeletonCard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn skeleton_card(self, theme: Theme) -> impl View {
        floem::views::Stack::vertical((
            crate::components::base::misc::misc::Misc::default().skeleton(theme, 100.0, 12.0, 4.0),
            crate::components::base::misc::misc::Misc::default().skeleton(theme, 200.0, 16.0, 4.0),
            floem::views::Stack::horizontal((
                crate::components::base::misc::misc::Misc::default().skeleton(theme, 60.0, 10.0, 4.0),
                floem::views::Empty::new().style(|s| s.width(8.0)),
                crate::components::base::misc::misc::Misc::default().skeleton(theme, 40.0, 10.0, 4.0),
            ))
            .style(|s| s.flex_row()),
            crate::components::base::misc::misc::Misc::default().skeleton(theme, 100.0, 8.0, 4.0),
            crate::components::base::misc::misc::Misc::default().skeleton(theme, 80.0, 8.0, 4.0),
        ))
        .style(move |s| {
            s.flex_col()
                .gap(8.0)
                .padding(16.0)
                .border_radius(theme.radius_md)
                .border(1.0)
                .border_color(theme.border)
                .background(theme.background_elevated)
                .width(280.0)
        })
    }

    pub fn skeleton_avatar(self, theme: Theme, size: f32) -> impl View {
        crate::components::base::misc::misc::Misc::default().skeleton(theme, size, size, 9999.0)
    }

    pub fn skeleton_button(self, theme: Theme) -> impl View {
        crate::components::base::misc::misc::Misc::default().skeleton(theme, 80.0, 32.0, theme.radius_sm)
    }
}
