use floem::peniko::Color;
use floem::style::Style;

pub trait StyleExt {
    fn bg(self, color: Color) -> Self;
    fn text_color(self, color: Color) -> Self;
    fn padding_all(self, v: f32) -> Self;
    fn padding_xy(self, x: f32, y: f32) -> Self;
    fn margin_all(self, v: f32) -> Self;
    fn border_all(self, w: f32) -> Self;
    fn rounded(self, r: f32) -> Self;
    fn font_size_px(self, s: f32) -> Self;
    fn bold(self) -> Self;
    fn row(self) -> Self;
    fn col(self) -> Self;
    fn center(self) -> Self;
    fn gap(self, v: f32) -> Self;
    fn width_px(self, w: f32) -> Self;
    fn height_px(self, h: f32) -> Self;
    fn width_full(self) -> Self;
    fn height_full(self) -> Self;
    fn min_height_px(self, h: f32) -> Self;
    fn min_width_px(self, w: f32) -> Self;

    /// HeroUI-style focus ring: a colored outline shown only for keyboard focus.
    fn focus_ring(self, color: Color) -> Self;
    /// Smoothly animates background-color changes (hover/selected/etc).
    fn transition_colors(self) -> Self;

    /// Fades a view in/out instead of hard-hiding it, and disables clicks while it's
    /// closed (the view stays mounted so the opacity transition can actually animate).
    fn overlay_fade(self, open: bool) -> Self;
    /// Pairs with `overlay_fade`: scales a panel in from 95% while it fades in, like
    /// HeroUI's modal/popover entrance animation.
    fn overlay_scale_in(self, open: bool) -> Self;
    /// Pairs with `overlay_fade`: slides a panel in from one edge (drawer-style).
    fn overlay_slide_x(self, open: bool, closed_offset: f32) -> Self;
    /// Vertical counterpart to `overlay_slide_x` (top/bottom drawers).
    fn overlay_slide_y(self, open: bool, closed_offset: f32) -> Self;
}

impl StyleExt for Style {
    fn bg(mut self, color: Color) -> Self {
        self = self.background(color);
        self
    }

    fn text_color(self, color: Color) -> Self {
        self.color(color)
    }

    fn padding_all(self, v: f32) -> Self {
        self.padding(v)
    }

    fn padding_xy(self, x: f32, y: f32) -> Self {
        self.padding_left(x)
            .padding_right(x)
            .padding_top(y)
            .padding_bottom(y)
    }

    fn margin_all(self, v: f32) -> Self {
        self.margin(v)
    }

    fn border_all(self, w: f32) -> Self {
        self.border(w)
    }

    fn rounded(self, r: f32) -> Self {
        self.border_radius(r)
    }

    fn font_size_px(self, s: f32) -> Self {
        self.font_size(s)
    }

    fn bold(self) -> Self {
        self.font_bold()
    }

    fn row(self) -> Self {
        self.flex_row()
    }

    fn col(self) -> Self {
        self.flex_col()
    }

    fn center(self) -> Self {
        self.items_center().justify_center()
    }

    fn gap(self, v: f32) -> Self {
        self.gap(v)
    }

    fn width_px(self, w: f32) -> Self {
        self.width(w)
    }

    fn height_px(self, h: f32) -> Self {
        self.height(h)
    }

    fn width_full(self) -> Self {
        self.width_full()
    }

    fn height_full(self) -> Self {
        self.height_full()
    }

    fn min_height_px(self, h: f32) -> Self {
        self.min_height(h)
    }

    fn min_width_px(self, w: f32) -> Self {
        self.min_width(w)
    }

    fn focus_ring(self, color: Color) -> Self {
        self.focus_visible(|s| s.outline(2.0).outline_color(color))
    }

    fn transition_colors(self) -> Self {
        let d = std::time::Duration::from_millis(150);
        self.transition(
            floem::style::Background,
            floem::style::Transition::ease_in_out(d),
        )
        .transition(
            floem::style::TextColor,
            floem::style::Transition::ease_in_out(d),
        )
        .transition(
            floem::style::BorderLeftColor,
            floem::style::Transition::ease_in_out(d),
        )
        .transition(
            floem::style::BorderTopColor,
            floem::style::Transition::ease_in_out(d),
        )
        .transition(
            floem::style::BorderRightColor,
            floem::style::Transition::ease_in_out(d),
        )
        .transition(
            floem::style::BorderBottomColor,
            floem::style::Transition::ease_in_out(d),
        )
    }

    fn overlay_fade(self, open: bool) -> Self {
        self.transition(
            floem::style::Opacity,
            floem::style::Transition::ease_in_out(std::time::Duration::from_millis(180)),
        )
        .opacity(if open { 1.0 } else { 0.0 })
        .apply_if(!open, |s| s.pointer_events_none().hide())
    }

    fn overlay_scale_in(self, open: bool) -> Self {
        let d = std::time::Duration::from_millis(180);
        self.transition(
            floem::style::ScaleX,
            floem::style::Transition::ease_in_out(d),
        )
        .transition(
            floem::style::ScaleY,
            floem::style::Transition::ease_in_out(d),
        )
        .scale(if open { 100.0 } else { 95.0 })
    }

    fn overlay_slide_x(self, open: bool, closed_offset: f32) -> Self {
        self.transition(
            floem::style::TranslateX,
            floem::style::Transition::ease_in_out(std::time::Duration::from_millis(220)),
        )
        .translate_x(if open { 0.0 } else { closed_offset })
    }

    fn overlay_slide_y(self, open: bool, closed_offset: f32) -> Self {
        self.transition(
            floem::style::TranslateY,
            floem::style::Transition::ease_in_out(std::time::Duration::from_millis(220)),
        )
        .translate_y(if open { 0.0 } else { closed_offset })
    }
}
