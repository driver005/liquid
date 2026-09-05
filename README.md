# Hero Floem

A [HeroUI](https://www.heroui.com/)-inspired component system built on top of [Floem](https://github.com/lapce/floem) — the native Rust UI framework by the Lapce team.

Hero Floem brings the polished, modern aesthetic and ergonomic API of HeroUI to Rust desktop applications. It provides a theming system with light/dark mode, color scales, design tokens, and 65+ reusable components with multiple variants and sizes.

## Features

- **Theming system** — light and dark themes with full color scales (50–950 shades per color)
- **Component variants** — Solid, Bordered, Light, Flat, Faded, Shadow, Ghost, Underlined
- **Color roles** — Default, Primary, Secondary, Success, Warning, Danger
- **Size system** — Small, Medium, Large
- **65+ components** — Button, Card, Input, Badge, Avatar, Chip, Switch, Checkbox, Radio, Slider, Modal, Navbar, Tabs, Accordion, Dropdown, Table, Alert, Divider, Link, Spinner, Progress, Skeleton, Sidebar, Breadcrumb, Pagination, Stepper, Command Palette, Drawer, Split View, Data Table, Tree View, Timeline, Stats Card, Carousel, Code Block, Description List, Calendar, Heatmap, Toast, Popover, Tooltip, Loading Overlay, Skeleton Card, Empty State, Select, Multi-Select, Textarea, Toggle Group, Number Input, Date Picker, Color Picker, File Upload, OTP Input, Form Layout, Search Input, Image Card, Gallery, Avatar with Status, List Item, Rating, Segmented Control, Tag Input, Context Menu, Reorderable List, Container, Grid Layout, Spacer, Scroll Area
- **Reactive** — built on Floem's fine-grained signal-based reactivity
- **GPU-accelerated** — inherits Floem's high-performance rendering

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
hero-floem = { git = "https://github.com/yourusername/hero-floem" }
```

Create your first app:

```rust
use hero_floem::prelude::*;

fn app() -> impl View {
    let theme = Theme::light();
    let count = create_rw_signal(0);

    v_stack((
        text("Counter")
            .style(|s| s.font_size(24.0).font_weight(FontWeight::BOLD)),
        dyn_text(move || count.get().to_string()),
        primary_button("Increment", move || count.update(|c| *c += 1), &theme),
    ))
    .style(|s| s.padding(40.0).gap(16.0).flex_col().items_center())
}

fn main() {
    floem::launch(app);
}
```

## Components

### Button

```rust
use hero_floem::prelude::*;

// Variant + color + size combos
button("Click me", || {}, &theme, ButtonProps {
    color: ColorRole::Primary,
    variant: Variant::Shadow,
    size: Size::Lg,
    ..Default::default()
});

// Convenience wrappers
primary_button("Save", || {}, &theme);
secondary_button("Cancel", || {}, &theme);
success_button("Confirm", || {}, &theme);
danger_button("Delete", || {}, &theme);
icon_button("×", || {}, &theme);
```

**Variants:** `Solid`, `Bordered`, `Light`, `Flat`, `Faded`, `Shadow`, `Ghost`, `Underlined`

**Colors:** `Default`, `Primary`, `Secondary`, `Success`, `Warning`, `Danger`

**Sizes:** `Sm`, `Md`, `Lg`

### Card

```rust
card(
    v_stack((
        card_header("Card Title", &theme),
        card_body(text("Card content goes here."), &theme),
        card_footer(
            h_stack((
                secondary_button("Cancel", || {}, &theme),
                primary_button("Save", || {}, &theme),
            )),
            &theme,
        ),
    )),
    &theme,
    CardProps {
        bordered: true,
        shadow: true,
        radius: 16.0,
        padding: 16.0,
    },
);
```

### Input

```rust
let value = create_rw_signal(String::new());

input(value, &theme, InputProps {
    size: Size::Md,
    disabled: false,
    placeholder: "Enter text...".to_string(),
});

labeled_input("Email", value, &theme);
```

### Badge

```rust
badge("New", &theme, ColorRole::Primary, Variant::Solid, Size::Sm);
status_badge("Active", &theme, ColorRole::Success);
```

### Avatar

```rust
avatar("JD", Color::from_rgb8(99, 102, 241), &theme, AvatarProps {
    size: Size::Md,
    bordered: false,
    radius: 9999.0, // circular
});

avatar_group(vec![
    ("A".to_string(), Color::from_rgb8(99, 102, 241)),
    ("B".to_string(), Color::from_rgb8(16, 185, 129)),
], &theme);
```

### Chip

```rust
chip("Rust", || {}, &theme, ColorRole::Primary, Variant::Flat);
chip_group(chips_signal, &theme, ColorRole::Primary);
```

### Switch, Checkbox, Radio, Slider

```rust
let checked = create_rw_signal(true);
switch(checked, &theme, ColorRole::Primary, Size::Md);

let accepted = create_rw_signal(false);
checkbox("Accept terms", accepted, &theme);

let selected = create_rw_signal("option1".to_string());
radio_group(
    vec![("Option 1", "option1"), ("Option 2", "option2")],
    selected,
    &theme,
);

let value = create_rw_signal(50.0f32);
slider(value, 0.0, 100.0, &theme, ColorRole::Primary, Size::Md);
```

### Alert

```rust
alert("Success", "Changes saved.", &theme, ColorRole::Success, Variant::Flat);

let visible = create_rw_signal(true);
dismissible_alert("Info", "Click X to dismiss.", &theme, ColorRole::Primary, visible);
```

### Modal

```rust
let open = create_rw_signal(false);
modal(
    open,
    "Dialog Title",
    text("Modal content"),
    || {},
    &theme,
);
```

### Navbar

```rust
navbar("MyApp", vec![
    ("Home", Box::new(|| {})),
    ("About", Box::new(|| {})),
], &theme);
```

### Tabs

```rust
let active = create_rw_signal(0usize);
tabs(
    vec![
        ("Tab 1", Box::new(|| text("Content 1").into_any())),
        ("Tab 2", Box::new(|| text("Content 2").into_any())),
    ],
    active,
    &theme,
);
```

### Accordion

```rust
let expanded = create_rw_signal(Some(0usize));
accordion(
    vec![
        ("Section 1".to_string(), text("Content 1").into_any()),
        ("Section 2".to_string(), text("Content 2").into_any()),
    ],
    expanded,
    &theme,
);
```

### Table

```rust
table(
    vec![
        Column { label: "Name".to_string(), width: Some(150.0) },
        Column { label: "Status".to_string(), width: None },
    ],
    vec![
        vec!["Alice".to_string(), "Active".to_string()],
        vec!["Bob".to_string(), "Pending".to_string()],
    ],
    &theme,
);
```

### Progress, Spinner, Skeleton

```rust
let progress = create_rw_signal(60.0f32);
progress(progress, 100.0, &theme, ColorRole::Primary, Size::Md);

spinner(&theme, ColorRole::Primary, Size::Md);

skeleton(&theme, 200.0, 12.0, 4.0);
skeleton_text(&theme, 3);
```

### Divider, Link

```rust
divider(&theme);
vertical_divider(&theme, 40.0);

link("Click me", || {}, &theme);
```

## Navigation & Layout Components

### Sidebar

```rust
sidebar(vec![
    SidebarItem { label: "Dashboard".into(), icon: Some("📊"), on_click: None },
    SidebarItem { label: "Settings".into(), icon: Some("⚙"), on_click: None },
], active_signal, &theme, SidebarProps::default());
```

### Breadcrumb

```rust
breadcrumb_simple(vec!["Home", "Projects", "Project X"], &theme);
```

### Pagination

```rust
let current = create_rw_signal(0usize);
pagination(current, 5, &theme);
simple_pagination(current, 10, &theme);
```

### Stepper

```rust
let current = create_rw_signal(1usize);
stepper(vec![
    Step { label: "Account".into(), description: Some("Create account".into()) },
    Step { label: "Profile".into(), description: Some("Set up profile".into()) },
], current, &theme);
```

### Command Palette

```rust
let open = create_rw_signal(false);
command_palette(open, vec![
    Command { label: "Open File".into(), shortcut: Some("Ctrl+O".into()), on_select: Box::new(|| {}) },
], &theme);
```

### Drawer (Slide-over)

```rust
let open = create_rw_signal(false);
drawer(open, "Title", content, || {}, &theme, DrawerProps::default());
```

### Split View

```rust
split_view(pane1, pane2, &theme, SplitViewProps {
    direction: SplitDirection::Horizontal,
    initial_ratio: 0.5,
    ..Default::default()
});
```

## Data Display Components

### Data Table (sortable, selectable)

```rust
data_table(columns, rows, selected, sort_col, sort_asc, &theme, DataTableProps {
    selectable: true, sortable: true, page_size: Some(10),
});
```

### Tree View

```rust
tree_view(vec![
    TreeNode { label: "src".into(), icon: Some("📁"), children: vec![...] },
], expanded, selected, &theme);
```

### Timeline

```rust
timeline(vec![
    TimelineItem { title: "Created".into(), description: Some("...".into()), color: ColorRole::Primary, timestamp: Some("Jan 1".into()) },
], &theme);
```

### Stats Card / KPI Card

```rust
stats_card("Total Users", "12,345", Some(("12%", true)), &theme);
kpi_card("Revenue", "$45K", "$", ColorRole::Primary, &theme);
```

### Carousel

```rust
carousel(vec![slide1.into_any(), slide2.into_any()], &theme);
```

### Code Block

```rust
code_block("fn main() { }", Some("rust"), &theme);
inline_code("let x = 5", &theme);
```

### Description List

```rust
description_list(vec![
    DescriptionEntry { term: "Name".into(), details: "Hero Floem".into() },
], &theme);
```

### Calendar

```rust
let selected = create_rw_signal(None::<(u32, u32, u32)>);
calendar(selected, 2026, 8, &theme);
```

### Heatmap

```rust
heatmap(data_2d, rows, cols, ColorRole::Primary, &theme);
heatmap_legend(ColorRole::Primary, &theme);
```

## Feedback & Overlay Components

### Toast

```rust
let toasts = create_rw_signal(vec![toast("Saved!", ColorRole::Success, 1)]);
toast_container(toasts, &theme);
```

### Popover

```rust
let open = create_rw_signal(false);
popover(trigger, content, open, &theme);
```

### Tooltip

```rust
tooltip(trigger, "Hint text", &theme);
rich_tooltip(trigger, "Title", "Description", &theme);
```

### Loading Overlay

```rust
let visible = create_rw_signal(false);
loading_overlay(visible, &theme);
loading_overlay_with_message(visible, "Saving...", &theme);
```

### Skeleton Card

```rust
skeleton_card(&theme);
skeleton_avatar(&theme, 48.0);
skeleton_button(&theme);
```

### Empty State

```rust
empty_state("📭", "No items", "Description", Some(("Create", Box::new(|| {}))), &theme);
```

## Form & Input Components

### Select / Multi-Select

```rust
select(vec![SelectOption { value: "a".into(), label: "Option A".into() }], selected, &theme);
multi_select(options, selected_vec, &theme);
```

### Textarea

```rust
textarea(value, &theme, "Placeholder", 4);
textarea_with_counter(value, &theme, "Write...", 4, 200);
```

### Toggle Group

```rust
toggle_group(vec!["Day".into(), "Week".into()], selected, &theme);
toggle_group_multi(vec!["Red".into(), "Blue".into()], selected_vec, &theme);
```

### Number Input

```rust
number_input(value_f64, 0.5, Some(0.0), Some(100.0), &theme);
number_input_int(value_i32, 1, Some(0), Some(100), &theme);
```

### Date Picker

```rust
let selected = create_rw_signal(None::<(u32, u32, u32)>);
date_picker(selected, 2026, 8, &theme);
```

### Color Picker

```rust
let color = create_rw_signal(Color::from_rgb8(99, 102, 241));
color_picker(color, &theme);
```

### File Upload

```rust
let files = create_rw_signal(Vec::<String>::new());
file_upload(files, &theme);
```

### OTP Input

```rust
let otp = create_rw_signal(String::new());
otp_input(otp, 6, &theme);
```

### Form Layout

```rust
form_layout(vec![FormField { label: "Name".into(), field: input.into_any(), error: None, hint: None }], &theme);
form_section("Profile", fields, &theme);
```

### Search Input

```rust
search_input(value, &theme);
search_input_with_results(value, results, on_select, &theme);
```

## Content & Media Components

### Image Card

```rust
image_card("Title", "Description", &theme, ImageCardProps::default());
```

### Gallery

```rust
gallery(vec![GalleryItem { bg: Color::BLUE, label: "Blue".into() }], &theme);
masonry_gallery(items, &theme);
```

### Avatar with Status

```rust
avatar_with_status("JD", bg_color, &theme, AvatarStatusProps { online: true, ..Default::default() });
```

### List Item

```rust
list_view(vec![
    ListItemData { icon: Some("📧"), title: "Email".into(), subtitle: Some("New".into()), trailing: Some("2m".into()) },
], &theme);
```

## Interactive Components

### Rating

```rust
let rating = create_rw_signal(3usize);
rating(rating, 5, &theme);
rating_with_label(rating, 5, &theme);
half_rating(rating_f32, 5, &theme);
```

### Segmented Control

```rust
segmented_control(vec!["List".into(), "Grid".into()], selected, &theme);
segmented_control_pills(vec!["All".into(), "Active".into()], selected, &theme);
```

### Tag Input

```rust
let tags = create_rw_signal(vec!["rust".to_string()]);
tag_input(tags, &theme);
```

### Context Menu

```rust
context_menu(trigger, vec![
    ContextMenuItem { label: "Copy".into(), icon: Some("📋"), on_click: Some(Box::new(|| {})), separator: false },
    context_menu_divider(),
], open, &theme);
```

### Reorderable List

```rust
let items = create_rw_signal(vec!["Item 1".to_string(), "Item 2".to_string()]);
reorderable_list(items, &theme);
drag_drop_list(items, &theme);
```

## Layout Utilities

### Container

```rust
container(content, ContainerProps { max_width: 1200.0, centered: true, padding: 24.0 });
container_fluid(content, 24.0);
section(content, &theme);
```

### Grid Layout

```rust
grid_layout(children, GridProps { columns: 3, gap: 16.0, ..Default::default() });
responsive_grid(children, &theme, 16.0);
auto_grid(children, 200.0, 16.0);
```

### Spacer / Flex Utilities

```rust
spacer();               // flex_grow: 1
fixed_spacer(16.0);    // fixed size
flex_grow(content, 2.0);
center(content);
space_between(children);
space_around(children);
```

### Scroll Area

```rust
scroll_area(content, &theme, ScrollAreaProps::default());
scroll_vertical(content, &theme);
scroll_horizontal(content, &theme);
scroll_both(content, &theme);
```

## Theming

The `Theme` struct holds all design tokens. Switch between light and dark:

```rust
let light = Theme::light();
let dark = Theme::dark();
```

Each color role maps to a full 11-step color scale (50–950):

```rust
theme.primary.d500   // main color
theme.primary.d100    // lightest
theme.primary.d950    // darkest
theme.success.d500    // green
theme.danger.d500     // red
```

You can create custom themes by modifying the `Theme` struct fields, or build your own `ColorScale` for custom brand colors.

## Showcase Example

Run the showcase app to see all components in action:

```sh
cargo run --example showcase
```

The showcase demonstrates every component with light/dark mode toggle, all variants, all color roles, and all sizes.

## Architecture

- `src/theme.rs` — Theme struct, color scales, design tokens, light/dark presets
- `src/style.rs` — Style helpers and variant-based style builders
- `src/components/` — All component implementations
- `src/prelude.rs` — Re-exports everything you need (including Floem's own prelude)
- `examples/showcase.rs` — Full demo application

## License

MIT
