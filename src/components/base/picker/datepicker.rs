use floem::prelude::*;
use crate::theme::{Theme, ColorRole};
use chrono::{Datelike, NaiveDate, Duration, Local, Months};

pub fn get_days_in_month(year: i32, month: u32) -> u32 {
    let (next_y, next_m) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    let next_month_first = NaiveDate::from_ymd_opt(next_y, next_m, 1).unwrap();
    (next_month_first - Duration::days(1)).day()
}

impl Theme {
    /// A standalone calendar date picker view.
    pub fn date_picker(
        &self,
        selected_date: RwSignal<Option<NaiveDate>>,
        color: ColorRole,
    ) -> impl View {
        let theme = self.clone();
        let scale = *theme.scale_for(color);
        
        let initial_view = selected_date.get_untracked()
            .unwrap_or_else(|| Local::now().date_naive())
            .with_day(1).unwrap();
            
        let current_view = RwSignal::new(initial_view);
        
        let next_month = move || {
            let curr = current_view.get();
            current_view.set(curr.checked_add_months(Months::new(1)).unwrap());
        };
        
        let prev_month = move || {
            let curr = current_view.get();
            current_view.set(curr.checked_sub_months(Months::new(1)).unwrap());
        };
        
        let header = floem::views::Stack::horizontal((
            floem::views::Label::new("<")
                .style(|s| s.padding(8.0).cursor(floem::style::CursorStyle::Pointer))
                .on_event_stop(floem::event::listener::Click, move |_, _| prev_month()),
            floem::views::dyn_container(
                move || current_view.get(),
                move |date| floem::views::Label::new(date.format("%B %Y").to_string()).into_any()
            ).style(|s| s.flex_grow(1.0).justify_center()),
            floem::views::Label::new(">")
                .style(|s| s.padding(8.0).cursor(floem::style::CursorStyle::Pointer))
                .on_event_stop(floem::event::listener::Click, move |_, _| next_month()),
        )).style(|s| s.width_full().items_center().justify_between().margin_bottom(8.0));
        
        let days_of_week = vec!["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
        let dow_view = floem::views::Stack::horizontal(
            days_of_week.into_iter().map(|d| {
                let theme_dow = theme.clone();
                floem::views::Label::new(d)
                    // Note: Use items_center and justify_center for alignment, no text_align
                    .style(move |s| s.width(32.0).items_center().justify_center().font_size(12.0).color(theme_dow.foreground_secondary))
                    .into_any()
            }).collect::<Vec<_>>()
        ).style(|s| s.width_full().justify_between().margin_bottom(8.0));
        
        let grid_view = floem::views::dyn_container(
            move || current_view.get(),
            move |view_date| {
                let year = view_date.year();
                let month = view_date.month();
                let start_weekday = view_date.weekday().num_days_from_monday();
                let days_in_month = get_days_in_month(year, month);
                
                let mut cells = Vec::new();
                
                for _ in 0..start_weekday {
                    cells.push(floem::views::empty().style(|s| s.width(32.0).height(32.0)).into_any());
                }
                
                for day in 1..=days_in_month {
                    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
                    let is_selected = selected_date.get() == Some(date);
                    
                    let cell = floem::views::Label::new(day.to_string())
                        .style(move |s| {
                            let s = s.width(32.0)
                             .height(32.0)
                             .items_center()
                             .justify_center()
                             .border_radius(theme.radius_md)
                             .cursor(floem::style::CursorStyle::Pointer);
                            if is_selected {
                                s.background(scale.d500).color(theme.background).hover(|s| s.background(scale.d600))
                            } else {
                                s.background(floem::peniko::Color::TRANSPARENT).color(theme.foreground).hover(|s| s.background(theme.content2))
                            }
                        })
                        .on_event_stop(floem::event::listener::Click, move |_, _| {
                            selected_date.set(Some(date));
                        })
                        .into_any();
                        
                    cells.push(cell);
                }
                
                let mut rows = Vec::new();
                // Consume cells 7 at a time
                let mut current_row = Vec::new();
                for cell in cells.into_iter() {
                    current_row.push(cell);
                    if current_row.len() == 7 {
                        let row = floem::views::Stack::horizontal(current_row.drain(..).collect::<Vec<_>>())
                            .style(|s| s.width_full().justify_between());
                        rows.push(row.into_any());
                    }
                }
                // Push remaining
                if !current_row.is_empty() {
                    let row = floem::views::Stack::horizontal(current_row.drain(..).collect::<Vec<_>>())
                        .style(|s| s.width_full().justify_start());
                    rows.push(row.into_any());
                }
                
                floem::views::Stack::vertical(rows)
                    .style(|s| s.flex_col().gap(4.0))
                    .into_any()
            }
        );
        
        let theme_clone = self.clone();
        floem::views::Stack::vertical((
            header,
            dow_view,
            grid_view
        )).style(move |s| {
            s.flex_col()
             .padding(16.0)
             .border(1.0)
             .border_color(theme_clone.border)
             .border_radius(theme_clone.radius_lg)
             .background(theme_clone.background)
        })
    }
}
