use floem::prelude::*;

use crate::theme::{ColorRole, Theme};

#[derive(Clone)]
pub struct Toast {
    pub message: String,
    pub color: ColorRole,
    pub id: usize,
}

pub fn toast_container(toasts: RwSignal<Vec<Toast>>, theme: Theme) -> impl View {
    let list = floem::views::dyn_stack(
        move || toasts.get(),
        |toast| toast.id,
        move |toast| {
            let toasts_sig = toasts;
            let toast_id = toast.id;
            let message = toast.message.clone();

            floem::views::Stack::horizontal((
                floem::views::Label::new(match toast.color {
                    ColorRole::Success => "✓",
                    ColorRole::Warning => "⚠",
                    ColorRole::Danger => "✕",
                    _ => "ℹ",
                })
                .style(move |s| s.apply(theme.toast_icon_style(toast.color))),
                floem::views::Label::new(message)
                    .style(move |s| s.apply(theme.toast_message_style())),
            ))
            .style(move |s| s.apply(theme.toast_item_container_style(toast.color)))
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                toasts_sig.update(|t| t.retain(|x| x.id != toast_id));
            })
        },
    );

    list.style(move |s| s.apply(theme.toast_list_container_style()))
}

impl Toast {
    pub fn new(message: &str, color: ColorRole, id: usize) -> Toast {
        Toast {
            message: message.to_string(),
            color,
            id,
        }
    }
}
