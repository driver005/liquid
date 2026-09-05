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

        let content2 = self.content2;
        
        Dropdown::new_rw(active_item, iterator)
        .list_item_view(move |item| {
            let item_clone = item.clone();
            let active_item_sig = active_item;
            
            floem::views::Label::new(item.clone())
                .style(move |s| {
                    let is_selected = active_item_sig.get() == item_clone;
                    s.width_full()
                     .padding(8.0)
                     .border_radius(radius)
                     .cursor(floem::style::CursorStyle::Pointer)
                     .background(if is_selected { scale.d500.with_alpha(0.15) } else { floem::peniko::Color::TRANSPARENT })
                     .color(if is_selected { scale.d400 } else { text_color })
                     .hover(move |s| s.background(content2))
                })
                .into_any()
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
