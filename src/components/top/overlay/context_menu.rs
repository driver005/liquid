use crate::style::StyleExt;
use crate::theme::Theme;
use floem::prelude::*;

pub struct ContextMenu {
    pub label: String,
    pub icon: Option<&'static str>,
    pub on_click: Option<Box<dyn Fn() + 'static>>,
    pub separator: bool,
}

impl ContextMenu {
    pub fn new(label: impl Into<String>, on_click: impl Fn() + 'static) -> Self {
        Self {
            label: label.into(),
            icon: None,
            on_click: Some(Box::new(on_click)),
            separator: false,
        }
    }

    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn context_menu(
        self,
        trigger: impl View + 'static,
        items: Vec<ContextMenu>,
        open: RwSignal<bool>,
        theme: Theme,
    ) -> impl View {
        let menu_items: Vec<_> = items
            .into_iter()
            .map(move |item| {
                let open_sig = open;
                if item.separator {
                    return floem::views::Empty::new()
                        .style(move |s| {
                            s.width_full()
                                .height(1.0)
                                .background(theme.divider)
                                .margin_vert(4.0)
                                .margin_horiz(0.0)
                        })
                        .into_any();
                }

                let label = item.label.clone();
                let icon = item.icon.map(|s| s.to_string());
                let on_click = item.on_click;

                floem::views::Stack::horizontal((
                    floem::views::dyn_container(
                        move || icon.clone(),
                        move |icon_str| {
                            if let Some(i) = icon_str {
                                floem::views::Label::new(i)
                                    .style(move |s| {
                                        s.font_size(14.0)
                                            .color(theme.foreground_secondary)
                                            .width(20.0)
                                    })
                                    .into_any()
                            } else {
                                floem::views::Empty::new().into_any()
                            }
                        },
                    ),
                    floem::views::Label::new(label)
                        .style(move |s| s.font_size(13.0).color(theme.foreground).flex_grow(1.0)),
                ))
                .style(move |s| {
                    s.flex_row()
                        .items_center()
                        .gap(8.0)
                        .width_full()
                        .padding_xy(12.0, 8.0)
                        .border_radius(theme.radius_sm)
                        .cursor(floem::style::CursorStyle::Pointer)
                        .transition_colors()
                        .hover(move |s| s.background(theme.content2))
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    if let Some(on_click) = &on_click {
                        on_click();
                    }
                    open_sig.set(false);
                })
                .into_any()
            })
            .collect();

        let menu = floem::views::Stack::vertical(menu_items).style(move |s| {
            let is_open = open.get();
            s.flex_col()
                .padding(4.0)
                .background(theme.background_elevated)
                .border(1.0)
                .border_color(theme.border)
                .border_radius(theme.radius_md)
                .box_shadow(crate::theme::make_shadow(
                    4.0,
                    16.0,
                    theme.shadow_color,
                    8.0,
                ))
                .absolute()
                .inset_top(100.0)
                .inset_left(0.0)
                .z_index(80)
                // If they are missing from StyleExt, we fallback to display none
                .overlay_fade(is_open)
                .overlay_scale_in(is_open)
            // .overlay_fade(is_open)
            // .overlay_scale_in(is_open)
        });

        let trigger_with_context =
            trigger.on_event_stop(floem::event::listener::PointerDown, move |_, event| {
                if event.button == Some(floem::ui_events::pointer::PointerButton::Secondary) {
                    open.set(true);
                }
            });

        floem::views::Stack::new((trigger_with_context, menu))
    }
}
impl ContextMenu {
    pub fn context_menu_divider() -> ContextMenu {
        ContextMenu {
            label: String::new(),
            icon: None,
            on_click: None,
            separator: true,
        }
    }
}
