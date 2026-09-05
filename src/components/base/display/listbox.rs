use floem::prelude::*;

use crate::theme::Theme;

pub struct Listbox {
    pub key: String,
    pub label: String,
    pub description: Option<String>,
    pub disabled: bool,
}

impl Listbox {
    pub fn item(self, key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            description: None,
            disabled: false,
        }
    }
}

fn row(
    item: Listbox,
    is_selected: impl Fn() -> bool + 'static + Clone,
    on_select: impl Fn() + 'static,
    theme: Theme,
) -> impl View {
    let disabled = item.disabled;
    let mut builder = crate::components::base::input::select_item::SelectItem::new()
        .show_check(true)
        .disabled(disabled);
        
    if let Some(desc) = item.description {
        builder = builder.description(desc);
    }
    
    let view = builder.item(item.label, is_selected, theme.clone(), crate::theme::ColorRole::Primary);
    
    let view = if disabled {
        view.into_any()
    } else {
        view.on_event_stop(floem::event::listener::Click, move |_, _| on_select()).into_any()
    };
    
    crate::components::base::button::ripple::Ripple::ripple_target(view, theme.primary.d500, theme.radius_md)
}

/// Single-select `Listbox`: an always-visible list of selectable rows.
impl Listbox {
    pub fn listbox(
        self,
        items: Vec<Listbox>,
        selected: RwSignal<Option<String>>,
        theme: Theme,
    ) -> impl View {
        let rows: Vec<_> = items
            .into_iter()
            .map(|item| {
                let key = item.key.clone();
                let key2 = key.clone();
                row(
                    item,
                    move || selected.get().as_deref() == Some(key.as_str()),
                    move || selected.set(Some(key2.clone())),
                    theme,
                )
            })
            .collect();

        floem::views::Stack::vertical_from_iter(rows)
            .style(move |s| s.apply(theme.listbox_container_style()))
    }

    /// Multi-select `Listbox`.
    pub fn listbox_multi(
        self,
        items: Vec<Listbox>,
        selected: RwSignal<Vec<String>>,
        theme: Theme,
    ) -> impl View {
        let rows: Vec<_> = items
            .into_iter()
            .map(|item| {
                let key = item.key.clone();
                let key2 = key.clone();
                row(
                    item,
                    move || selected.get().contains(&key),
                    move || {
                        selected.update(|v| {
                            if let Some(pos) = v.iter().position(|k| k == &key2) {
                                v.remove(pos);
                            } else {
                                v.push(key2.clone());
                            }
                        });
                    },
                    theme,
                )
            })
            .collect();

        floem::views::Stack::vertical_from_iter(rows)
            .style(move |s| s.apply(theme.listbox_container_style()))
    }
}
