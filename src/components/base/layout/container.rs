use floem::prelude::*;

use crate::theme::Theme;

pub struct Container {
    pub max_width: f32,
    pub centered: bool,
    pub padding: f32,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            max_width: 1200.0,
            centered: true,
            padding: 24.0,
        }
    }
}

impl Container {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn container(self, content: impl View + 'static, theme: Theme) -> impl View {
        content.style({
            move |s| s.apply(theme.container_style(self.max_width, self.centered, self.padding))
        })
    }

    pub fn container_fluid(
        self,
        content: impl View + 'static,
        padding: f32,
        theme: Theme,
    ) -> impl View {
        content.style(move |s| s.apply(theme.container_fluid_style(padding)))
    }

    pub fn section(self, content: impl View + 'static, theme: Theme) -> impl View {
        content.style(move |s| s.apply(theme.container_section_style()))
    }
}
