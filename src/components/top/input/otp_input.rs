use floem::prelude::*;
use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct OtpInput {}
impl OtpInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn otp_input(self, value: RwSignal<String>, length: usize, theme: Theme) -> impl View {
        let cell_signals: Vec<_> = (0..length).map(|_| RwSignal::new(String::new())).collect();
        
        let mut ids = Vec::new();
        let mut inputs = Vec::new();
        
        for i in 0..length {
            let cell_val = cell_signals[i];
            let input = floem::views::TextInput::new(cell_val);
            ids.push(input.id());
            
            let styled_input = input
                .style(move |s| {
                    s.width(48.0)
                     .height(48.0)
                     .padding_vert(16.0)
                     .padding_horiz(18.0) // Symmetrical padding to center horizontally
                     .font_size(18.0)
                     .border(1.0)
                     .border_color(theme.border)
                     .border_radius(theme.radius_md)
                     .background(theme.background)
                     .color(theme.foreground)
                     .focus(|s| s.border_color(theme.primary.d500))
                });
            inputs.push(styled_input);
        }

        for (i, cell_sig) in cell_signals.iter().enumerate() {
            let sig = *cell_sig;
            let ids_clone = ids.clone();
            
            floem::reactive::Effect::new(move |_| {
                let text = sig.get();
                if text.len() > 1 {
                    sig.set(text.chars().take(1).collect());
                } else if text.len() == 1 {
                    if i + 1 < length {
                        ids_clone[i + 1].request_focus();
                    }
                }
            });
        }

        // Add backspace handling
        let inputs_with_events: Vec<_> = inputs.into_iter().enumerate().map(|(i, input)| {
            let ids_clone = ids.clone();
            let cell_signals_clone = cell_signals.clone();
            let cell_val = cell_signals[i];
            
            input.on_event_stop(floem::event::listener::KeyDown, move |_cx, event| {
                if event.key == Key::Named(NamedKey::Backspace) {
                    if cell_val.get().is_empty() && i > 0 {
                        cell_signals_clone[i - 1].set(String::new());
                        ids_clone[i - 1].request_focus();
                    }
                }
            })
        }).collect();

        floem::views::Stack::horizontal_from_iter(inputs_with_events).style(|s| s.flex_row().gap(8.0))
    }
}
