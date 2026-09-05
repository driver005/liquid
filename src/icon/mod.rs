pub mod icons;

use floem::prelude::*;
use crate::theme::Theme;
pub use icons::Icon;

impl Icon {
    pub fn view(self, size: f32, theme: Theme) -> impl View {
        let svg_str = self.get_svg().to_string();
        floem::views::svg(svg_str)
            .style(move |s| {
                s.width(size)
                    .height(size)
                    .color(theme.foreground)
            })
    }
}
