use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct CodeBlock {}
impl CodeBlock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn code_block(self, code: &str, language: Option<&str>, theme: Theme) -> impl View {
        let code_text = code.to_string();
        let lang = language.map(|s| s.to_string());

        let copy_action = {
            let _code = code_text.clone();
            move || {
                // Clipboard copy would go here; platform-dependent
            }
        };

        floem::views::Stack::vertical((
            floem::views::Stack::horizontal((
                lang.map(|l| {
                    floem::views::Label::new(l.to_uppercase()).style(move |s| {
                        s.font_size(11.0)
                            .color(theme.foreground_secondary)
                            .font_weight(floem::text::FontWeight::BOLD)
                    })
                })
                .map(|v| v.into_any())
                .unwrap_or_else(|| floem::views::Empty::new().into_any()),
                floem::views::Empty::new().style(|s| s.flex_grow(1.0)),
                floem::views::Label::new("Copy")
                    .style(move |s| {
                        s.font_size(12.0)
                            .color(theme.foreground_secondary)
                            .padding_xy(8.0, 4.0)
                            .border_radius(theme.radius_sm)
                            .background(theme.content3)
                            .cursor(floem::style::CursorStyle::Pointer)
                            .hover(|s| s.color(theme.foreground))
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| copy_action()),
            ))
            .style(move |s| {
                s.flex_row()
                    .items_center()
                    .gap(8.0)
                    .padding_vert(8.0)
                    .padding_horiz(12.0)
                    .background(theme.content2)
                    .border_bottom(1.0)
                    .border_bottom_color(theme.divider)
            }),
            floem::views::Label::new(code_text).style(move |s| {
                s.font_family("monospace")
                    .font_size(13.0)
                    .color(theme.foreground)
                    .padding(16.0)
                    .width_full()
                    .line_height(1.5)
            }),
        ))
        .style(move |s| {
            s.flex_col()
                .width_full()
                .border_radius(theme.radius_md)
                .border(1.0)
                .border_color(theme.border)
                .background(theme.background_elevated)
                .overflow_x(floem::taffy::style::Overflow::Hidden)
                .overflow_y(floem::taffy::style::Overflow::Hidden)
        })
    }

    pub fn inline_code(self, code: &str, theme: Theme) -> impl View {
        let code = code.to_string();
        floem::views::Label::new(code).style(move |s| {
            s.font_family("monospace")
                .font_size(13.0)
                .color(theme.primary.d700)
                .background(theme.primary.d50)
                .padding_xy(4.0, 2.0)
                .border_radius(4.0)
        })
    }
}
