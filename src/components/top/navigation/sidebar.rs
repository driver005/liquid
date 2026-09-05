use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

pub struct SidebarItem {
    pub label: String,
    pub icon: Option<&'static str>,
    pub on_click: Option<Box<dyn Fn() + 'static>>,
}

pub struct Sidebar {
    pub width: f32,
    pub collapsible: bool,
    pub bordered: bool,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            width: 240.0,
            collapsible: true,
            bordered: true,
        }
    }
}

impl Sidebar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sidebar(
        self,
        items: Vec<SidebarItem>,
        active: RwSignal<usize>,
        theme: Theme,
    ) -> impl View {
        let collapsed = floem::reactive::RwSignal::new(false);

        let item_views: Vec<_> = items
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let active_sig = active;
                let label = item.label.clone();
                let icon = item.icon.map(|s| s.to_string());
                let on_click = item.on_click;

                floem::views::Stack::horizontal((
                    icon.map(|ic| {
                        floem::views::Label::new(ic).style({
                            move |s| {
                                let is_active = active_sig.get() == i;
                                s.apply(theme.sidebar_item_icon_style(is_active))
                            }
                        })
                    })
                    .map(|v| v.into_any())
                    .unwrap_or_else(|| floem::views::Empty::new().into_any()),
                    floem::views::Label::new(label).style({
                        move |s| {
                            let is_active = active_sig.get() == i;
                            let show = !collapsed.get();
                            s.apply(theme.sidebar_item_label_style(is_active, show))
                        }
                    }),
                ))
                .style(move |s| {
                    let is_active = active_sig.get() == i;
                    s.flex_row()
                        .items_center()
                        .gap(10.0)
                        .width_full()
                        .padding_xy(12.0, 10.0)
                        .border_radius(theme.radius_sm)
                        .cursor(floem::style::CursorStyle::Pointer)
                        .background(if is_active {
                            theme.primary.d50
                        } else {
                            Color::TRANSPARENT
                        })
                        .transition_colors()
                        .hover(|s| {
                            s.background(if is_active {
                                theme.primary.d100
                            } else {
                                theme.content2
                            })
                        })
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    active.set(i);
                    if let Some(ref on_click) = on_click {
                        on_click();
                    }
                })
            })
            .collect();

        let toggle_btn = floem::views::Label::derived(
            move || if collapsed.get() { "▶" } else { "◀" },
        )
        .style(move |s| {
            s.apply(theme.sidebar_toggle_button_style())
                .hover(move |s| s.apply(theme.sidebar_toggle_button_hover_style()))
        });

        floem::views::Stack::vertical((
            floem::views::Stack::horizontal((
                floem::views::Empty::new().style(|s| s.flex_grow(1.0)),
                toggle_btn.on_event_stop(floem::event::listener::Click, move |_, _| {
                    collapsed.update(|c| *c = !*c)
                }),
            ))
            .style(move |s| s.apply(theme.sidebar_header_container_style())),
            floem::views::Stack::vertical_from_iter(item_views)
                .style(move |s| s.apply(theme.sidebar_items_container_style())),
        ))
        .style({
            move |s| {
                let w = if collapsed.get() { 56.0 } else { self.width };
                s.apply(theme.sidebar_container_style(w, self.bordered))
            }
        })
    }

    pub fn sidebar_section(
        self,
        title: &str,
        items: Vec<SidebarItem>,
        active: RwSignal<usize>,
        theme: Theme,
    ) -> impl View {
        let title_text = title.to_uppercase();
        floem::views::Stack::vertical((
            floem::views::Label::new(title_text)
                .style(move |s| s.apply(theme.sidebar_section_title_style())),
            Sidebar {
                width: 240.0,
                collapsible: false,
                bordered: false,
            }
            .sidebar(items, active, theme),
        ))
        .style(|s| s.flex_col().gap(4.0).width_full())
    }
}
