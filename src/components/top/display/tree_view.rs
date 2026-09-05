use crate::theme::Theme;
use floem::prelude::*;
use floem::AnyView;

pub struct TreeView {
    pub label: String,
    pub icon: Option<&'static str>,
    pub children: Vec<TreeView>,
}
impl Default for TreeView {
    fn default() -> Self {
        Self {
            label: String::new(),
            icon: None,
            children: Vec::new(),
        }
    }
}

impl TreeView {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            children: vec![],
        }
    }
    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn children(mut self, children: Vec<TreeView>) -> Self {
        self.children = children;
        self
    }

    pub fn tree_view(
        self,
        nodes: Vec<TreeView>,
        expanded: RwSignal<Vec<String>>,
        selected: RwSignal<Option<String>>,
        theme: Theme,
    ) -> impl View {
        let views: Vec<_> = nodes
            .into_iter()
            .map(move |node| Self::tree_node(node, &expanded, &selected, theme, 0))
            .collect();

        floem::views::Stack::vertical(views)
            .style(move |s| s.apply(theme.tree_view_container_style()))
    }

    fn tree_node(
        node: TreeView,
        expanded: &RwSignal<Vec<String>>,
        selected: &RwSignal<Option<String>>,
        theme: Theme,
        depth: usize,
    ) -> AnyView {
        let expanded_sig = *expanded;
        let selected_sig = *selected;
        let label = node.label.clone();
        let label_toggle = label.clone();
        let label_wrap = label.clone();
        let label_select = label.clone();
        let has_children = !node.children.is_empty();
        let icon = node.icon.map(|s| s.to_string());
        let children = node.children;
        let node_label = label.clone();

        let is_expanded = expanded_sig.get().contains(&label);

        let arrow_label = label.clone();
        let arrow = floem::views::Label::derived(move || {
            let label = arrow_label.clone();
            if has_children {
                if expanded_sig.get().contains(&label) {
                    "▾"
                } else {
                    "▸"
                }
            } else {
                ""
            }
            .to_string()
        })
        .style(move |s| s.apply(theme.tree_view_arrow_style()));

        let row = floem::views::Stack::horizontal((
            arrow.on_event_stop(floem::event::listener::Click, move |_, _| {
                expanded_sig.update(|e| {
                    if e.contains(&label_toggle) {
                        e.retain(|x| x != &label_toggle);
                    } else {
                        e.push(label_toggle.clone());
                    }
                });
            }),
            floem::views::dyn_container(
                move || icon.clone(),
                move |icon_str| {
                    if let Some(i) = icon_str {
                        floem::views::Label::new(i)
                            .style(move |s| s.apply(theme.tree_view_icon_style()))
                            .into_any()
                    } else {
                        floem::views::Empty::new().into_any()
                    }
                },
            ),
            floem::views::Label::new(label.clone()).style({
                move |s| {
                    s.apply(
                        theme.tree_view_label_style(selected_sig.get() == Some(node_label.clone())),
                    )
                }
            }),
        ))
        .style({
            move |s| {
                s.apply(
                    theme
                        .tree_view_row_style(depth, selected_sig.get() == Some(label_wrap.clone())),
                )
                .hover(move |s| s.apply(theme.tree_view_row_hover_style()))
            }
        })
        .on_event_stop(floem::event::listener::Click, move |_, _| {
            selected_sig.set(Some(label_select.clone()))
        });

        let child_views: Vec<_> = if has_children && is_expanded {
            children
                .into_iter()
                .map(move |child| {
                    Self::tree_node(child, &expanded_sig, &selected_sig, theme, depth + 1)
                })
                .collect()
        } else {
            vec![]
        };

        if child_views.is_empty() {
            row.into_any()
        } else {
            floem::views::Stack::vertical((
                row,
                floem::views::Stack::vertical(child_views)
                    .style(move |s| s.apply(theme.tree_view_node_container_style())),
            ))
            .style(move |s| s.apply(theme.tree_view_node_container_style()))
            .into_any()
        }
    }
}
