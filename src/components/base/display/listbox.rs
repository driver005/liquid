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
    is_selected: impl Fn() -> bool + 'static,
    on_select: impl Fn() + 'static,
    theme: Theme,
) -> impl View {
    let label = item.label;
    let description = item.description;
    let disabled = item.disabled;

    let label_col = if let Some(desc) = description {
        floem::views::Stack::vertical((
            floem::views::Label::new(label).style(move |s| s.apply(theme.listbox_title_style())),
            floem::views::Label::new(desc).style(move |s| s.apply(theme.listbox_desc_style())),
        ))
        .style(|s| s.flex_col().gap(1.0).flex_grow(1.0))
        .into_any()
    } else {
        floem::views::Label::new(label)
            .style(move |s| s.apply(theme.listbox_label_style()))
            .into_any()
    };

    let check = floem::views::Label::new("✓")
        .style(move |s| s.apply(theme.listbox_check_style(is_selected())));

    let row = floem::views::Stack::horizontal((label_col, check)).style({
        move |s| {
            s.apply(theme.listbox_row_style(disabled))
                .apply_if(!disabled, |s| {
                    s.hover(move |s| s.apply(theme.listbox_row_hover_style()))
                })
        }
    });

    let row = if disabled {
        row
    } else {
        row.on_event_stop(floem::event::listener::Click, move |_, _| on_select())
    };

    crate::components::base::button::ripple::Ripple::ripple_target(row, theme.primary.d500, theme.radius_md)
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
