use floem::peniko::Color;
use floem::prelude::*;

use crate::theme::Theme;

pub struct Stepper {
    pub label: String,
    pub description: Option<String>,
}
impl Default for Stepper {
    fn default() -> Self {
        Self {
            label: String::new(),
            description: None,
        }
    }
}

impl Stepper {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: None,
        }
    }
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn stepper(self, steps: Vec<Stepper>, current: RwSignal<usize>, theme: Theme) -> impl View {
        let total = steps.len();

        let step_views: Vec<_> = steps
            .into_iter()
            .enumerate()
            .map(|(i, step)| {
                let current_sig = current;
                let label = step.label.clone();
                let desc = step.description.clone();
                let is_last = i == total - 1;

                let circle = floem::views::Empty::new().style(move |s| {
                    let curr = current_sig.get();
                    let is_complete = i < curr;
                    let is_current = i == curr;
                    let bg = if is_complete {
                        theme.success.d500
                    } else if is_current {
                        theme.primary.d500
                    } else {
                        theme.content3
                    };
                    let fg = if is_complete || is_current {
                        Color::WHITE
                    } else {
                        theme.foreground_secondary
                    };

                    s.size(32.0, 32.0)
                        .border_radius(9999.0)
                        .background(bg)
                        .flex_row()
                        .items_center()
                        .justify_center()
                        .color(fg)
                        .font_size(13.0)
                        .font_weight(floem::text::FontWeight::BOLD)
                });

                let circle_with_num = floem::views::Stack::new((
                    circle,
                    floem::views::Label::new(if i < current.get() {
                        "✓".to_string()
                    } else {
                        (i + 1).to_string()
                    })
                    .style(move |s| {
                        let curr = current_sig.get();
                        let is_complete = i < curr;
                        let is_current = i == curr;
                        s.font_size(13.0)
                            .font_weight(floem::text::FontWeight::BOLD)
                            .color(if is_complete || is_current {
                                Color::WHITE
                            } else {
                                theme.foreground_secondary
                            })
                    }),
                ));

                let label_view = floem::views::Stack::vertical((
                    floem::views::Label::new(label).style(move |s| {
                        let is_current = current_sig.get() == i;
                        s.font_size(13.0)
                            .font_weight(if is_current {
                                floem::text::FontWeight::BOLD
                            } else {
                                floem::text::FontWeight::NORMAL
                            })
                            .color(if is_current {
                                theme.foreground
                            } else {
                                theme.foreground_secondary
                            })
                    }),
                    desc.map(|d| {
                        floem::views::Label::new(d)
                            .style(move |s| s.font_size(11.0).color(theme.foreground_secondary))
                    })
                    .map(|v| v.into_any())
                    .unwrap_or_else(|| floem::views::Empty::new().into_any()),
                ))
                .style(|s| s.flex_col().gap(2.0));

                let step_content = floem::views::Stack::horizontal((circle_with_num, label_view))
                    .style(|s| s.flex_row().items_center().gap(8.0));

                let connector = if !is_last {
                    Some(floem::views::Empty::new().style(move |s| {
                        let is_complete = current_sig.get() > i;
                        s.flex_grow(1.0)
                            .height(2.0)
                            .background(if is_complete {
                                theme.success.d500
                            } else {
                                theme.content3
                            })
                            .margin_left(8.0)
                    }))
                } else {
                    None
                };

                floem::views::Stack::horizontal((
                    step_content,
                    connector
                        .map(|c| c.into_any())
                        .unwrap_or_else(|| floem::views::Empty::new().into_any()),
                ))
                .style(|s| s.flex_row().items_center().flex_grow(1.0))
            })
            .collect();

        floem::views::Stack::horizontal_from_iter(step_views)
            .style(|s| s.flex_row().items_center().gap(0.0).width_full())
    }
}
