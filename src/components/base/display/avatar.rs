use floem::peniko::Color;
use floem::prelude::*;

use crate::theme::{Size, Theme};

pub struct Avatar {
    pub size: Size,
    pub bordered: bool,
    pub radius: f32,
}

impl Default for Avatar {
    fn default() -> Self {
        Self {
            size: Size::Md,
            bordered: false,
            radius: 9999.0,
        }
    }
}

impl Avatar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn avatar(self, initials: impl Into<String>, bg_color: Color, theme: Theme) -> impl View {
        let initials = initials.into();

        floem::views::Label::new(initials).style({
            move |s| {
                s.apply(theme.avatar_style(self.size, self.bordered, self.radius, bg_color.clone()))
            }
        })
    }

    pub fn avatar_group(self, avatars: Vec<(String, Color)>, theme: Theme) -> impl View {
        let views: Vec<_> = avatars
            .into_iter()
            .map(|(initials, color)| Avatar::default().avatar(initials, color, theme))
            .collect();

        floem::views::Stack::horizontal_from_iter(views)
            .style(move |s| s.apply(theme.avatar_group_style()))
    }
}
