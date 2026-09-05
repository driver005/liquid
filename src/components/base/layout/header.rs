use floem::prelude::*;
use crate::theme::{Theme, ColorRole};

impl Theme {
    pub fn simple_header_uikit(&self, title: &str, role: ColorRole) -> impl View {
        let compiled_title = String::from(title);
        let accent = self.scale_for(role);
        let scale_d500 = accent.d500;
        let scale_d600 = accent.d600;

        Container::new(Label::new(compiled_title).style(|s| s.font_size(28.0))).style(
            move |s| {
                s.padding_horiz(24.0)
                    .padding_vert(18.0)
                    .width_full()
                    .background(scale_d500)
                    .border_bottom(1.5)
                    .border_color(scale_d600)
            },
        )
    }

    pub fn padded_container_uikit<V: View + 'static>(&self, child: V) -> impl View {
        Container::new(floem::views::v_stack((child,))).style(move |s| s.width_full().padding(24.0))
    }
}
