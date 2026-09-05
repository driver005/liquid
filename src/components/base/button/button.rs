use std::fmt::Display;
use floem::peniko::Color;
use floem::views::{container, Decorators};
use floem::views::ButtonClass;
use floem::{view::View, views::button};
use crate::theme::{Theme, ColorRole};

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Regular,
    Emphasized,
}

impl Theme {
    pub fn button_ui_kit<S: Display + 'static>(
        &self,
        label_func: impl Fn() -> S + 'static,
        variant: ButtonVariant,
        role: ColorRole,
    ) -> impl View {
        let accent = self.scale_for(role);
        let bg_color = self.background;
        let fg_color = self.foreground;
        let scale_d500 = accent.d500;
        let scale_d600 = accent.d600;

        container(button(floem::views::label(label_func))).style(move |s| {
            s.class(ButtonClass, move |s| {
                s.apply_if(variant == ButtonVariant::Emphasized, |s| {
                    s.active(|s| s.background(scale_d600))
                     .background(scale_d500)
                     .border_color(scale_d500)
                     .focus(|s| {
                         s.border_color(scale_d600)
                          .hover(|s| s.background(scale_d600).border_color(scale_d600))
                     })
                     .hover(|s| s.background(scale_d600).border_color(scale_d600))
                })
                .apply_if(variant == ButtonVariant::Regular, |s| {
                    s.active(|s| s.background(scale_d500.with_alpha(0.6)))
                     .background(bg_color)
                     .border_color(scale_d500)
                     .focus(|s| {
                         s.border_color(scale_d600)
                          .hover(|s| s.background(scale_d500.with_alpha(0.2)))
                     })
                     .hover(|s| {
                         s.background(scale_d500.with_alpha(0.2))
                          .border_color(scale_d600)
                     })
                })
                .border(1.0)
                .border_radius(5.0)
                .color(if variant == ButtonVariant::Emphasized { Color::WHITE } else { scale_d500 })
                .padding_horiz(20.0)
                .padding_vert(10.0)
                .disabled(|s| {
                    s.background(scale_d500.with_alpha(0.3))
                     .border_color(scale_d500.with_alpha(0.3))
                     .color(fg_color.with_alpha(0.5))
                })
            })
        })
    }
}
