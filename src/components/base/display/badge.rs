use floem::prelude::*;

use crate::theme::{ColorRole, Size, Theme, Variant};

#[derive(Default, Clone)]
pub struct Badge {}
impl Badge {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn badge(
        self,
        label: impl Into<String>,
        theme: Theme,
        color: ColorRole,
        variant: Variant,
        size: Size,
    ) -> impl View {
        let style = theme.badge_style(color, variant, size);
        let label = label.into();

        floem::views::Label::new(label).style(move |s| s.apply(style.clone()))
    }

    pub fn status_badge(
        self,
        label: impl Into<String>,
        theme: Theme,
        color: ColorRole,
    ) -> impl View {
        self.badge(label, theme, color, Variant::Flat, Size::Sm)
    }
}
