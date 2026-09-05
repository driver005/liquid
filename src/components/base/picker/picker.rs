use floem::prelude::*;
use crate::theme::{Theme, ColorRole};
use std::fmt::Display;

pub struct Picker {}

impl Picker {
    pub fn new() -> Self {
        Self {}
    }

    /// An inline picker with a search input and a virtualized list of options.
    pub fn picker_with_input<T>(
        &self,
        options: impl Fn() -> Vec<T> + 'static,
        selected: RwSignal<Option<T>>,
        theme: Theme,
        color: ColorRole,
    ) -> impl View
    where
        T: Display + Clone + PartialEq + 'static,
    {
        let search_text = RwSignal::new(String::new());
        let scale = *theme.scale_for(color);

        let filtered_options = move || {
            let query = search_text.get().to_lowercase();
            let all = options();
            if query.is_empty() {
                all
            } else {
                all.into_iter()
                    .filter(|item| item.to_string().to_lowercase().contains(&query))
                    .collect::<Vec<_>>()
            }
        };

        let input_view = crate::components::base::input::input::TextInput::new().text_input(search_text, color, theme.clone())
            .style(move |s| s.width_full().margin_bottom(8.0));

        let list_view = floem::views::dyn_stack(
            filtered_options,
            |item| item.to_string(), // use string representation as key
            move |item| {
                let item_clone = item.clone();
                let item_str = item.to_string();
                let item_clone_for_style = item.clone();
                let is_selected_item = item_clone_for_style.clone();
                crate::components::base::input::select_item::SelectItem::new().item(
                    item_str,
                    move || selected.get() == Some(is_selected_item.clone()),
                    theme.clone(),
                    color
                )
                    .on_event_stop(floem::event::listener::Click, move |_, _| {
                        search_text.set(item_clone.to_string());
                        selected.set(Some(item_clone.clone()));
                    })
                    .into_any()
            }
        )
        .style(|s| s.flex_col().width_full())
        .scroll()
        .style(|s| s.width_full().flex_grow(1.0).min_height(100.0).max_height(300.0));

        floem::views::Stack::vertical((input_view, list_view))
            .style(move |s| {
                s.flex_col()
                    .width_full()
                    .padding(12.0)
                    .border(1.0)
                    .border_color(theme.border)
                    .border_radius(theme.radius_md)
                    .background(theme.background)
            })
    }
}
