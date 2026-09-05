use floem::prelude::*;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct OtpInput {}
impl OtpInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn otp_input(self, value: RwSignal<String>, length: usize, theme: Theme) -> impl View {
        let inputs: Vec<_> = (0..length)
            .map(|i| {
                let val_sig = value;
                let cell_val = floem::reactive::RwSignal::new(String::new());

                floem::views::TextInput::new(cell_val)
                    .style(move |s| s.apply(theme.otp_cell_style()))
                    .on_event_stop(floem::event::listener::KeyDown, move |_cx, _key_event| {
                        let text = cell_val.get();
                        if !text.is_empty() {
                            val_sig.update(|v| {
                                while v.len() <= i {
                                    v.push(' ');
                                }
                                let mut chars: Vec<char> = v.chars().collect();
                                if i < chars.len() {
                                    chars[i] = text.chars().next().unwrap_or(' ');
                                }
                                *v = chars.into_iter().collect();
                            });
                        }
                    })
            })
            .collect();

        floem::views::Stack::horizontal_from_iter(inputs).style(|s| s.flex_row().gap(8.0))
    }
}
