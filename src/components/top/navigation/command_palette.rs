use crate::style::StyleExt;
use floem::prelude::*;

use crate::theme::Theme;

pub struct CommandPalette {
    pub label: String,
    pub shortcut: Option<String>,
    pub on_select: Box<dyn Fn() + 'static>,
}

impl CommandPalette {
    pub fn new(label: impl Into<String>, on_select: impl Fn() + 'static) -> Self {
        Self {
            label: label.into(),
            shortcut: None,
            on_select: Box::new(on_select),
        }
    }
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn command_palette(
        open: RwSignal<bool>,
        commands: Vec<CommandPalette>,
        theme: Theme,
    ) -> impl View {
        let search = floem::reactive::RwSignal::new(String::new());
        let selected = floem::reactive::RwSignal::new(0usize);

        let _filtered: RwSignal<Vec<usize>> =
            floem::reactive::RwSignal::new((0..commands.len()).collect());

        let overlay = floem::views::Empty::new()
            .style(move |s| s.apply(theme.command_palette_overlay_style()));

        let search_input = floem::views::TextInput::new(search)
            .placeholder("Type a command...")
            .style(move |s| {
                s.width_full()
                    .padding(12.0)
                    .border(0.0)
                    .border_color(Color::TRANSPARENT)
                    .background(Color::TRANSPARENT)
                    .color(theme.foreground)
                    .font_size(16.0)
            });

        let command_rows: Vec<_> = commands
            .into_iter()
            .enumerate()
            .map(|(i, cmd)| {
                let label = cmd.label.clone();
                // let shortcut = cmd.shortcut.clone();
                let on_select = cmd.on_select;
                let open_sig = open;

                let mut builder = crate::components::base::input::select_item::SelectItem::new();
                if let Some(sc) = cmd.shortcut {
                    builder = builder.trailing(sc);
                }
                
                builder.item(
                    label,
                    move || selected.get() == i,
                    theme.clone(),
                    crate::theme::ColorRole::Primary
                )
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    on_select();
                    open_sig.set(false);
                })
                .into_any()
            })
            .collect();

        let palette = floem::views::Stack::vertical((
            search_input,
            floem::views::Stack::vertical_from_iter(command_rows)
                .style(move |s| s.apply(theme.command_palette_list_container_style())),
        ))
        .style(move |s| s.apply(theme.command_palette_panel_style()));

        floem::views::Stack::new((
            overlay.on_event_stop(floem::event::listener::Click, move |_, _| open.set(false)),
            palette.style({
                move |s| {
                    let is_open = open.get();
                    s.apply(theme.command_palette_wrapper_style())
                        .overlay_scale_in(is_open)
                }
            }),
        ))
        .style({
            move |s| {
                let is_open = open.get();
                s.apply(theme.command_palette_container_style())
                    .overlay_fade(is_open)
            }
        })
    }
}
