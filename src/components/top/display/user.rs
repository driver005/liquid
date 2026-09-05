use floem::peniko::Color;
use floem::prelude::*;

use crate::prelude::Avatar;
use crate::theme::{Size, Theme};

pub struct User {
    pub avatar_bg: Color,
    pub size: Size,
    pub description: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            avatar_bg: Color::from_rgb8(99, 102, 241),
            size: Size::Md,
            description: None,
        }
    }
}

/// A HeroUI-style `User` row: avatar + name + optional description, stacked
/// horizontally. Commonly used in tables, comment lists, and menus.
impl User {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user(self, initials: &str, name: &str, theme: Theme) -> impl View {
        let name = name.to_string();
        let initials = initials.to_string();
        let name_font = match self.size {
            Size::Sm => 13.0,
            Size::Md => 14.0,
            Size::Lg => 16.0,
        };
        let desc_font = match self.size {
            Size::Sm => 11.0,
            Size::Md => 12.0,
            Size::Lg => 13.0,
        };

        let name_col = if let Some(desc) = self.description {
            floem::views::Stack::vertical((
                floem::views::Label::new(name)
                    .style(move |s| s.apply(theme.user_name_style(name_font))),
                floem::views::Label::new(desc)
                    .style(move |s| s.apply(theme.user_desc_style(desc_font))),
            ))
            .style(move |s| s.apply(theme.user_text_col_style()))
            .into_any()
        } else {
            floem::views::Label::new(name)
                .style(move |s| s.apply(theme.user_name_style(name_font)))
                .into_any()
        };

        floem::views::Stack::horizontal((
            Avatar {
                size: self.size,
                ..Default::default()
            }
            .avatar(initials, self.avatar_bg, theme),
            name_col,
        ))
        .style(move |s| s.apply(theme.user_container_style()))
    }
}
