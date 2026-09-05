//! # Hero Floem
//!
//! A HeroUI-inspired component system built on top of [Floem](https://github.com/lapce/floem).
//!
//! This crate provides a set of beautiful, themeable, reusable UI components
//! for building desktop applications with Rust and Floem.
//!
//! ## Features
//!
//! - **Theming system** with light/dark mode, color scales, and design tokens
//! - **Component variants** inspired by HeroUI (Solid, Bordered, Light, Flat, Faded, Shadow, Ghost, Underlined)
//! - **Color roles** (Default, Primary, Secondary, Success, Warning, Danger)
//! - **Size system** (Small, Medium, Large)
//! - **20+ components** including Button, Card, Input, Badge, Avatar, Chip,
//!   Switch, Checkbox, Radio, Slider, Modal, Navbar, Tabs, Accordion, Dropdown,
//!   Table, Alert, Divider, Link, Spinner, Progress, Skeleton
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use hero_floem::prelude::*;
//!
//! fn app() -> impl View {
//!     let theme = Theme::light();
//!     let count = floem::reactive::RwSignal::new(0);
//!
//!     floem::views::Stack::vertical((
//!         floem::views::Label::new("Counter").style(|s| s.font_size(24.0).font_weight(FontWeight::BOLD)),
//!         floem::views::Label::derived(move || count.get().to_string()),
//!         primary_button("Increment", move || count.update(|c| *c += 1), &theme),
//!     ))
//!     .style(|s| s.padding(40.0).gap(16.0).flex_col().items_center())
//! }
//!
//! fn main() {
//!     floem::launch(app);
//! }
//! ```

pub mod components;
pub mod icon;
pub mod prelude;
pub mod style;
pub mod theme;
