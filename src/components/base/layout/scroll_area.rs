use floem::prelude::*;

use crate::theme::Theme;

pub struct ScrollArea {
    pub direction: ScrollDirection,
    pub show_scrollbar: bool,
    pub padding: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Vertical,
    Horizontal,
    Both,
}

impl Default for ScrollArea {
    fn default() -> Self {
        Self {
            direction: ScrollDirection::Vertical,
            show_scrollbar: true,
            padding: 0.0,
        }
    }
}

impl ScrollArea {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scroll_area(self, content: impl View + 'static, theme: &Theme) -> impl View {
        let _theme = theme.clone();

        floem::views::Scroll::new(content).style(move |s| {
            let mut s = s.width_full().height_full();
            match self.direction {
                ScrollDirection::Vertical => {
                    s = s.overflow_y(floem::taffy::style::Overflow::Scroll);
                }
                ScrollDirection::Horizontal => {
                    s = s.overflow_x(floem::taffy::style::Overflow::Scroll);
                }
                ScrollDirection::Both => {
                    s = s
                        .overflow_x(floem::taffy::style::Overflow::Scroll)
                        .overflow_y(floem::taffy::style::Overflow::Scroll);
                }
            }
            if self.padding > 0.0 {
                s = s.padding(self.padding);
            }
            s
        })
    }

    pub fn scroll_vertical(self, content: impl View + 'static, theme: &Theme) -> impl View {
        ScrollArea {
            direction: ScrollDirection::Vertical,
            ..Default::default()
        }
        .scroll_area(content, theme)
    }

    pub fn scroll_horizontal(self, content: impl View + 'static, theme: &Theme) -> impl View {
        ScrollArea {
            direction: ScrollDirection::Horizontal,
            ..Default::default()
        }
        .scroll_area(content, theme)
    }

    pub fn scroll_both(self, content: impl View + 'static, theme: &Theme) -> impl View {
        ScrollArea {
            direction: ScrollDirection::Both,
            ..Default::default()
        }
        .scroll_area(content, theme)
    }
}
