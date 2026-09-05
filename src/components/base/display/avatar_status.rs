use floem::peniko::Color;
use floem::prelude::*;

use crate::theme::{Size, Theme};

pub struct AvatarStatus {
    pub size: Size,
    pub online: bool,
    pub show_ring: bool,
}

impl Default for AvatarStatus {
    fn default() -> Self {
        Self {
            size: Size::Md,
            online: true,
            show_ring: false,
        }
    }
}

impl AvatarStatus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn avatar_with_status(
        self,
        initials: impl Into<String>,
        bg_color: Color,
        theme: Theme,
    ) -> impl View {
        let dim = match self.size {
            Size::Sm => 28.0,
            Size::Md => 36.0,
            Size::Lg => 48.0,
        };
        let font_size = match self.size {
            Size::Sm => 12.0,
            Size::Md => 14.0,
            Size::Lg => 18.0,
        };
        let dot_size = match self.size {
            Size::Sm => 8.0,
            Size::Md => 10.0,
            Size::Lg => 12.0,
        };
        let initials = initials.into();

        floem::views::Stack::new((
            floem::views::Label::new(initials).style(move |s| {
                let mut s = s
                    .width(dim)
                    .height(dim)
                    .background(bg_color)
                    .color(Color::WHITE)
                    .font_size(font_size)
                    .font_weight(floem::text::FontWeight::BOLD)
                    .border_radius(9999.0)
                    .flex_row()
                    .items_center()
                    .justify_center();
                if self.show_ring {
                    s = s.border(2.0).border_color(if self.online {
                        theme.success.d500
                    } else {
                        theme.foreground_secondary
                    });
                }
                s
            }),
            floem::views::Empty::new().style(move |s| {
                let dot_bg = if self.online {
                    theme.success.d500
                } else {
                    theme.foreground_secondary
                };
                s.size(dot_size, dot_size)
                    .border_radius(9999.0)
                    .background(dot_bg)
                    .border(2.0)
                    .border_color(theme.background)
                    .absolute()
                    .inset_bottom(0.0)
                    .inset_right(0.0)
            }),
        ))
        .style(move |s| s.size(dim, dim))
    }
}
