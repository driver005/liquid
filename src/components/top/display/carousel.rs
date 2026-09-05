use floem::prelude::*;
use floem::AnyView;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct Carousel {}
impl Carousel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn carousel(self, slides: Vec<AnyView>, theme: Theme) -> impl View {
        let current = floem::reactive::RwSignal::new(0usize);
        let total = slides.len();

        let slide_views: Vec<_> = slides
            .into_iter()
            .enumerate()
            .map(|(i, slide)| {
                let current_sig = current;
                slide.style(move |s| s.apply(theme.carousel_slide_style(current_sig.get() == i)))
            })
            .collect();

        let prev_btn = floem::views::Label::new("‹")
            .style({
                move |s| {
                    s.apply(theme.carousel_nav_button_style())
                        .hover(move |s| s.apply(theme.carousel_nav_button_hover_style()))
                }
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                current.update(|c| {
                    if *c > 0 {
                        *c -= 1
                    } else {
                        *c = total.saturating_sub(1)
                    }
                });
            });

        let next_btn = floem::views::Label::new("›")
            .style({
                move |s| {
                    s.apply(theme.carousel_nav_button_style())
                        .hover(move |s| s.apply(theme.carousel_nav_button_hover_style()))
                }
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                current.update(|c| {
                    if *c < total.saturating_sub(1) {
                        *c += 1
                    } else {
                        *c = 0
                    }
                });
            });

        let dots: Vec<_> = (0..total)
            .map(|i| {
                let current_sig = current;
                floem::views::Empty::new()
                    .style(move |s| s.apply(theme.carousel_dot_style(current_sig.get() == i)))
                    .on_event_stop(floem::event::listener::Click, move |_, _| current.set(i))
            })
            .collect();

        floem::views::Stack::new((
            floem::views::Stack::vertical_from_iter(slide_views)
                .style(move |s| s.apply(theme.carousel_slides_container_style())),
            floem::views::Stack::horizontal((
                prev_btn,
                floem::views::Empty::new()
                    .style(move |s| s.apply(theme.carousel_nav_spacer_style())),
                next_btn,
            ))
            .style(move |s| s.apply(theme.carousel_nav_container_style())),
            floem::views::Stack::horizontal_from_iter(dots)
                .style(move |s| s.apply(theme.carousel_dots_container_style())),
        ))
        .style(move |s| s.apply(theme.carousel_container_style()))
    }
}
