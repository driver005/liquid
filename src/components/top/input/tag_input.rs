use crate::style::StyleExt;
use crate::theme::Theme;
use floem::prelude::*;

#[derive(Default, Clone)]
pub struct TagInput {}
impl TagInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tag_input(self, tags: RwSignal<Vec<String>>, theme: Theme) -> impl View {
        let input_val = floem::reactive::RwSignal::new(String::new());

        let add_tag = move || {
            let text = input_val.get().trim().to_string();
            if !text.is_empty() {
                tags.update(|t| t.push(text));
                input_val.set(String::new());
            }
        };

        let tag_views = floem::views::dyn_stack(
            move || tags.get(),
            move |t| t.clone(),
            move |tag| {
                let tags_sig = tags;
                let tag_clone = tag.clone();
                floem::views::Stack::horizontal((
                    floem::views::Label::new(tag.clone())
                        .style(move |s| {
                            s.font_size(13.0)
                             .color(theme.foreground)
                             .items_center().justify_center()
                        }),
                    floem::views::Label::new("✕")
                        .style(move |s| {
                            s.font_size(13.0)
                                .color(theme.foreground_secondary)
                                .cursor(floem::style::CursorStyle::Pointer)
                                .items_center().justify_center()
                                .hover(move |s| s.color(theme.danger.d500))
                        })
                        .on_event_stop(floem::event::listener::Click, move |_, _| {
                            tags_sig.update(|t| t.retain(|x| x != &tag_clone));
                        }),
                ))
                .style(move |s| {
                    s.flex_row()
                        .items_center() // Geometrically centers the labels
                        .gap(6.0)
                        .padding_horiz(8.0)
                        .height(24.0) // Container height
                        .border_radius(theme.radius_sm)
                        .background(theme.content3)
                })
            },
        );

        floem::views::Stack::horizontal((
            tag_views.style(move |s| {
                s.flex_row()
                    .gap(4.0)
                    .flex_wrap(floem::taffy::style::FlexWrap::Wrap)
            }),
            floem::views::TextInput::new(input_val)
                .placeholder("Add tag...")
                .style(move |s| {
                    s.flex_grow(1.0)
                        .border(0.0)
                        .background(Color::TRANSPARENT)
                        .color(theme.foreground)
                        .font_size(14.0)
                        .padding_horiz(8.0)
                        .items_center().justify_center()
                })
                .on_event_stop(floem::event::listener::KeyDown, move |_, event| {
                    if event.key == Key::Named(NamedKey::Enter) {
                        add_tag();
                    }
                }),
        ))
        .style(move |s| {
            s.flex_row()
                .items_center()
                .gap(4.0)
                .flex_wrap(floem::taffy::style::FlexWrap::Wrap)
                .apply(theme.input_container_style())
                .padding_horiz(8.0)
                .min_height(36.0)
        })
    }
}
