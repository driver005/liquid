use floem::peniko::Color;
use floem::prelude::*;

use crate::theme::Theme;

pub struct ImageCard {
    pub image_bg: Color,
    pub image_height: f32,
    pub bordered: bool,
    pub shadow: bool,
}

impl Default for ImageCard {
    fn default() -> Self {
        Self {
            image_bg: Color::from_rgb8(229, 231, 235),
            image_height: 160.0,
            bordered: true,
            shadow: false,
        }
    }
}

impl ImageCard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn image_card(self, title: &str, description: &str, theme: Theme) -> impl View {
        let title_text = title.to_string();
        let desc_text = description.to_string();

        floem::views::Stack::vertical((
            floem::views::Empty::new().style({
                move |s| s.apply(theme.image_card_image_style(self.image_bg, self.image_height))
            }),
            floem::views::Stack::vertical((
                floem::views::Label::new(title_text)
                    .style(move |s| s.apply(theme.image_card_title_style())),
                floem::views::Label::new(desc_text)
                    .style(move |s| s.apply(theme.image_card_desc_style())),
            ))
            .style(move |s| s.apply(theme.image_card_content_container_style())),
        ))
        .style({
            move |s| {
                s.apply(theme.image_card_style(self.bordered, self.shadow))
                    .width(280.0)
            }
        })
    }
}
