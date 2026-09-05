use floem::prelude::{SignalGet, SignalUpdate};
use std::fmt::Display;

use floem::prelude::*;
use floem::{
    
    peniko::Color,
    reactive::{create_signal, ReadSignal},
    style::{AlignItems, CursorStyle},
    view::View,
    views::{container, h_stack, label, svg, Decorators},
};

use crate::theme::{Theme, ColorRole};

impl Theme {
    fn radio_symbol_uikit(&self, is_selected: bool, role: ColorRole) -> impl View {
        let accent = self.scale_for(role);
        let scale_d500 = accent.d500;
        
        let svg_str = if is_selected {
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><circle cx="8" cy="8" r="4" style="fill:white;"/></svg>"#
        } else {
            ""
        }.to_string();

        svg(svg_str).style(move |s| {
            s.width(16.0).height(16.0)
        })
    }

    pub fn labeled_radio_uikit<S: Display + 'static>(
        &self,
        is_selected: bool,
        label_render_func: impl Fn() -> S + 'static,
        role: ColorRole,
    ) -> impl View {
        let (is_hovering, set_is_hovering) = create_signal(false);
        let (is_focused, set_is_focused) = create_signal(false);
        
        let accent = self.scale_for(role);
        let bg_color = self.background;
        let fg_color = self.foreground;
        let scale_d500 = accent.d500;
        let scale_d600 = accent.d600;

        container(
            h_stack((
                self.radio_symbol_uikit(is_selected, role).style(move |s| {
                    let unhovered_bg_color = if is_selected { scale_d500 } else { bg_color };
                    let unhovered_border_color = if is_selected { scale_d500 } else { fg_color.with_alpha(0.5) };
                    
                    let hovered_bg_color = if is_selected { scale_d600 } else { fg_color.with_alpha(0.1) };
                    let hovered_border_color = if is_selected { scale_d600 } else { scale_d500 };

                    s.background(unhovered_bg_color)
                        .padding(2.0)
                        .border(1.0)
                        .border_color(unhovered_border_color)
                        .border_radius(50.0)
                        .disabled(|s| {
                            s.background(scale_d500.with_alpha(0.3))
                             .border_color(scale_d500.with_alpha(0.3))
                        })
                        .apply_if(is_hovering.get(), |s| {
                            s.background(hovered_bg_color)
                                .border_color(hovered_border_color)
                        })
                        .apply_if(is_focused.get(), |s| {
                            s.border_color(scale_d600)
                        })
                }),
                label(move || label_render_func()).style(move |s| {
                    s.disabled(|s| s.color(fg_color.with_alpha(0.5)))
                }),
            ))
            .keyboard_navigable()
            .style(|s| {
                s.align_items(AlignItems::Center)
                    .border_radius(5.0)
                    .cursor(CursorStyle::Pointer)
                    .focus_visible(|s| {
                        s.outline(2.0)
                            .outline_color(Color::WHITE.with_alpha(0.5))
                    })
                    .gap(10.0)
            })
        )
    }
}
