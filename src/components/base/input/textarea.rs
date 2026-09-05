use floem::prelude::*;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct Textarea {}
impl Textarea {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn textarea(
        value: RwSignal<String>,
        theme: Theme,
        placeholder: &str,
        rows: usize,
    ) -> impl View {
        let ph = placeholder.to_string();
        let height = (rows as f32 * 20.0) + 16.0;

        floem::views::TextInput::new(value)
            .placeholder(ph)
            .style(move |s| s.apply(theme.textarea_style(false)).height(height))
    }

    pub fn textarea_with_counter(
        self,
        value: RwSignal<String>,
        theme: Theme,
        placeholder: &str,
        rows: usize,
        max_chars: usize,
    ) -> impl View {
        let val = value;

        floem::views::Stack::vertical((
            Self::textarea(val, theme, placeholder, rows),
            floem::views::Stack::horizontal((
                floem::views::Empty::new().style(|s| s.flex_grow(1.0)),
                floem::views::Label::derived(move || {
                    let len = val.get().len();
                    format!("{}/{}", len, max_chars)
                })
                .style({
                    move |s| {
                        let len = val.get().len();
                        let is_over = len > max_chars;
                        s.apply(theme.textarea_counter_label_style(is_over))
                    }
                }),
            ))
            .style(move |s| s.apply(theme.textarea_counter_container_style())),
        ))
        .style(move |s| s.apply(theme.textarea_container_style()))
    }
}
