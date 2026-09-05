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
                let on_click = item.on_click;

                let mut builder = crate::components::base::input::select_item::SelectItem::new();
                if let Some(ic) = item.icon {
                    builder = builder.icon(ic);
                }

                builder.item(
                    label,
                    || false,
                    theme.clone(),
                    crate::theme::ColorRole::Primary
                )
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
