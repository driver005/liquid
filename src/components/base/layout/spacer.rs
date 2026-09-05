use floem::prelude::*;
use floem::AnyView;

#[derive(Default, Clone)]
pub struct Spacer {}
impl Spacer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spacer() -> impl View {
        floem::views::Empty::new().style(|s| s.flex_grow(1.0))
    }

    pub fn fixed_spacer(self, size: f32) -> impl View {
        floem::views::Empty::new().style(move |s| s.width(size).height(size))
    }

    pub fn vertical_spacer(self, size: f32) -> impl View {
        floem::views::Empty::new().style(move |s| s.height(size))
    }

    pub fn horizontal_spacer(self, size: f32) -> impl View {
        floem::views::Empty::new().style(move |s| s.width(size))
    }

    pub fn flex_grow(self, content: impl View + 'static, grow: f32) -> impl View {
        content.style(move |s| s.flex_grow(grow))
    }

    pub fn flex_shrink(self, content: impl View + 'static, shrink: f32) -> impl View {
        content.style(move |s| s.flex_shrink(shrink))
    }

    pub fn flex_none(self, content: impl View + 'static) -> impl View {
        content.style(|s| s.flex_grow(0.0).flex_shrink(0.0))
    }

    pub fn center(self, content: impl View + 'static) -> impl View {
        content.style(|s| s.flex_row().items_center().justify_center())
    }

    pub fn center_col(self, content: impl View + 'static) -> impl View {
        content.style(|s| s.flex_col().items_center().justify_center())
    }

    pub fn space_between(self, children: Vec<AnyView>) -> impl View {
        floem::views::Stack::horizontal_from_iter(children)
            .style(|s| s.flex_row().width_full().justify_between())
    }

    pub fn space_around(self, children: Vec<AnyView>) -> impl View {
        floem::views::Stack::horizontal_from_iter(children)
            .style(|s| s.flex_row().width_full().justify_around())
    }
}
