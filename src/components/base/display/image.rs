use floem::prelude::*;

use crate::theme::Theme;

pub struct Image {
    pub width: f32,
    pub height: f32,
    pub radius: f32,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            width: 200.0,
            height: 150.0,
            radius: 12.0,
        }
    }
}

/// A HeroUI-style `Image`: shows the decoded bytes once available, and a
/// placeholder (with `alt` text) while `bytes` is `None` — covers both the
/// loading state and a load-failure fallback (just set it back to `None`).
impl Image {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn image(self, bytes: RwSignal<Option<Vec<u8>>>, alt: &str, theme: Theme) -> impl View {
        let alt = alt.to_string();
        let w = self.width;
        let h = self.height;
        let r = self.radius;

        dyn_container(
            move || bytes.get(),
            move |data| {
                let alt = alt.clone();
                match data {
                    Some(data) => floem::views::img(move || data.clone())
                        .style(move |s| {
                            s.width(w).height(h).border_radius(r).transition(
                                floem::style::Opacity,
                                floem::style::Transition::ease_in_out(
                                    std::time::Duration::from_millis(200),
                                ),
                            )
                        })
                        .into_any(),
                    None => floem::views::Stack::new((floem::views::Label::new(alt),))
                        .style(move |s| {
                            s.width(w)
                                .height(h)
                                .border_radius(r)
                                .background(theme.content2)
                                .flex_row()
                                .items_center()
                                .justify_center()
                                .color(theme.foreground_secondary)
                                .font_size(12.0)
                        })
                        .into_any(),
                }
            },
        )
    }
}
