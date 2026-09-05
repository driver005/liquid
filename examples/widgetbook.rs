//! hero-floem Widgetbook
//!
//! Visual browser for every component in the library.
//! - Sidebar: searchable component list, grouped by category
//! - Main: live rendered component with light/dark toggle
//! - Top bar: component name, zoom controls, inspect button

use floem::prelude::*;
use hero_floem::prelude::*;

// ─── Component registry ──────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Category {
    Button,
    Input,
    Display,
    Navigation,
    Overlay,
    Layout,
    Icon,
    Advanced,
}

impl Category {
    fn label(self) -> &'static str {
        match self {
            Category::Button => "Button",
            Category::Input => "Input",
            Category::Display => "Display",
            Category::Navigation => "Navigation",
            Category::Overlay => "Overlay",
            Category::Layout => "Layout",
            Category::Icon => "Icon",
            Category::Advanced => "Advanced",
        }
    }
    fn all() -> &'static [Category] {
        &[
            Category::Button,
            Category::Input,
            Category::Display,
            Category::Navigation,
            Category::Overlay,
            Category::Layout,
            Category::Icon,
            Category::Advanced,
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Component {
    // Button
    ButtonPrimary,
    ButtonVariants,
    Checkbox,
    Radio,
    Switch,
    ToggleGroup,
    // Input
    TextInput,
    Textarea,
    Slider,
    SearchInput,
    OtpInput,
    TagInput,
    Rating,
    // Display
    Badge,
    Chip,
    Avatar,
    Card,
    Table,
    Kbd,
    CodeBlock,
    ListItem,
    Listbox,
    StatsCard,
    Timeline,
    EmptyState,
    // Navigation
    Tabs,
    Dropdown,
    Breadcrumb,
    Pagination,
    SegmentedControl,
    Stepper,
    Picker,
    DatePicker,
    // Overlay
    Tooltip,
    Alert,
    Toast,
    // Layout
    Accordion,
    // Icon
    LucideIcons,
    // Advanced
    Carousel,
    TreeView,
    Heatmap,
    UserCard,
}

impl Component {
    fn label(self) -> &'static str {
        match self {
            Component::ButtonPrimary => "Button",
            Component::ButtonVariants => "Button Variants",
            Component::Checkbox => "Checkbox",
            Component::Radio => "Radio",
            Component::Switch => "Switch",
            Component::ToggleGroup => "Toggle Group",
            Component::TextInput => "Text Input",
            Component::Textarea => "Textarea",
            Component::Slider => "Slider",
            Component::SearchInput => "Search Input",
            Component::OtpInput => "OTP Input",
            Component::TagInput => "Tag Input",
            Component::Rating => "Rating",
            Component::Badge => "Badge",
            Component::Chip => "Chip",
            Component::Avatar => "Avatar",
            Component::Card => "Card",
            Component::Table => "Table",
            Component::Kbd => "Keyboard",
            Component::CodeBlock => "Code Block",
            Component::ListItem => "List Item",
            Component::Listbox => "Listbox",
            Component::StatsCard => "Stats Card",
            Component::Timeline => "Timeline",
            Component::EmptyState => "Empty State",
            Component::Tabs => "Tabs",
            Component::Dropdown => "Dropdown",
            Component::Breadcrumb => "Breadcrumb",
            Component::Pagination => "Pagination",
            Component::SegmentedControl => "Segmented Control",
            Component::Stepper => "Stepper",
            Component::Picker => "Picker",
            Component::DatePicker => "Date Picker",
            Component::Tooltip => "Tooltip",
            Component::Alert => "Alert",
            Component::Toast => "Toast",
            Component::Accordion => "Accordion",
            Component::LucideIcons => "Lucide Icons",
            Component::Carousel => "Carousel",
            Component::TreeView => "Tree View",
            Component::Heatmap => "Heatmap",
            Component::UserCard => "User Card",
        }
    }
    fn category(self) -> Category {
        match self {
            Component::ButtonPrimary | Component::ButtonVariants | Component::Checkbox
            | Component::Radio | Component::Switch | Component::ToggleGroup => Category::Button,
            Component::TextInput | Component::Textarea | Component::Slider
            | Component::SearchInput | Component::OtpInput | Component::TagInput
            | Component::Rating => Category::Input,
            Component::Badge | Component::Chip | Component::Avatar | Component::Card
            | Component::Table | Component::Kbd | Component::CodeBlock | Component::ListItem
            | Component::Listbox | Component::StatsCard | Component::Timeline
            | Component::EmptyState => Category::Display,
            Component::Tabs | Component::Dropdown | Component::Breadcrumb
            | Component::Pagination | Component::SegmentedControl | Component::Stepper => Category::Navigation,
            Component::Picker | Component::DatePicker => Category::Input,
            Component::Tooltip | Component::Alert | Component::Toast => Category::Overlay,
            Component::Accordion => Category::Layout,
            Component::LucideIcons => Category::Icon,
            Component::Carousel | Component::TreeView | Component::Heatmap
            | Component::UserCard => Category::Advanced,
        }
    }
    fn all() -> &'static [Component] {
        &[
            Component::ButtonPrimary, Component::ButtonVariants, Component::Checkbox,
            Component::Radio, Component::Switch, Component::ToggleGroup,
            Component::TextInput, Component::Textarea, Component::Slider,
            Component::SearchInput, Component::OtpInput, Component::TagInput,
            Component::Rating,
            Component::Badge, Component::Chip, Component::Avatar, Component::Card,
            Component::Table, Component::Kbd, Component::CodeBlock, Component::ListItem,
            Component::Listbox, Component::StatsCard, Component::Timeline, Component::EmptyState,
            Component::Tabs, Component::Dropdown, Component::Breadcrumb, Component::Pagination,
            Component::SegmentedControl, Component::Stepper, Component::Picker, Component::DatePicker,
            Component::Tooltip, Component::Alert, Component::Toast,
            Component::Accordion,
            Component::LucideIcons,
            Component::Carousel, Component::TreeView, Component::Heatmap, Component::UserCard,
        ]
    }
}

// ─── App ─────────────────────────────────────────────────────────────────────

fn app_view() -> impl View {
    let is_dark = RwSignal::new(true);
    let search = RwSignal::new(String::new());
    let current = RwSignal::new(Component::ButtonPrimary);
    let zoom = RwSignal::new(1.0f64);

    floem::views::dyn_container(
        move || is_dark.get(),
        move |dark_mode| {
            let theme = move || if dark_mode { Theme::dark() } else { Theme::light() };
            // ── Sidebar search ──
                let search_text = hero_floem::components::base::input::input::TextInput::new().text_input(search, ColorRole::Primary, theme())
                    .style(|s| s.width_full().margin_bottom(8.0));
            
                // ── Component list ──
                let sidebar_list = floem::views::dyn_container(
                    move || search.get(),
                    move |query| {
                        let q = query.to_lowercase();
                        let th = theme();
                        let filtered: Vec<Component> = Component::all()
                            .iter()
                            .copied()
                            .filter(|c| q.is_empty() || c.label().to_lowercase().contains(&q))
                            .collect();
            
                        let mut by_cat: Vec<(Category, Vec<Component>)> = vec![];
                        for cat in Category::all() {
                            let items: Vec<Component> = filtered.iter().copied().filter(|c| c.category() == *cat).collect();
                            if !items.is_empty() {
                                by_cat.push((*cat, items));
                            }
                        }
            
                        let rows: Vec<Box<dyn View>> = by_cat.into_iter().flat_map(|(cat, items)| {
                            let mut v: Vec<Box<dyn View>> = vec![
                                Box::new(floem::views::Label::new(cat.label()).style(move |s| {
                                    s.font_bold()
                                        .font_size(11.0)
                                        .color(th.foreground_secondary)
                                        .padding_top(12.0)
                                        .padding_bottom(4.0)
                                        .padding_left(8.0)
                                        .width_full()
                                })) as Box<dyn View>,
                            ];
                            for comp in items {
                                let label = comp.label();
                                v.push(Box::new(
                                    floem::views::Label::new(label)
                                        .style(move |s| {
                                            let active = current.get() == comp;
                                            s.padding(6.0)
                                                .padding_left(16.0)
                                                .width_full()
                                                .border_radius(4.0)
                                                .cursor(floem::style::CursorStyle::Pointer)
                                                .background(if active { th.primary.d500.with_alpha(0.15) } else { floem::peniko::Color::TRANSPARENT })
                                                .color(if active { th.primary.d400 } else { th.foreground })
                                                .hover(|s| s.background(th.content2))
                                        })
                                        .on_event_stop(floem::event::listener::Click, move |_, _| current.set(comp))
                                ) as Box<dyn View>);
                            }
                            v
                        }).collect();
            
                        floem::views::Stack::vertical_from_iter(rows.into_iter()).style(|s| s.flex_col().width_full()).into_any()
                    }
                ).scroll().style(|s| s.flex_grow(1.0).width_full());
            
                // ── Sidebar ──
                let sidebar = floem::views::Stack::vertical((
                    floem::views::Label::new("hero-floem").style(move |s| {
                        s.font_size(16.0).font_bold().padding(16.0).color(theme().foreground)
                    }),
                    floem::views::Container::new(search_text).style(|s| s.padding_horiz(8.0).width_full()),
                    sidebar_list,
                )).style(move |s| {
                    s.width(220.0).height_full()
                        .border_right(1.0)
                        .border_color(theme().border)
                        .background(theme().background)
                        .flex_col()
                });
            
                // ── Top bar ──
                let topbar = floem::views::Stack::horizontal((
                    floem::views::dyn_container(
                        move || current.get(),
                        move |c| floem::views::Label::new(c.label()).style(move |s| {
                            s.font_size(18.0).font_bold().color(theme().foreground)
                        }).into_any()
                    ),
                    floem::views::Stack::horizontal((
                        // zoom out
                        theme().button_ui_kit(move || "−", ButtonVariant::Regular, ColorRole::Primary)
                            .on_event_stop(floem::event::listener::Click, move |_, _| {
                                zoom.update(|z| *z = (*z - 0.1).max(0.5));
                            }),
                        floem::views::dyn_container(
                            move || zoom.get(),
                            move |z| floem::views::Label::new(format!("{:.0}%", z * 100.0))
                                .style(move |s| s.color(theme().foreground).min_width(48.0))
                                .into_any()
                        ),
                        // zoom in
                        theme().button_ui_kit(move || "+", ButtonVariant::Regular, ColorRole::Primary)
                            .on_event_stop(floem::event::listener::Click, move |_, _| {
                                zoom.update(|z| *z = (*z + 0.1).min(3.0));
                            }),
                        // reset zoom
                        theme().button_ui_kit(move || "Reset", ButtonVariant::Regular, ColorRole::Primary)
                            .on_event_stop(floem::event::listener::Click, move |_, _| zoom.set(1.0)),
                        // dark/light toggle
                        theme().button_ui_kit(
                            move || if is_dark.get() { "☀ Light" } else { "☾ Dark" },
                            ButtonVariant::Emphasized,
                            ColorRole::Primary,
                        ).on_event_stop(floem::event::listener::Click, move |_, _| {
                            is_dark.update(|d| *d = !*d);
                        }),
                        // inspect
                        floem::views::Button::new("Inspect")
                            .action(floem::action::inspect),
                    )).style(|s| s.gap(8.0).items_center()),
                )).style(move |s| {
                    s.width_full()
                        .padding(12.0)
                        .justify_between()
                        .items_center()
                        .border_bottom(1.0)
                        .border_color(theme().border)
                        .background(theme().background)
                });
            
                // ── Content ──
                let content = floem::views::dyn_container(
                    move || current.get(),
                    move |comp| {
                        let th = theme();
                        render_component(comp, th).into_any()
                    },
                )
                .style(move |s| {
                    let z = zoom.get();
                    s.flex_grow(1.0)
                        .padding(48.0)
                        .background(theme().background_elevated)
                        .scale(z as f32 * 100.0)
                })
                .scroll()
                .style(|s| s.flex_grow(1.0));
            
                let main = floem::views::Stack::vertical((topbar, content))
                    .style(|s| s.flex_col().flex_grow(1.0).height_full());
            
                floem::views::Stack::horizontal((sidebar, main))
                    .style(move |s| {
                        s.size_full()
                            .background(theme().background)
                            .color(theme().foreground)
                    })
            .into_any()
        }
    )
}

// ─── Component renderer ──────────────────────────────────────────────────────

fn render_component(comp: Component, theme: Theme) -> impl View {
    match comp {
        Component::ButtonPrimary => render_button(theme).into_any(),
        Component::ButtonVariants => render_button_variants(theme).into_any(),
        Component::Checkbox => render_checkbox(theme).into_any(),
        Component::Radio => render_radio(theme).into_any(),
        Component::Switch => render_switch(theme).into_any(),
        Component::ToggleGroup => render_toggle_group(theme).into_any(),
        Component::TextInput => render_text_input(theme).into_any(),
        Component::Textarea => render_textarea(theme).into_any(),
        Component::Slider => render_slider(theme).into_any(),
        Component::SearchInput => render_search_input(theme).into_any(),
        Component::OtpInput => render_otp_input(theme).into_any(),
        Component::TagInput => render_tag_input(theme).into_any(),
        Component::Rating => render_rating(theme).into_any(),
        Component::Badge => render_badge(theme).into_any(),
        Component::Chip => render_chip(theme).into_any(),
        Component::Avatar => render_avatar(theme).into_any(),
        Component::Card => render_card(theme).into_any(),
        Component::Table => render_table(theme).into_any(),
        Component::Kbd => render_kbd(theme).into_any(),
        Component::CodeBlock => render_code_block(theme).into_any(),
        Component::ListItem => render_list_item(theme).into_any(),
        Component::Listbox => render_listbox(theme).into_any(),
        Component::StatsCard => render_stats_card(theme).into_any(),
        Component::Timeline => render_timeline(theme).into_any(),
        Component::EmptyState => render_empty_state(theme).into_any(),
        Component::Tabs => render_tabs(theme).into_any(),
        Component::Dropdown => render_dropdown(theme).into_any(),
        Component::Breadcrumb => render_breadcrumb(theme).into_any(),
        Component::Pagination => render_pagination(theme).into_any(),
        Component::SegmentedControl => render_segmented_control(theme).into_any(),
        Component::Stepper => render_stepper(theme).into_any(),
        Component::Picker => render_picker(theme).into_any(),
        Component::DatePicker => render_datepicker(theme).into_any(),
        Component::Tooltip => render_tooltip(theme).into_any(),
        Component::Alert => render_alert(theme).into_any(),
        Component::Toast => render_toast(theme).into_any(),
        Component::Accordion => render_accordion(theme).into_any(),
        Component::LucideIcons => render_lucide_icons(theme).into_any(),
        Component::Carousel => render_carousel(theme).into_any(),
        Component::TreeView => render_tree_view(theme).into_any(),
        Component::Heatmap => render_heatmap(theme).into_any(),
        Component::UserCard => render_user_card(theme).into_any(),
    }
}

fn wb_title(text: &'static str) -> impl View {
    floem::views::Label::new(text)
        .style(|s| s.font_size(20.0).font_bold().margin_bottom(24.0))
}

fn wb_section(text: &'static str) -> impl View {
    floem::views::Label::new(text)
        .style(|s| s.font_size(13.0).font_bold().margin_top(24.0).margin_bottom(8.0).opacity(0.6))
}

// ─── Individual component renders ─────────────────────────────────────────────

fn render_button(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Button"),
        theme.button_ui_kit(move || "Click me", ButtonVariant::Emphasized, ColorRole::Primary),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_button_variants(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Button Variants"),
        wb_section("Colors"),
        floem::views::Stack::horizontal((
            theme.button_ui_kit(move || "Primary", ButtonVariant::Emphasized, ColorRole::Primary),
            theme.button_ui_kit(move || "Success", ButtonVariant::Emphasized, ColorRole::Success),
            theme.button_ui_kit(move || "Danger", ButtonVariant::Emphasized, ColorRole::Danger),
            theme.button_ui_kit(move || "Warning", ButtonVariant::Emphasized, ColorRole::Warning),
        )).style(|s| s.gap(12.0).flex_wrap(floem::style::FlexWrap::Wrap)),
        wb_section("Styles"),
        floem::views::Stack::horizontal((
            theme.button_ui_kit(move || "Emphasized", ButtonVariant::Emphasized, ColorRole::Primary),
            theme.button_ui_kit(move || "Regular", ButtonVariant::Regular, ColorRole::Primary),
        )).style(|s| s.gap(12.0)),
    )).style(|s| s.flex_col().gap(4.0))
}

fn render_checkbox(theme: Theme) -> impl View {
    let c1 = RwSignal::new(false);
    let c2 = RwSignal::new(true);
    let c3 = RwSignal::new(false);
    floem::views::Stack::vertical((
        wb_title("Checkbox"),
        theme.labeled_checkbox_uikit(c1.read_only(), move || "Unchecked", ColorRole::Primary),
        theme.labeled_checkbox_uikit(c2.read_only(), move || "Checked", ColorRole::Primary),
        theme.labeled_checkbox_uikit(c3.read_only(), move || "Accept terms and conditions", ColorRole::Success),
    )).style(|s| s.flex_col().gap(12.0))
}

fn render_radio(theme: Theme) -> impl View {
    let sel = RwSignal::new(0usize);
    floem::views::Stack::vertical((
        wb_title("Radio"),
        floem::views::dyn_container(
            move || sel.get(),
            move |s| {
                floem::views::Stack::vertical((
                    theme.labeled_radio_uikit(s == 0, move || "Option A", ColorRole::Primary)
                        .on_event_stop(floem::event::listener::Click, move |_, _| sel.set(0)),
                    theme.labeled_radio_uikit(s == 1, move || "Option B", ColorRole::Primary)
                        .on_event_stop(floem::event::listener::Click, move |_, _| sel.set(1)),
                    theme.labeled_radio_uikit(s == 2, move || "Option C", ColorRole::Primary)
                        .on_event_stop(floem::event::listener::Click, move |_, _| sel.set(2)),
                )).style(|s| s.flex_col().gap(10.0)).into_any()
            }
        ),
    )).style(|s| s.flex_col().gap(16.0))
}

fn render_switch(theme: Theme) -> impl View {
    let s1 = RwSignal::new(false);
    let s2 = RwSignal::new(true);
    floem::views::Stack::vertical((
        wb_title("Switch"),
        Switch::new().labeled_switch("Wi-Fi", s1, theme, ColorRole::Primary),
        Switch::new().labeled_switch("Bluetooth", s2, theme, ColorRole::Success),
        Switch::new().labeled_switch("Airplane Mode", RwSignal::new(false), theme, ColorRole::Warning),
    )).style(|s| s.flex_col().gap(16.0))
}

fn render_toggle_group(theme: Theme) -> impl View {
    let sel = RwSignal::new(None::<usize>);
    floem::views::Stack::vertical((
        wb_title("Toggle Group"),
        ToggleGroup::new().toggle_group(
            vec!["Day".to_string(), "Week".to_string(), "Month".to_string()],
            sel,
            theme,
        ),
    )).style(|s| s.flex_col().gap(16.0))
}

fn render_text_input(theme: Theme) -> impl View {
    let text = RwSignal::new(String::new());
    floem::views::Stack::vertical((
        wb_title("Text Input"),
        hero_floem::components::base::input::input::TextInput::new().text_input(text, ColorRole::Primary, theme.clone()),
        wb_section("With value"),
        hero_floem::components::base::input::input::TextInput::new().text_input(RwSignal::new("Hello world".to_string()), ColorRole::Primary, theme.clone()),
    )).style(|s| s.flex_col().gap(8.0).max_width(400.0))
}

fn render_textarea(theme: Theme) -> impl View {
    let text = RwSignal::new(String::new());
    floem::views::Stack::vertical((
        wb_title("Textarea"),
        Textarea::new().textarea_with_counter(text, theme, "Type a message…", 4, 200),
    )).style(|s| s.flex_col().gap(8.0).max_width(400.0))
}

fn render_slider(theme: Theme) -> impl View {
    let val = RwSignal::new(0.4f64);
    floem::views::Stack::vertical((
        wb_title("Slider"),
        hero_floem::components::base::input::slider::slider(val.read_only(), val.write_only(), theme),
    )).style(|s| s.flex_col().gap(16.0).max_width(400.0))
}

fn render_search_input(theme: Theme) -> impl View {
    let val = RwSignal::new(String::new());
    floem::views::Stack::vertical((
        wb_title("Search Input"),
        SearchInput::new().search_input(val, theme),
    )).style(|s| s.flex_col().gap(8.0).max_width(400.0))
}

fn render_otp_input(theme: Theme) -> impl View {
    let val = RwSignal::new(String::new());
    floem::views::Stack::vertical((
        wb_title("OTP Input"),
        OtpInput::new().otp_input(val, 6, theme),
    )).style(|s| s.flex_col().gap(16.0))
}

fn render_tag_input(theme: Theme) -> impl View {
    let tags = RwSignal::new(vec!["rust".to_string(), "floem".to_string()]);
    floem::views::Stack::vertical((
        wb_title("Tag Input"),
        TagInput::new().tag_input(tags, theme),
    )).style(|s| s.flex_col().gap(16.0).max_width(400.0))
}

fn render_rating(theme: Theme) -> impl View {
    let val = RwSignal::new(3usize);
    floem::views::Stack::vertical((
        wb_title("Rating"),
        Rating::new().rating_with_label(val, 5, theme),
    )).style(|s| s.flex_col().gap(16.0))
}

fn render_badge(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Badge"),
        floem::views::Stack::horizontal((
            Badge::new().badge("Default", theme, ColorRole::Primary, Variant::Solid, Size::Md),
            Badge::new().badge("Success", theme, ColorRole::Success, Variant::Solid, Size::Md),
            Badge::new().badge("Danger", theme, ColorRole::Danger, Variant::Solid, Size::Md),
            Badge::new().badge("Warning", theme, ColorRole::Warning, Variant::Solid, Size::Md),
        )).style(|s| s.gap(8.0).flex_wrap(floem::style::FlexWrap::Wrap)),
        wb_section("Status badges"),
        floem::views::Stack::horizontal((
            Badge::new().status_badge("Online", theme, ColorRole::Success),
            Badge::new().status_badge("Offline", theme, ColorRole::Danger),
            Badge::new().status_badge("Away", theme, ColorRole::Warning),
        )).style(|s| s.gap(8.0)),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_chip(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Chip"),
        Chip::new().chip("React", || {}, theme, ColorRole::Primary, Variant::Flat),
        Chip::new().chip("Rust", || {}, theme, ColorRole::Success, Variant::Flat),
        Chip::new().chip("Removable ✕", || {}, theme, ColorRole::Danger, Variant::Solid),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_avatar(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Avatar"),
        wb_section("Single"),
        Avatar::new().avatar("JD", theme.primary.d500, theme),
        wb_section("Group"),
        Avatar::new().avatar_group(vec![
            ("AB".to_string(), theme.primary.d500),
            ("CD".to_string(), theme.success.d500),
            ("EF".to_string(), theme.danger.d500),
        ], theme),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_card(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Card"),
        Card::new().card(
            floem::views::Label::new("Card content goes here"),
            theme,
        ),
    )).style(|s| s.flex_col().gap(8.0).max_width(400.0))
}

fn render_table(theme: Theme) -> impl View {
    let cols = vec![
        Table::new("Name").width(120.0),
        Table::new("Status").width(80.0),
        Table::new("Value").width(80.0),
    ];
    let rows = vec![
        vec!["Button".into(), "✓".into(), "base".into()],
        vec!["Checkbox".into(), "✓".into(), "base".into()],
        vec!["Slider".into(), "✓".into(), "base".into()],
    ];
    floem::views::Stack::vertical((
        wb_title("Table"),
        Table::new("").table(cols, rows, theme),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_kbd(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Keyboard"),
        floem::views::Stack::horizontal((
            Kbd::kbd("⌘", theme),
            Kbd::kbd("K", theme),
        )).style(|s| s.gap(4.0)),
        wb_section("Combo"),
        Kbd::new().kbd_combo(&["Ctrl", "Shift", "P"], theme),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_code_block(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Code Block"),
        CodeBlock::new().code_block(
            r#"fn main() {
    println!("Hello, Floem!");
}"#,
            Some("rust"),
            theme,
        ),
        wb_section("Inline"),
        CodeBlock::new().inline_code("let x = 42;", theme),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_list_item(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("List Item"),
        ListItem::list_item(
            ListItem::new("Profile Settings").subtitle("Manage your account"),
            None::<Box<dyn Fn()>>,
            theme,
        ),
        ListItem::list_item(
            ListItem::new("Notifications").subtitle("Configure alerts").trailing("3"),
            None::<Box<dyn Fn()>>,
            theme,
        ),
    )).style(|s| s.flex_col().gap(4.0).max_width(400.0))
}

fn render_listbox(theme: Theme) -> impl View {
    let sel = RwSignal::new(Some("rust".to_string()));
    floem::views::Stack::vertical((
        wb_title("Listbox"),
        Listbox { key: String::new(), label: String::new(), description: None, disabled: false }.listbox(vec![
            Listbox { key: "rust".into(), label: "Rust".into(), description: None, disabled: false },
            Listbox { key: "python".into(), label: "Python".into(), description: None, disabled: false },
            Listbox { key: "go".into(), label: "Go".into(), description: None, disabled: false },
        ], sel, theme),
    )).style(|s| s.flex_col().gap(8.0).max_width(300.0))
}

fn render_stats_card(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Stats Card"),
        floem::views::Stack::horizontal((
            StatsCard::new().stats_card("Total Users", "12,340", Some(("+12%", true)), theme),
            StatsCard::new().kpi_card("Revenue", "$48,210", "💰", ColorRole::Primary, theme),
        )).style(|s| s.gap(16.0).flex_wrap(floem::style::FlexWrap::Wrap)),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_timeline(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Timeline"),
        Timeline::new("Project created")
            .description("Initial commit")
            .timestamp("2026-01-01")
            .color(ColorRole::Primary)
            .timeline(vec![
                Timeline::new("Development started")
                    .timestamp("2026-02-15")
                    .color(ColorRole::Success),
                Timeline::new("Beta release")
                    .description("Internal testing")
                    .timestamp("2026-08-01")
                    .color(ColorRole::Warning),
                Timeline::new("v1.0 launched")
                    .timestamp("2026-09-05")
                    .color(ColorRole::Danger),
            ], theme),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_empty_state(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Empty State"),
        EmptyState::new().empty_state(
            "📭",
            "Nothing here yet",
            "Add some items to get started",
            None,
            theme,
        ),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_tabs(theme: Theme) -> impl View {
    let active = RwSignal::new(0usize);
    floem::views::Stack::vertical((
        wb_title("Tabs"),
        hero_floem::components::base::navigation::tabs::Tabs::new().tabs(
            vec!["General".to_string(), "Security".to_string()],
            active,
            move |idx| match idx {
                0 => floem::views::Label::new("General settings panel").into_any(),
                1 => floem::views::Label::new("Security settings panel").into_any(),
                _ => floem::views::Label::new("").into_any(),
            },
            theme.clone(),
            ColorRole::Primary
        ),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_dropdown(theme: Theme) -> impl View {
    let sel1 = RwSignal::new("Option 1");
    let opts1 = vec!["Option 1", "Option 2", "Option 3", "Option 4"];
    
    let sel2 = RwSignal::new("Apple");
    let opts2 = vec!["Apple", "Banana", "Cherry"];
    
    floem::views::Stack::vertical((
        wb_title("Dropdown"),
        theme.dropdown(sel1, opts1, ColorRole::Primary),
        wb_section("Another selection"),
        theme.dropdown(sel2, opts2, ColorRole::Success),
    )).style(|s| s.flex_col().gap(16.0).max_width(200.0))
}

fn render_breadcrumb(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Breadcrumb"),
        Breadcrumb::new().breadcrumb_simple(vec!["Home", "Components", "Breadcrumb"], theme),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_pagination(theme: Theme) -> impl View {
    let page = RwSignal::new(1usize);
    floem::views::Stack::vertical((
        wb_title("Pagination"),
        Pagination::new().simple_pagination(page, 10, theme),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_segmented_control(theme: Theme) -> impl View {
    let sel = RwSignal::new(0usize);
    floem::views::Stack::vertical((
        wb_title("Segmented Control"),
        SegmentedControl::new().segmented_control(
            vec!["List".to_string(), "Grid".to_string(), "Map".to_string()],
            sel,
            theme,
        ),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_stepper(theme: Theme) -> impl View {
    let step = RwSignal::new(1usize);
    floem::views::Stack::vertical((
        wb_title("Stepper"),
        Stepper::new("Account")
            .description("Create your account")
            .stepper(vec![
                Stepper::new("Profile").description("Set up your profile"),
                Stepper::new("Plan").description("Choose a plan"),
                Stepper::new("Done").description("All set!"),
            ], step, theme),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_tooltip(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Tooltip"),
        theme.tooltip_core(
            floem::views::Label::new("Hover me (Wait 0.6s)")
                .style(move |s| s.padding(12.0).border(1.0).border_radius(4.0).border_color(theme.border)),
            move || floem::views::Label::new("Tooltip content here"),
        ),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_alert(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Alert"),
        Alert::new().alert("Success!", "Your changes have been saved.", theme, ColorRole::Success, Variant::Flat),
        Alert::new().alert("Warning!", "This action is irreversible.", theme, ColorRole::Warning, Variant::Flat),
        Alert::new().alert("Error!", "Something went wrong.", theme, ColorRole::Danger, Variant::Flat),
    )).style(|s| s.flex_col().gap(12.0).max_width(480.0))
}

fn render_toast(theme: Theme) -> impl View {
    let toasts = RwSignal::new(vec![
        Toast { message: "File saved successfully!".to_string(), color: ColorRole::Success, id: 1 },
        Toast { message: "Network error occurred!".to_string(), color: ColorRole::Danger, id: 2 },
    ]);
    floem::views::Stack::vertical((
        wb_title("Toast"),
        toast_container(toasts, theme),
    )).style(|s| s.flex_col().gap(12.0).max_width(380.0))
}

fn render_accordion(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Accordion"),
        Accordion::new("", floem::views::Empty::new()).accordion(vec![
            Accordion::new("What is Floem?", floem::views::Label::new("A native Rust UI framework.")),
            Accordion::new("What is hero-floem?", floem::views::Label::new("A component library on Floem.")),
        ], RwSignal::new(None::<usize>), theme),
    )).style(|s| s.flex_col().gap(4.0).max_width(500.0))
}

fn render_lucide_icons(theme: Theme) -> impl View {
    let all_icons: &[Icon] = &[
        Icon::Check, Icon::X, Icon::Plus, Icon::Minus, Icon::Search, Icon::Settings,
        Icon::User, Icon::Heart, Icon::Star, Icon::Camera, Icon::Mail, Icon::Phone,
        Icon::House, Icon::ArrowRight, Icon::ArrowLeft, Icon::ChevronDown, Icon::ChevronUp,
        Icon::Info, Icon::TriangleAlert, Icon::Ban, Icon::Copy, Icon::Trash2,
        Icon::Pencil, Icon::Eye, Icon::EyeOff, Icon::Lock, Icon::LockOpen,
        Icon::Bell, Icon::BellOff, Icon::Download, Icon::Upload,
    ];
    let rows: Vec<Box<dyn View>> = all_icons.iter().copied().map(|icon| {
        let th = theme;
        Box::new(floem::views::Stack::vertical((
            icon.view().style(move |s| s.size(28.0, 28.0).color(th.foreground)),
            floem::views::Label::new(icon.get_debug_name()).style(move |s| {
                s.font_size(9.0).color(th.foreground_secondary).max_width(72.0)
            }),
        )).style(|s| s.flex_col().items_center().gap(4.0).width(80.0).padding(8.0))) as Box<dyn View>
    }).collect();

    floem::views::Stack::vertical((
        wb_title("Lucide Icons"),
        floem::views::Label::new("3,063 icons available — showing a subset").style(move |s| s.color(theme.foreground_secondary).margin_bottom(16.0)),
        floem::views::Stack::horizontal_from_iter(rows.into_iter()).style(|s| s.flex_wrap(floem::style::FlexWrap::Wrap).gap(4.0)),
    )).style(|s| s.flex_col().gap(4.0))
}

fn render_carousel(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Carousel"),
        Carousel::new().carousel(
            vec![
                floem::views::Label::new("Slide 1").style(move |s| s.padding(48.0).background(theme.primary.d500)).into_any(),
                floem::views::Label::new("Slide 2").style(move |s| s.padding(48.0).background(theme.success.d500)).into_any(),
                floem::views::Label::new("Slide 3").style(move |s| s.padding(48.0).background(theme.danger.d500)).into_any(),
            ],
            theme,
        ),
    )).style(|s| s.flex_col().gap(8.0).max_width(500.0))
}

fn render_tree_view(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("Tree View"),
        TreeView::new("src/").children(vec![
            TreeView::new("components/").children(vec![
                TreeView::new("button.rs"),
                TreeView::new("input.rs"),
            ]),
            TreeView::new("main.rs"),
        ]).tree_view(vec![
            TreeView::new("src/").children(vec![
                TreeView::new("components/"),
            ]),
            TreeView::new("Cargo.toml"),
        ], RwSignal::new(vec!["src/".to_string()]), RwSignal::new(None::<String>), theme),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_heatmap(theme: Theme) -> impl View {
    let data: Vec<Vec<f32>> = (0..7).map(|row| (0..52).map(|col| ((row + col) as f32 % 7.0) / 7.0).collect()).collect();
    floem::views::Stack::vertical((
        wb_title("Heatmap"),
        Heatmap::new().heatmap(data, 7, 52, ColorRole::Primary, theme),
    )).style(|s| s.flex_col().gap(8.0))
}

fn render_user_card(theme: Theme) -> impl View {
    floem::views::Stack::vertical((
        wb_title("User Card"),
        User::new().user("JD", "Jane Doe", theme),
    )).style(|s| s.flex_col().gap(8.0))
}

fn rand_f32() -> f32 {
    // Simple deterministic "random" for demo data
    static COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let c = COUNTER.fetch_add(7, std::sync::atomic::Ordering::Relaxed);
    ((c % 100) as f32) / 100.0
}

pub fn main() {
    floem::Application::new()
        .window(
            move |_| app_view(),
            Some(
                floem::window::WindowConfig::default()
                    .size(floem::kurbo::Size::new(1200.0, 780.0))
                    .title("hero-floem Widgetbook"),
            ),
        )
        .run()
}

fn render_picker(theme: Theme) -> impl View {
    let sel = RwSignal::new(None::<String>);
    let opts = move || vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string(), "Date".to_string(), "Elderberry".to_string()];
    
    floem::views::Stack::vertical((
        wb_title("Picker (Searchable)"),
        Picker::new().picker_with_input(opts, sel, theme, ColorRole::Primary),
        floem::views::dyn_container(
            move || sel.get(),
            move |val| {
                floem::views::Label::new(format!("Selected: {:?}", val))
                    .style(move |s| s.margin_top(16.0).color(theme.foreground_secondary))
                    .into_any()
            }
        )
    )).style(|s| s.flex_col().gap(8.0).max_width(300.0))
}

fn render_datepicker(theme: Theme) -> impl View {
    let sel = RwSignal::new(None::<chrono::NaiveDate>);
    
    floem::views::Stack::vertical((
        wb_title("Date Picker"),
        theme.date_picker(sel, ColorRole::Primary),
        floem::views::dyn_container(
            move || sel.get(),
            move |val| {
                floem::views::Label::new(if let Some(d) = val { format!("Selected: {}", d) } else { "No date selected".to_string() })
                    .style(move |s| s.margin_top(16.0).color(theme.foreground_secondary))
                    .into_any()
            }
        )
    )).style(|s| s.flex_col().gap(8.0).max_width(320.0))
}
