use floem::prelude::*;
use crate::theme::{Theme, ColorRole};

#[derive(Default, Clone)]
pub struct SelectItem {
    pub icon: Option<String>,
    pub description: Option<String>,
    pub trailing: Option<String>,
    pub show_check: bool,
    pub disabled: bool,
}

impl SelectItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
    
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
    
    pub fn trailing(mut self, trailing: impl Into<String>) -> Self {
        self.trailing = Some(trailing.into());
        self
    }
    
    pub fn show_check(mut self, show: bool) -> Self {
        self.show_check = show;
        self
    }
    
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn item(
        self,
        label: impl Into<String>,
        is_selected: impl Fn() -> bool + 'static + Clone,
        theme: Theme,
        color: ColorRole,
    ) -> impl View {
        let scale = *theme.scale_for(color);
        let text_color = theme.foreground;
        let bg_hover = theme.content2;
        let disabled = self.disabled;
        let has_desc = self.description.is_some();

        let icon_view = self.icon.map(|ic| {
            floem::views::Label::new(ic).style(move |s| {
                s.font_size(16.0)
                    .color(theme.foreground_secondary)
                    .width(24.0)
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .margin_right(8.0)
            }).into_any()
        }).unwrap_or_else(|| floem::views::Empty::new().into_any());

        let label_col = {
            let is_sel_label = is_selected.clone();
            let lbl = floem::views::Label::new(label.into()).style(move |s| {
                s.font_size(14.0)
                    .font_weight(if has_desc { floem::text::FontWeight::BOLD } else { floem::text::FontWeight::NORMAL })
                    .color(if is_sel_label() { scale.d400 } else { text_color })
            });

            if let Some(desc) = self.description {
                floem::views::Stack::vertical((
                    lbl,
                    floem::views::Label::new(desc).style(move |s| {
                        s.font_size(12.0)
                            .color(theme.foreground_secondary)
                            .margin_top(2.0)
                    })
                )).style(|s| s.flex_col().gap(0.0).flex_grow(1.0)).into_any()
            } else {
                lbl.style(|s| s.flex_grow(1.0)).into_any()
            }
        };

        let mut trailing_stack = vec![];
        
        if let Some(t) = self.trailing {
            trailing_stack.push(floem::views::Label::new(t).style(move |s| {
                s.font_size(12.0)
                    .color(theme.foreground_secondary)
                    .padding_left(8.0)
            }).into_any());
        }

        if self.show_check {
            let is_sel_check = is_selected.clone();
            trailing_stack.push(floem::views::Label::new("✓").style(move |s| {
                s.color(scale.d500).font_weight(floem::text::FontWeight::BOLD).padding_left(8.0)
                 .apply_if(!is_sel_check(), |s| s.hide())
            }).into_any());
        }

        let trailing_view = if trailing_stack.is_empty() {
            floem::views::Empty::new().into_any()
        } else {
            floem::views::Stack::horizontal_from_iter(trailing_stack)
                .style(|s| s.flex_row().items_center())
                .into_any()
        };

        let is_sel_main = is_selected.clone();
        floem::views::Stack::horizontal((icon_view, label_col, trailing_view))
            .style(move |s| {
                let is_sel_hover = is_sel_main.clone();
                s.flex_row()
                 .items_center()
                 .width_full()
                 .padding_horiz(12.0).padding_vert(8.0)
                 .border_radius(theme.radius_md)
                 .background(if is_sel_main() { scale.d500.with_alpha(0.15) } else { floem::peniko::Color::TRANSPARENT })
                 .apply_if(disabled, |s| s.opacity(0.5))
                 .apply_if(!disabled, move |s| {
                     s.cursor(floem::style::CursorStyle::Pointer)
                      .hover(move |s| {
                          if is_sel_hover() {
                              s
                          } else {
                              s.background(bg_hover)
                          }
                      })
                 })
            })
    }
}
