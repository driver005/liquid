use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

/// A HeroUI-style `Autocomplete`: a text input whose dropdown reactively
/// filters `options` by the current input text as you type.
#[derive(Default, Clone)]
pub struct Autocomplete {}
impl Autocomplete {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn autocomplete(options: Vec<String>, value: RwSignal<String>, theme: Theme) -> impl View {
        let open = floem::reactive::RwSignal::new(false);
        let options = std::rc::Rc::new(options);
        let options_for_filter = options.clone();
        let input = floem::views::TextInput::new(value)
            .placeholder("Search...")
            .style(move |s| s.apply(theme.input_container_style()))
            .on_event_stop(floem::event::listener::FocusGained, move |_cx, _gained| {
                open.set(true);
            })
            .on_event_stop(floem::event::listener::FocusLost, move |_cx, _| {
                // Delayed so a click on a row (which also blurs the input) has
                // time to register before the panel disappears.
                floem::action::exec_after(std::time::Duration::from_millis(150), move |_| {
                    open.set(false);
                });
            });

        let rows = floem::views::dyn_stack(
            move || {
                let query = value.get().to_lowercase();
                options_for_filter
                    .iter()
                    .filter(|opt| query.is_empty() || opt.to_lowercase().contains(&query))
                    .cloned()
                    .collect::<Vec<_>>()
            },
            |opt: &String| opt.clone(),
            move |opt| {
                let opt2 = opt.clone();
                floem::views::Label::new(opt)
                    .style(move |s| {
                        s.font_size(14.0)
                            .color(theme.foreground)
                            .padding_xy(12.0, 8.0)
                            .width_full()
                            .cursor(floem::style::CursorStyle::Pointer)
                            .transition_colors()
                            .hover(|s| s.background(theme.content2))
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| {
                        value.set(opt2.clone());
                        open.set(false);
                    })
            },
        )
        .style(move |s| {
            let is_open = open.get();
            let theme = &theme;
            s.flex_col()
                .width_full()
                .max_height(240.0)
                .border(1.0)
                .border_color(theme.border)
                .border_top(0.0)
                .border_top_color(Color::TRANSPARENT)
                .border_bottom_left_radius(theme.radius_md)
                .border_bottom_right_radius(theme.radius_md)
                .background(theme.background_elevated)
                .box_shadow(crate::theme::make_shadow(
                    4.0,
                    12.0,
                    theme.shadow_color,
                    8.0,
                ))
                .absolute()
                .inset_top(100.0)
                .inset_left(0.0)
                .z_index(40)
                .overflow_x(floem::taffy::style::Overflow::Hidden)
                .overflow_y(floem::taffy::style::Overflow::Scroll)
                .overlay_fade(is_open)
        });

        floem::views::Stack::new((input, rows))
    }
}
