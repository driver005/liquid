use floem::prelude::*;
use floem::AnyView;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct Calendar {}
impl Calendar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn calendar(
        self,
        selected: RwSignal<Option<(u32, u32, u32)>>,
        year: u32,
        month: u32,
        theme: Theme,
    ) -> impl View {
        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
                {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        };

        let first_day_of_week = Self::day_of_week(year, month, 1);

        let month_names = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];

        let weekday_labels = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

        let weekday_views: Vec<_> = weekday_labels
            .iter()
            .map(|wd| {
                floem::views::Label::new(*wd).style(move |s| {
                    s.font_size(11.0)
                        .font_weight(floem::text::FontWeight::BOLD)
                        .color(theme.foreground_secondary)
                        .width(36.0)
                        .flex_row()
                        .items_center()
                        .justify_center()
                        .padding(4.0)
                })
            })
            .collect();

        let mut cells: Vec<AnyView> = Vec::new();

        for _ in 0..first_day_of_week {
            cells.push(
                floem::views::Empty::new()
                    .style(move |s| s.size(36.0, 36.0))
                    .into_any(),
            );
        }

        for day in 1..=days_in_month {
            let _selected_sig = selected;
            let is_selected = selected.get() == Some((year, month, day));

            cells.push(
                floem::views::Label::new(day.to_string())
                    .style(move |s| {
                        s.font_size(13.0)
                            .color(if is_selected {
                                Color::WHITE
                            } else {
                                theme.foreground
                            })
                            .size(36.0, 36.0)
                            .border_radius(theme.radius_sm)
                            .background(if is_selected {
                                theme.primary.d500
                            } else {
                                Color::TRANSPARENT
                            })
                            .flex_row()
                            .items_center()
                            .justify_center()
                            .cursor(floem::style::CursorStyle::Pointer)
                            .hover(|s| {
                                if !is_selected {
                                    s.background(theme.content2)
                                } else {
                                    s.background(theme.primary.d600)
                                }
                            })
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| {
                        selected.set(Some((year, month, day)))
                    })
                    .into_any(),
            );
        }

        floem::views::Stack::vertical((
            floem::views::Label::new(format!(
                "{} {}",
                month_names[(month as usize - 1).min(11)],
                year
            ))
            .style(move |s| {
                s.font_size(16.0)
                    .font_weight(floem::text::FontWeight::BOLD)
                    .color(theme.foreground)
                    .margin_bottom(12.0)
            }),
            floem::views::Stack::horizontal_from_iter(weekday_views)
                .style(|s| s.flex_row().gap(0.0)),
            floem::views::Stack::vertical_from_iter({
                let mut rows = Vec::new();
                let mut cells_iter = cells.into_iter();
                loop {
                    let week: Vec<_> = cells_iter.by_ref().take(7).collect();
                    if week.is_empty() {
                        break;
                    }
                    rows.push(
                        floem::views::Stack::horizontal_from_iter(week)
                            .style(move |s| s.flex_row().gap(0.0)),
                    );
                }
                rows
            })
            .style(|s| s.flex_col().gap(0.0)),
        ))
        .style(move |s| {
            s.flex_col()
                .padding(16.0)
                .border_radius(theme.radius_md)
                .border(1.0)
                .border_color(theme.border)
                .background(theme.background_elevated)
                .width_full()
        })
    }

    fn day_of_week(year: u32, month: u32, day: u32) -> u32 {
        let y = if month < 3 { year - 1 } else { year };
        let m = if month < 3 { month + 12 } else { month };
        let k = y % 100;
        let j = y / 100;
        let h = (day + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
        (h + 6) % 7
    }
}
