use floem::{
    views::{svg, Decorators},
    IntoView, View,
};

pub fn lucide(original_svg: &str) -> impl View {
    // In the future we can extract stroke-width and font-size from style cx,
    // but for now we just return the SVG and let Floem's native SVG handling do the rest.
    svg(original_svg.to_string())
        .style(|s| s.items_center().justify_center().min_size(1., 1.))
}

include!("lucide_icons.txt");

impl Icon {
    pub fn view(&self) -> impl View {
        lucide(self.get_svg())
    }
}
