use floem::prelude::*;
use floem::views::dropdown::Dropdown;
use crate::theme::{Theme, ColorRole};
use std::fmt::{Display, Debug};

impl Theme {
    /// Creates a styled dropdown component using Floem's native Dropdown widget.
    pub fn dropdown<T>(
        &self,
        active_item: RwSignal<T>,
        iterator: impl IntoIterator<Item = T> + Clone + 'static,
        color: ColorRole,
    ) -> impl View
    where
        T: Display + Debug + Clone + PartialEq + 'static,
    {
        let scale = *self.scale_for(color);
        let border = self.border;
        let bg = self.background;
        let bg_hover = self.background_elevated;
        let text_color = self.foreground;
        let radius = self.radius_md;

        let theme = self.clone();
        
        Dropdown::new_rw(active_item, iterator)
        .list_item_view(move |item| {
            let item_clone = item.clone();
            let active_item_sig = active_item;
            
            let is_selected_clone = item_clone.clone();
            crate::components::base::input::select_item::SelectItem::new().item(
                item.clone().to_string(),
                move || active_item_sig.get() == is_selected_clone,
                theme.clone(),
                color
            ).into_any()
        })
        .style(move |s| {
            s.border(1.0)
                .border_color(border)
                .border_radius(radius)
                .background(bg)
                .color(text_color)
                .padding_horiz(12.0)
                .padding_vert(6.0)
                .hover(|s| s.background(bg_hover))
                .focus(|s| s.border_color(scale.d500).outline(2.0).outline_color(scale.d500.with_alpha(0.3)))
        })
    }
}
