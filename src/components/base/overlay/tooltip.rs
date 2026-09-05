use floem::prelude::*;
use crate::theme::{Theme, ColorRole};

impl Theme {
    pub fn tooltip_core<V: View + 'static, T: View + 'static>(&self, child: V, tooltip_view: impl Fn() -> T + 'static) -> impl View {
        let bg = self.content2;
        let fg = self.foreground;
        
        child.tooltip(move || {
            Container::new(tooltip_view()).style(move |s| {
                s.background(bg)
                 .color(fg)
                 .padding(8.0)
                 .border_radius(4.0)
                 
            })
        })
    }
}
