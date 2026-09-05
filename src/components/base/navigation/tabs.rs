use floem::prelude::*;
use crate::theme::{Theme, ColorRole};

impl Theme {
    pub fn tab_core<T>(&self, active_tab: impl Fn() -> usize + 'static, view_fn: impl Fn(usize) -> T + 'static) -> impl View 
    where T: View + 'static
    {
        let bg_color = self.background;
        let fg_color = self.foreground;
        
        floem::views::dyn_view(move || view_fn(active_tab())).style(move |s| {
            s.width_full()
             .height_full()
             .background(bg_color)
             .color(fg_color)
        })
    }
}
