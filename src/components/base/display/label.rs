use floem::prelude::*;

#[derive(Default, Clone)]
pub struct Label {}

impl Label {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label<S: std::fmt::Display>(self, text: S) -> impl View {
        floem::views::Label::new(text).style(move |s| {
            s.items_center().justify_center().padding_bottom(2.0)
        })
    }
    
    pub fn derived<S: std::fmt::Display + 'static>(self, text: impl Fn() -> S + 'static) -> impl View {
        floem::views::Label::derived(text).style(move |s| {
            s.items_center().justify_center().padding_bottom(2.0)
        })
    }
}
