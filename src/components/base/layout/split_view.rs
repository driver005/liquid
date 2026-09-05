use floem::prelude::*;

use crate::theme::Theme;

pub struct SplitView {
    pub direction: SplitDirection,
    pub initial_ratio: f32,
    pub min_first: f32,
    pub min_second: f32,
    pub divider_size: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

impl Default for SplitView {
    fn default() -> Self {
        Self {
            direction: SplitDirection::Horizontal,
            initial_ratio: 0.5,
            min_first: 80.0,
            min_second: 80.0,
            divider_size: 6.0,
        }
    }
}

impl SplitView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn split_view(
        self,
        first: impl View + 'static,
        second: impl View + 'static,
        theme: Theme,
    ) -> impl View {
        let ratio = floem::reactive::RwSignal::new(self.initial_ratio);

        let divider = floem::views::Empty::new().style(move |s| {
            let mut s = s
                .background(theme.border)
                .cursor(floem::style::CursorStyle::Pointer);
            match self.direction {
                SplitDirection::Horizontal => {
                    s = s.width(self.divider_size).height_full();
                }
                SplitDirection::Vertical => {
                    s = s.height(self.divider_size).width_full();
                }
            }
            s.hover(|s| s.background(theme.primary.d300))
        });

        let first_pane = first.style(move |s| {
            let r = ratio.get();
            match self.direction {
                SplitDirection::Horizontal => s.width(r * 100.0).height_full(),
                SplitDirection::Vertical => s.height(r * 100.0).width_full(),
            }
        });

        let second_pane = second.style(move |s| {
            let r = ratio.get();
            match self.direction {
                SplitDirection::Horizontal => s.width((1.0 - r) * 100.0).height_full(),
                SplitDirection::Vertical => s.height((1.0 - r) * 100.0).width_full(),
            }
        });

        match self.direction {
            SplitDirection::Horizontal => {
                floem::views::Stack::horizontal((first_pane, divider, second_pane))
                    .style(|s| s.flex_row().width_full().height_full())
            }
            SplitDirection::Vertical => {
                floem::views::Stack::vertical((first_pane, divider, second_pane))
                    .style(|s| s.flex_col().width_full().height_full())
            }
        }
    }
}
