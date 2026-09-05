use floem::prelude::{SignalGet, SignalUpdate};
use std::fmt::Display;

use floem::prelude::*;
use floem::{
    
    peniko::Color,
    reactive::{create_signal, ReadSignal},
    style::AlignItems,
    view::View,
    views::{container, h_stack, label, svg, Decorators},
};

use crate::theme::{Theme, ColorRole};

impl Theme {
    fn checkbox_symbol_uikit(&self, read_signal: ReadSignal<bool>) -> impl View {
        const CHECKED_SVG: &str = r#"
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16">
				<g transform="matrix(0.925671,0,0,0.925671,2.36266,1.94611)">
					<path d="M5.19,11.83L0.18,7.44L1.82,5.56L4.81,8.17L10,1.25L12,2.75L5.19,11.83Z" style="fill:white;fill-rule:nonzero;"/>
				</g>
			</svg>
		"#;
        let svg_str = move || if read_signal.get() { CHECKED_SVG } else { "" }.to_string();
        svg(svg_str)
    }

    pub fn labeled_checkbox_uikit<S: Display + 'static>(
        &self,
        read_signal: ReadSignal<bool>,
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
                self.checkbox_symbol_uikit(read_signal).style(move |s| {
                    let is_selected = read_signal.get();
                    let unhovered_bg_color = if is_selected { scale_d500 } else { bg_color };
                    let unhovered_border_color = if is_selected { scale_d500 } else { fg_color.with_alpha(0.5) };
                    
                    let hovered_bg_color = if is_selected { scale_d600 } else { fg_color.with_alpha(0.1) };
                    let hovered_border_color = if is_selected { scale_d600 } else { scale_d500 };

                    s.background(unhovered_bg_color)
                        .padding(12.0)
                        .border(1.0)
                        .border_color(unhovered_border_color)
                        .border_radius(5.0)
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
                    .focus_visible(|s| {
                        s.outline(2.0)
                            .outline_color(Color::WHITE.with_alpha(0.5))
                    })
                    .gap(10.0)
            })
        )
    }
}
