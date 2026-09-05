use crate::style::StyleExt;
use floem::style::{BoxShadow, Length, Style};

pub fn make_shadow(offset_y: f32, blur: f32, color: Color, spread: f32) -> Vec<BoxShadow> {
    vec![BoxShadow {
        left_offset: Length::Pt(0.0),
        right_offset: Length::Pt(0.0),
        top_offset: Length::Pt(offset_y as f64),
        bottom_offset: Length::Pt(offset_y as f64),
        blur_radius: Length::Pt(blur as f64),
        spread: Length::Pt(spread as f64),
        color,
    }]
}

use floem::peniko::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Variant {
    Solid,
    Bordered,
    Light,
    Flat,
    Faded,
    Shadow,
    Ghost,
    Underlined,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorRole {
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Size {
    Sm,
    Md,
    Lg,
}

#[derive(Clone, Copy, Debug)]
pub struct ColorScale {
    pub d50: Color,
    pub d100: Color,
    pub d200: Color,
    pub d300: Color,
    pub d400: Color,
    pub d500: Color,
    pub d600: Color,
    pub d700: Color,
    pub d800: Color,
    pub d900: Color,
    pub d950: Color,
}

impl ColorScale {
    pub fn hex10(hexes: [&str; 10], d950: &str) -> Self {
        let c = |h: &str| {
            let h = h.trim_start_matches('#');
            let r = u8::from_str_radix(&h[0..2], 16).unwrap();
            let g = u8::from_str_radix(&h[2..4], 16).unwrap();
            let b = u8::from_str_radix(&h[4..6], 16).unwrap();
            Color::from_rgb8(r, g, b)
        };
        Self {
            d50: c(hexes[0]),
            d100: c(hexes[1]),
            d200: c(hexes[2]),
            d300: c(hexes[3]),
            d400: c(hexes[4]),
            d500: c(hexes[5]),
            d600: c(hexes[6]),
            d700: c(hexes[7]),
            d800: c(hexes[8]),
            d900: c(hexes[9]),
            d950: c(d950),
        }
    }

    pub fn step(&self, i: u32) -> Color {
        match i {
            0 => self.d50,
            1 => self.d100,
            2 => self.d200,
            3 => self.d300,
            4 => self.d400,
            5 => self.d500,
            6 => self.d600,
            7 => self.d700,
            8 => self.d800,
            9 => self.d900,
            _ => self.d950,
        }
    }

    /// Reverses the 50..900 ramp (900<->50, 800<->100, ..., 500<->400) — the same
    /// trick HeroUI's `swapColorValues` uses so a neutral scale reads correctly on a
    /// dark background (light steps become the "visible" end instead of the dark end).
    pub fn swapped(&self) -> Self {
        Self {
            d50: self.d900,
            d100: self.d800,
            d200: self.d700,
            d300: self.d600,
            d400: self.d500,
            d500: self.d400,
            d600: self.d300,
            d700: self.d200,
            d800: self.d100,
            d900: self.d50,
            d950: self.d950,
        }
    }
}

// Exact HeroUI (v2) palette — packages/core/theme/src/colors/*.ts.
// The scales are identical in light and dark mode; HeroUI keeps each role's
// `DEFAULT` (the 500 step) the same across themes and only re-maps the
// neutral/gray scale, which is what `ColorScale::swapped` is for.

pub fn primary_scale() -> ColorScale {
    ColorScale::hex10(
        [
            "#e6f1fe", "#cce3fd", "#99c7fb", "#66aaf9", "#338ef7", "#006FEE", "#005bc4", "#004493",
            "#002e62", "#001731",
        ],
        "#000d1a",
    )
}

pub fn secondary_scale() -> ColorScale {
    ColorScale::hex10(
        [
            "#f2eafa", "#e4d4f4", "#c9a9e9", "#ae7ede", "#9353d3", "#7828c8", "#6020a0", "#481878",
            "#301050", "#180828",
        ],
        "#0c0414",
    )
}

pub fn success_scale() -> ColorScale {
    ColorScale::hex10(
        [
            "#e8faf0", "#d1f4e0", "#a2e9c1", "#74dfa2", "#45d483", "#17c964", "#12a150", "#0e793c",
            "#095028", "#052814",
        ],
        "#02140a",
    )
}

pub fn warning_scale() -> ColorScale {
    ColorScale::hex10(
        [
            "#fefce8", "#fdedd3", "#fbdba7", "#f9c97c", "#f7b750", "#f5a524", "#c4841d", "#936316",
            "#62420e", "#312107",
        ],
        "#191104",
    )
}

pub fn danger_scale() -> ColorScale {
    ColorScale::hex10(
        [
            "#fee7ef", "#fdd0df", "#faa0bf", "#f871a0", "#f54180", "#f31260", "#c20e4d", "#920b3a",
            "#610726", "#310413",
        ],
        "#18020a",
    )
}

/// HeroUI's "zinc" neutral scale — backs the `Default` color role and the
/// library's own gray surfaces/borders.
pub fn neutral_scale() -> ColorScale {
    ColorScale::hex10(
        [
            "#fafafa", "#f4f4f5", "#e4e4e7", "#d4d4d8", "#a1a1aa", "#71717a", "#52525b", "#3f3f46",
            "#27272a", "#18181b",
        ],
        "#0c0c0d",
    )
}

#[derive(Clone, Debug, Copy)]
pub struct Theme {
    pub mode: ThemeMode,
    pub primary: ColorScale,
    pub secondary: ColorScale,
    pub success: ColorScale,
    pub warning: ColorScale,
    pub danger: ColorScale,
    pub neutral: ColorScale,
    pub background: Color,
    pub background_elevated: Color,
    pub foreground: Color,
    pub foreground_secondary: Color,
    pub border: Color,
    pub border_hover: Color,
    pub divider: Color,
    pub overlay: Color,
    pub content1: Color,
    pub content2: Color,
    pub content3: Color,
    pub default: Color,
    pub default_foreground: Color,
    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_lg: f32,
    pub font_family: &'static str,
    pub font_size_sm: f32,
    pub font_size_md: f32,
    pub font_size_lg: f32,
    pub spacing_unit: f32,
    pub shadow_color: Color,
}

impl Theme {
    pub fn light() -> Self {
        let neutral = neutral_scale();
        Self {
            mode: ThemeMode::Light,
            primary: primary_scale(),
            secondary: secondary_scale(),
            success: success_scale(),
            warning: warning_scale(),
            danger: danger_scale(),
            background: Color::from_rgb8(255, 255, 255),
            background_elevated: Color::from_rgb8(255, 255, 255),
            foreground: Color::from_rgb8(0x11, 0x18, 0x1c),
            foreground_secondary: neutral.d500,
            border: neutral.d200,
            border_hover: neutral.d300,
            divider: Color::from_rgb8(17, 17, 17).with_alpha(0.15),
            overlay: Color::BLACK.with_alpha(0.5),
            content1: Color::from_rgb8(255, 255, 255),
            content2: neutral.d100,
            content3: neutral.d200,
            default: neutral.d300,
            default_foreground: Color::from_rgb8(0x11, 0x18, 0x1c),
            radius_sm: 8.0,
            radius_md: 12.0,
            radius_lg: 14.0,
            font_family: "sans-serif",
            font_size_sm: 14.0,
            font_size_md: 16.0,
            font_size_lg: 18.0,
            spacing_unit: 8.0,
            shadow_color: Color::BLACK.with_alpha(0.10),
            neutral,
        }
    }

    pub fn dark() -> Self {
        let neutral = neutral_scale().swapped();
        Self {
            mode: ThemeMode::Dark,
            primary: primary_scale(),
            secondary: secondary_scale(),
            success: success_scale(),
            warning: warning_scale(),
            danger: danger_scale(),
            background: Color::from_rgb8(0, 0, 0),
            background_elevated: neutral.d900,
            foreground: Color::from_rgb8(0xec, 0xed, 0xee),
            foreground_secondary: neutral.d500,
            border: neutral.d700,
            border_hover: neutral.d600,
            divider: Color::WHITE.with_alpha(0.15),
            overlay: Color::BLACK.with_alpha(0.7),
            content1: neutral.d900,
            content2: neutral.d800,
            content3: neutral.d700,
            default: neutral.d700,
            default_foreground: Color::from_rgb8(0xec, 0xed, 0xee),
            radius_sm: 8.0,
            radius_md: 12.0,
            radius_lg: 14.0,
            font_family: "sans-serif",
            font_size_sm: 14.0,
            font_size_md: 16.0,
            font_size_lg: 18.0,
            spacing_unit: 8.0,
            shadow_color: Color::BLACK.with_alpha(0.35),
            neutral,
        }
    }

    pub fn scale_for(&self, role: ColorRole) -> &ColorScale {
        match role {
            ColorRole::Default => &self.neutral,
            ColorRole::Primary => &self.primary,
            ColorRole::Secondary => &self.secondary,
            ColorRole::Success => &self.success,
            ColorRole::Warning => &self.warning,
            ColorRole::Danger => &self.danger,
        }
    }

    pub fn color_for(&self, role: ColorRole) -> Color {
        match role {
            ColorRole::Default => self.default,
            ColorRole::Primary => self.primary.d500,
            ColorRole::Secondary => self.secondary.d500,
            ColorRole::Success => self.success.d500,
            ColorRole::Warning => self.warning.d500,
            ColorRole::Danger => self.danger.d500,
        }
    }

    pub fn radius(&self, size: Size) -> f32 {
        match size {
            Size::Sm => self.radius_sm,
            Size::Md => self.radius_md,
            Size::Lg => self.radius_lg,
        }
    }

    pub fn font_size(&self, size: Size) -> f32 {
        match size {
            Size::Sm => self.font_size_sm,
            Size::Md => self.font_size_md,
            Size::Lg => self.font_size_lg,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

pub fn with_alpha(color: Color, alpha: f32) -> Color {
    color.with_alpha(alpha)
}

impl Theme {
    pub fn button_style(
        &self,
        color: ColorRole,
        variant: Variant,
        size: Size,
        disabled: bool,
    ) -> Style {
        let scale = self.scale_for(color);
        let radius = self.radius(size);
        let font_size = self.font_size(size);

        let (pad_y, pad_x) = match size {
            Size::Sm => (4.0, 10.0),
            Size::Md => (6.0, 14.0),
            Size::Lg => (10.0, 20.0),
        };

        let mut base = Style::new()
            .padding_xy(pad_x, pad_y)
            .border_radius(radius)
            .font_size(font_size)
            .font_family(self.font_family)
            .flex_row()
            .items_center()
            .justify_center()
            .cursor(floem::style::CursorStyle::Pointer)
            .transition_colors()
            .focus_ring(self.primary.d500);

        base = match variant {
            Variant::Solid => {
                let bg = if color == ColorRole::Default {
                    self.default
                } else {
                    scale.d500
                };
                let fg = if color == ColorRole::Default {
                    self.default_foreground
                } else {
                    Color::WHITE
                };
                base.background(bg).color(fg)
            }
            Variant::Bordered => {
                let _border_color = if color == ColorRole::Default {
                    self.border
                } else {
                    scale.d500
                };
                let fg = if color == ColorRole::Default {
                    self.foreground
                } else {
                    scale.d500
                };
                base.border(1.5).color(fg).background(Color::TRANSPARENT)
            }
            Variant::Light => {
                let bg = if color == ColorRole::Default {
                    self.content3
                } else {
                    scale.d100
                };
                let fg = if color == ColorRole::Default {
                    self.foreground
                } else {
                    scale.d700
                };
                base.background(bg).color(fg)
            }
            Variant::Flat => {
                let fg = if color == ColorRole::Default {
                    self.foreground
                } else {
                    scale.d700
                };
                base.color(fg).background(Color::TRANSPARENT)
            }
            Variant::Faded => {
                let bg = if color == ColorRole::Default {
                    self.content2
                } else {
                    scale.d50
                };
                let fg = if color == ColorRole::Default {
                    self.foreground
                } else {
                    scale.d700
                };
                base.background(bg).color(fg).border(1.0)
            }
            Variant::Shadow => {
                let bg = if color == ColorRole::Default {
                    self.default
                } else {
                    scale.d500
                };
                let fg = if color == ColorRole::Default {
                    self.default_foreground
                } else {
                    Color::WHITE
                };
                base.background(bg).color(fg).box_shadow(make_shadow(
                    4.0,
                    12.0,
                    self.shadow_color,
                    0.0,
                ))
            }
            Variant::Ghost => {
                let fg = if color == ColorRole::Default {
                    self.foreground
                } else {
                    scale.d700
                };
                base.color(fg).background(Color::TRANSPARENT)
            }
            Variant::Underlined => {
                let fg = if color == ColorRole::Default {
                    self.foreground
                } else {
                    scale.d500
                };
                base.color(fg)
                    .background(Color::TRANSPARENT)
                    .border_bottom(2.0)
            }
        };

        if disabled {
            base = base.opacity(0.5);
        }

        base
    }

    pub fn button_hover_style(&self, color: ColorRole, variant: Variant) -> Style {
        let scale = self.scale_for(color);

        match variant {
            Variant::Solid => {
                let bg = if color == ColorRole::Default {
                    self.content3
                } else {
                    scale.d600
                };
                Style::new().background(bg)
            }
            Variant::Bordered | Variant::Ghost | Variant::Underlined => {
                let bg = if color == ColorRole::Default {
                    self.content2
                } else {
                    scale.d50
                };
                Style::new().background(bg)
            }
            Variant::Light | Variant::Faded => {
                let bg = if color == ColorRole::Default {
                    self.content3
                } else {
                    scale.d200
                };
                Style::new().background(bg)
            }
            Variant::Flat => {
                let bg = if color == ColorRole::Default {
                    self.content2
                } else {
                    scale.d100
                };
                Style::new().background(bg)
            }
            Variant::Shadow => {
                let bg = if color == ColorRole::Default {
                    self.content3
                } else {
                    scale.d600
                };
                Style::new().background(bg).box_shadow(make_shadow(
                    6.0,
                    16.0,
                    self.shadow_color,
                    0.0,
                ))
            }
        }
    }

    pub fn input_style(&self, size: Size, disabled: bool) -> Style {
        let radius = self.radius(size);
        let font_size = self.font_size(size);
        let (pad_y, pad_x) = match size {
            Size::Sm => (4.0, 8.0),
            Size::Md => (8.0, 12.0),
            Size::Lg => (10.0, 14.0),
        };

        let mut s = Style::new()
            .padding_xy(pad_x, pad_y)
            .border_radius(radius)
            .border(1.0)
            .border_color(self.border)
            .background(self.background_elevated)
            .color(self.foreground)
            .font_size(font_size)
            .font_family(self.font_family)
            .width_full()
            .transition_colors()
            .focus_ring(self.primary.d500);

        if disabled {
            s = s.opacity(0.5);
        }

        s
    }

    pub fn card_style(&self, radius: f32, bordered: bool, shadow: bool) -> Style {
        let mut s = Style::new()
            .background(self.background_elevated)
            .border_radius(radius)
            .padding(16.0)
            .flex_col();

        if bordered {
            s = s.border(1.0);
        }

        if shadow {
            s = s.box_shadow(make_shadow(2.0, 8.0, self.shadow_color, 0.0));
        }

        s
    }

    pub fn badge_style(&self, color: ColorRole, variant: Variant, size: Size) -> Style {
        let scale = self.scale_for(color);
        let radius = self.radius(size);
        let font_size = match size {
            Size::Sm => 11.0,
            Size::Md => 12.0,
            Size::Lg => 13.0,
        };
        let (pad_y, pad_x) = match size {
            Size::Sm => (2.0, 6.0),
            Size::Md => (3.0, 8.0),
            Size::Lg => (4.0, 10.0),
        };

        let mut base = Style::new()
            .padding_xy(pad_x, pad_y)
            .border_radius(radius)
            .font_size(font_size)
            .font_bold()
            .flex_row()
            .items_center()
            .justify_center();

        base = match variant {
            Variant::Solid => {
                let bg = if color == ColorRole::Default {
                    self.default
                } else {
                    scale.d500
                };
                let fg = if color == ColorRole::Default {
                    self.default_foreground
                } else {
                    Color::WHITE
                };
                base.background(bg).color(fg)
            }
            Variant::Bordered => {
                let _border_color = if color == ColorRole::Default {
                    self.border
                } else {
                    scale.d500
                };
                let fg = if color == ColorRole::Default {
                    self.foreground
                } else {
                    scale.d700
                };
                base.border(1.0).color(fg).background(Color::TRANSPARENT)
            }
            Variant::Flat | Variant::Faded => {
                let bg = if color == ColorRole::Default {
                    self.content2
                } else {
                    scale.d100
                };
                let fg = if color == ColorRole::Default {
                    self.foreground
                } else {
                    scale.d700
                };
                base.background(bg).color(fg)
            }
            _ => {
                let bg = if color == ColorRole::Default {
                    self.content2
                } else {
                    scale.d100
                };
                let fg = if color == ColorRole::Default {
                    self.foreground
                } else {
                    scale.d700
                };
                base.background(bg).color(fg)
            }
        };

        base
    }

    pub fn alert_style(&self, color: ColorRole, variant: Variant) -> Style {
        let scale = self.scale_for(color);
        let radius = self.radius(Size::Md);

        let mut base = Style::new()
            .padding(12.0)
            .border_radius(radius)
            .flex_row()
            .items_start()
            .gap(8.0)
            .width_full();

        base = match variant {
            Variant::Solid => {
                let bg = if color == ColorRole::Default {
                    self.default
                } else {
                    scale.d500
                };
                let fg = if color == ColorRole::Default {
                    self.default_foreground
                } else {
                    Color::WHITE
                };
                base.background(bg).color(fg)
            }
            Variant::Bordered => {
                let _border_color = if color == ColorRole::Default {
                    self.border
                } else {
                    scale.d500
                };
                let fg = if color == ColorRole::Default {
                    self.foreground
                } else {
                    scale.d800
                };
                base.border(1.0).color(fg).background(Color::TRANSPARENT)
            }
            Variant::Flat => {
                let fg = if color == ColorRole::Default {
                    self.foreground
                } else {
                    scale.d800
                };
                base.color(fg).background(Color::TRANSPARENT)
            }
            _ => {
                let bg = if color == ColorRole::Default {
                    self.content2
                } else {
                    scale.d50
                };
                let fg = if color == ColorRole::Default {
                    self.foreground
                } else {
                    scale.d800
                };
                base.background(bg).color(fg)
            }
        };

        base
    }
    pub fn checkbox_style(&self, is_on: bool) -> Style {
        Style::new()
            .size(20.0, 20.0)
            .border_radius(4.0)
            .border(2.0)
            .border_color(if is_on {
                self.primary.d500
            } else {
                self.border
            })
            .background(if is_on {
                self.primary.d500
            } else {
                floem::peniko::Color::TRANSPARENT
            })
            .color(floem::peniko::Color::WHITE)
            .cursor(floem::style::CursorStyle::Pointer)
            .transition_colors()
            .focus_ring(self.primary.d500)
    }

    pub fn radio_style(&self, is_on: bool) -> Style {
        Style::new()
            .size(20.0, 20.0)
            .border_radius(9999.0)
            .border(2.0)
            .border_color(if is_on {
                self.primary.d500
            } else {
                self.border
            })
            .background(floem::peniko::Color::TRANSPARENT)
            .cursor(floem::style::CursorStyle::Pointer)
            .transition_colors()
            .focus_ring(self.primary.d500)
    }

    pub fn select_trigger_style(&self, radius: f32) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .width_full()
            .padding_xy(12.0, 8.0)
            .border(1.0)
            .border_color(self.border)
            .border_radius(radius)
            .background(self.background_elevated)
            .cursor(floem::style::CursorStyle::Pointer)
            .transition_colors()
    }

    pub fn menu_style(&self, radius: f32) -> Style {
        Style::new()
            .flex_col()
            .width_full()
            .padding(4.0)
            .background(self.background_elevated)
            .border(1.0)
            .border_color(self.border)
            .border_radius(radius)
            .box_shadow(make_shadow(4.0, 12.0, self.shadow_color, 8.0))
    }

    pub fn textarea_style(&self, disabled: bool) -> Style {
        let mut s = Style::new()
            .padding_xy(12.0, 8.0)
            .border_radius(self.radius_md)
            .border(1.0)
            .border_color(self.border)
            .background(self.background_elevated)
            .color(self.foreground)
            .font_size(self.font_size_md)
            .width_full()
            .transition_colors()
            .focus_ring(self.primary.d500);
        if disabled {
            s = s.opacity(0.5);
        }
        s
    }

    pub fn search_input_style(&self) -> Style {
        Style::new()
            .width_full()
            .border(1.0)
            .border_color(self.border)
            .border_radius(self.radius_md)
            .background(self.background_elevated)
            .color(self.foreground)
            .font_size(14.0)
            .font_family(self.font_family)
            .transition_colors()
            .focus_ring(self.primary.d500)
    }

    pub fn dropdown_trigger_style(&self) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .width_full()
            .padding_xy(12.0, 8.0)
            .border(1.0)
            .border_color(self.border)
            .border_radius(self.radius_md)
            .background(self.background_elevated)
            .cursor(floem::style::CursorStyle::Pointer)
            .transition_colors()
    }

    pub fn input_container_style(&self) -> Style {
        Style::new()
            .width_full()
            .padding_xy(12.0, 8.0)
            .border(1.0)
            .border_color(self.border)
            .border_radius(self.radius_md)
            .background(self.background_elevated)
            .color(self.foreground)
            .font_size(14.0)
            .font_family(self.font_family)
            .transition_colors()
            .focus_ring(self.primary.d500)
    }

    pub fn otp_cell_style(&self) -> Style {
        Style::new()
            .size(48.0, 48.0)
            .border(1.0)
            .border_color(self.border)
            .border_radius(self.radius_md)
            .background(self.background_elevated)
            .color(self.foreground)
            .font_size(20.0)
            .font_weight(floem::text::FontWeight::BOLD)
            .text_align(floem::text::Alignment::Center)
            .transition_colors()
            .focus_ring(self.primary.d500)
    }

    pub fn color_picker_swatch_style(&self, color: floem::peniko::Color, is_sel: bool) -> Style {
        Style::new()
            .size(28.0, 28.0)
            .border_radius(self.radius_sm)
            .background(color)
            .border(2.0)
            .border_color(if is_sel {
                self.foreground
            } else {
                floem::peniko::Color::TRANSPARENT
            })
            .cursor(floem::style::CursorStyle::Pointer)
            .hover(move |s| {
                s.border(2.0)
                    .border_color(self.foreground_secondary)
                    .border_radius(self.radius_sm)
            })
    }

    pub fn drag_drop_zone_style(&self, is_active: bool) -> Style {
        Style::new()
            .width_full()
            .height(150.0)
            .flex_col()
            .items_center()
            .justify_center()
            .gap(12.0)
            .border(1.0)
            .border_color(if is_active {
                self.primary.d500
            } else {
                self.border
            })
            .border_radius(self.radius_sm)
            .background(if is_active {
                self.primary.d50
            } else {
                self.background_elevated
            })
            .cursor(floem::style::CursorStyle::Pointer)
            .transition_colors()
    }

    pub fn file_upload_zone_style(&self, is_dragging: bool) -> Style {
        Style::new()
            .width_full()
            .padding_xy(24.0, 32.0)
            .flex_col()
            .items_center()
            .justify_center()
            .gap(12.0)
            .border(2.0)
            .border_color(if is_dragging {
                self.primary.d500
            } else {
                self.border
            })
            .border_radius(self.radius_md)
            .background(if is_dragging {
                self.primary.d50
            } else {
                self.background_elevated
            })
            .cursor(floem::style::CursorStyle::Pointer)
            .transition_colors()
            .hover(move |s| {
                s.border(2.0)
                    .border_color(self.primary.d300)
                    .border_radius(self.radius_md)
                    .background(self.primary.d50)
            })
    }

    pub fn drag_drop_item_style(&self, is_dragging: bool) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .gap(8.0)
            .width_full()
            .padding_xy(12.0, 10.0)
            .border(1.0)
            .border_color(if is_dragging {
                self.primary.d300
            } else {
                self.border
            })
            .border_radius(self.radius_sm)
            .background(if is_dragging {
                self.primary.d50
            } else {
                self.background_elevated
            })
            .opacity(if is_dragging { 0.7 } else { 1.0 })
            .transition_colors()
    }

    pub fn segmented_control_container_style(&self) -> Style {
        Style::new()
            .flex_row()
            .gap(2.0)
            .padding(3.0)
            .border_radius(self.radius_md)
            .background(self.content2)
    }

    pub fn segmented_control_segment_style(&self, is_sel: bool) -> Style {
        Style::new()
            .font_size(13.0)
            .padding_xy(16.0, 8.0)
            .color(if is_sel {
                Color::WHITE
            } else {
                self.foreground_secondary
            })
            .background(if is_sel {
                self.primary.d500
            } else {
                Color::TRANSPARENT
            })
            .border_radius(self.radius_sm)
            .flex_grow(1.0)
            .cursor(floem::style::CursorStyle::Pointer)
            .transition_colors()
    }

    pub fn segmented_control_segment_hover_style(&self, is_sel: bool) -> Style {
        Style::new().background(if is_sel {
            self.primary.d600
        } else {
            self.content3
        })
    }

    pub fn segmented_control_pills_container_style(&self) -> Style {
        Style::new().flex_row().gap(8.0)
    }

    pub fn segmented_control_pills_segment_style(&self, is_sel: bool) -> Style {
        Style::new()
            .font_size(13.0)
            .padding_xy(14.0, 6.0)
            .color(if is_sel {
                self.primary.d700
            } else {
                self.foreground_secondary
            })
            .background(if is_sel {
                self.primary.d100
            } else {
                Color::TRANSPARENT
            })
            .border(1.0)
            .border_color(if is_sel {
                self.primary.d300
            } else {
                self.border
            })
            .border_radius(9999.0)
            .cursor(floem::style::CursorStyle::Pointer)
            .transition_colors()
    }

    pub fn segmented_control_pills_segment_hover_style(&self, is_sel: bool) -> Style {
        Style::new().background(if is_sel {
            self.primary.d200
        } else {
            self.content2
        })
    }

    pub fn select_option_label_style(&self, is_sel: bool) -> Style {
        Style::new()
            .font_size(14.0)
            .color(if is_sel {
                self.primary.d500
            } else {
                self.foreground
            })
            .flex_grow(1.0)
    }

    pub fn select_option_check_style(&self) -> Style {
        Style::new().font_size(14.0).color(self.primary.d500)
    }

    pub fn select_option_style(&self) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .width_full()
            .padding_xy(12.0, 8.0)
            .cursor(floem::style::CursorStyle::Pointer)
            .transition_colors()
    }

    pub fn select_option_hover_style(&self) -> Style {
        Style::new().background(self.content2)
    }

    pub fn select_trigger_label_style(&self) -> Style {
        Style::new()
            .font_size(14.0)
            .color(self.foreground)
            .flex_grow(1.0)
    }

    pub fn select_trigger_icon_style(&self) -> Style {
        Style::new()
            .font_size(12.0)
            .color(self.foreground_secondary)
    }

    pub fn multi_select_trigger_style(&self) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .width_full()
            .padding_xy(12.0, 8.0)
            .border(1.0)
            .border_color(self.border)
            .border_radius(self.radius_md)
            .background(self.background_elevated)
    }

    pub fn multi_select_option_checkbox_style(&self, is_sel: bool) -> Style {
        Style::new()
            .size(16.0, 16.0)
            .border(2.0)
            .border_color(if is_sel {
                self.primary.d500
            } else {
                self.border
            })
            .border_radius(4.0)
            .background(if is_sel {
                self.primary.d500
            } else {
                Color::TRANSPARENT
            })
    }

    pub fn multi_select_option_label_style(&self) -> Style {
        Style::new()
            .font_size(14.0)
            .color(self.foreground)
            .flex_grow(1.0)
    }

    pub fn alert_icon_style(&self, color: ColorRole) -> Style {
        Style::new().font_size(18.0).color(self.color_for(color))
    }

    pub fn alert_title_style(&self, color: ColorRole) -> Style {
        Style::new()
            .font_size(14.0)
            .font_weight(floem::text::FontWeight::BOLD)
            .color(self.color_for(color))
    }

    pub fn alert_message_style(&self) -> Style {
        Style::new()
            .font_size(13.0)
            .color(self.foreground_secondary)
            .margin_top(4.0)
    }

    pub fn alert_dismissible_message_style(&self) -> Style {
        Style::new()
            .font_size(13.0)
            .color(self.foreground_secondary)
    }

    pub fn alert_close_style(&self) -> Style {
        Style::new()
            .font_size(14.0)
            .color(self.foreground_secondary)
            .cursor(floem::style::CursorStyle::Pointer)
            .transition_colors()
    }

    pub fn alert_close_hover_style(&self) -> Style {
        Style::new().color(self.foreground)
    }

    pub fn dropdown_menu_style(&self) -> Style {
        Style::new()
            .flex_col()
            .padding(4.0)
            .background(self.background_elevated)
            .border(1.0)
            .border_color(self.border)
            .border_radius(self.radius_md)
            .box_shadow(crate::theme::make_shadow(4.0, 16.0, self.shadow_color, 8.0))
            .absolute()
            .inset_top(100.0)
            .inset_left(0.0)
            .z_index(80)
    }

    pub fn dropdown_item_style(&self) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .gap(8.0)
            .width_full()
            .padding_xy(12.0, 8.0)
            .border_radius(self.radius_sm)
            .cursor(floem::style::CursorStyle::Pointer)
            .transition_colors()
    }

    pub fn dropdown_item_hover_style(&self) -> Style {
        Style::new().background(self.content2)
    }

    pub fn dropdown_item_label_style(&self) -> Style {
        Style::new()
            .font_size(13.0)
            .color(self.foreground)
            .flex_grow(1.0)
    }

    pub fn dropdown_item_icon_style(&self) -> Style {
        Style::new()
            .font_size(14.0)
            .color(self.foreground_secondary)
            .width(20.0)
    }

    pub fn dropdown_divider_style(&self) -> Style {
        Style::new()
            .width_full()
            .height(1.0)
            .background(self.divider)
            .margin_vert(4.0)
            .margin_horiz(0.0)
    }

    pub fn data_table_style(&self) -> Style {
        Style::new()
            .flex_col()
            .width_full()
            .border(1.0)
            .border_color(self.border)
            .border_radius(self.radius_lg)
            .background(self.background)
    }

    pub fn data_table_header_row_style(&self) -> Style {
        Style::new()
            .flex_row()
            .width_full()
            .background(self.content1)
            .border_bottom(1.0)
            .border_bottom_color(self.border)
    }

    pub fn data_table_header_cell_style(&self) -> Style {
        Style::new()
            .padding_xy(16.0, 12.0)
            .flex_row()
            .items_center()
            .gap(4.0)
    }

    pub fn data_table_header_label_style(&self) -> Style {
        Style::new()
            .font_size(12.0)
            .font_weight(floem::text::FontWeight::BOLD)
            .color(self.foreground_secondary)
    }

    pub fn data_table_row_style(&self, is_last: bool) -> Style {
        Style::new()
            .flex_row()
            .width_full()
            .transition_colors()
            .apply_if(!is_last, |s| {
                s.border_bottom(1.0).border_bottom_color(self.border)
            })
    }

    pub fn data_table_row_hover_style(&self) -> Style {
        Style::new().background(self.content1)
    }

    pub fn data_table_cell_style(&self) -> Style {
        Style::new()
            .padding_xy(16.0, 16.0)
            .flex_row()
            .items_center()
    }

    pub fn input_hover_style(&self) -> Style {
        Style::new().border_color(self.border_hover)
    }

    pub fn select_trigger_hover_style(&self) -> Style {
        Style::new().border_color(self.border_hover)
    }

    pub fn control_label_style(&self) -> Style {
        Style::new()
            .color(self.foreground)
            .font_size(14.0)
            .cursor(floem::style::CursorStyle::Pointer)
    }

    pub fn control_container_style(&self) -> Style {
        Style::new().flex_row().items_center().gap(8.0)
    }

    pub fn radio_group_container_style(&self) -> Style {
        Style::new().flex_col().gap(8.0)
    }

    pub fn slider_container_style(&self, height: f32) -> Style {
        Style::new()
            .width_full()
            .height(height * 2.0)
            .flex_row()
            .items_center()
    }

    pub fn switch_thumb_style(&self, size: Size, is_on: bool) -> Style {
        let (track_w, _track_h, thumb_size, pad) = match size {
            Size::Sm => (28.0, 16.0, 12.0, 2.0),
            Size::Md => (36.0, 20.0, 16.0, 2.0),
            Size::Lg => (48.0, 26.0, 22.0, 2.0),
        };
        let thumb_offset = track_w - thumb_size - pad;
        let thumb_x = if is_on { thumb_offset } else { pad };
        Style::new()
            .width(thumb_size)
            .height(thumb_size)
            .border_radius(9999.0)
            .background(floem::peniko::Color::WHITE)
            .box_shadow(crate::theme::make_shadow(
                1.0,
                2.0,
                floem::peniko::Color::BLACK.with_alpha(0.2),
                2.0,
            ))
            .absolute()
            .inset_left(thumb_x)
            .inset_top(pad)
            .transition(
                floem::style::InsetLeft,
                floem::style::Transition::ease_in_out(std::time::Duration::from_secs_f64(0.15)),
            )
    }

    pub fn switch_track_style(&self, size: Size, is_on: bool, color: ColorRole) -> Style {
        let (track_w, track_h, _thumb_size, _pad) = match size {
            Size::Sm => (28.0, 16.0, 12.0, 2.0),
            Size::Md => (36.0, 20.0, 16.0, 2.0),
            Size::Lg => (48.0, 26.0, 22.0, 2.0),
        };
        let scale = *self.scale_for(color);
        let bg = if is_on {
            if color == ColorRole::Default {
                self.foreground_secondary
            } else {
                scale.d500
            }
        } else {
            self.border
        };

        Style::new()
            .width(track_w)
            .height(track_h)
            .border_radius(9999.0)
            .background(bg)
            .cursor(floem::style::CursorStyle::Pointer)
            .transition(
                floem::style::Background,
                floem::style::Transition::linear(std::time::Duration::from_secs_f64(0.15)),
            )
    }

    pub fn switch_label_style(&self) -> Style {
        Style::new().color(self.foreground).font_size(14.0)
    }

    pub fn switch_container_style(&self) -> Style {
        Style::new().flex_row().items_center().gap(8.0)
    }

    pub fn textarea_counter_label_style(&self, is_over: bool) -> Style {
        Style::new().font_size(11.0).color(if is_over {
            self.danger.d500
        } else {
            self.foreground_secondary
        })
    }

    pub fn textarea_counter_container_style(&self) -> Style {
        Style::new().flex_row().items_center().width_full()
    }

    pub fn textarea_container_style(&self) -> Style {
        Style::new().flex_col().gap(4.0).width_full()
    }

    pub fn tooltip_style(&self) -> Style {
        Style::new()
            .font_size(12.0)
            .color(floem::peniko::Color::WHITE)
            .padding_xy(8.0, 4.0)
            .border_radius(6.0)
            .background(floem::peniko::Color::from_rgb8(30, 30, 35))
            .box_shadow(crate::theme::make_shadow(
                2.0,
                8.0,
                floem::peniko::Color::BLACK.with_alpha(0.3),
                4.0,
            ))
            .absolute()
            .inset_bottom(100.0)
            .inset_left(0.0)
            .z_index(200)
            .opacity(0.0)
            .transition(
                floem::style::Opacity,
                floem::style::Transition::ease_in_out(std::time::Duration::from_secs_f64(0.15)),
            )
    }

    pub fn rich_tooltip_title_style(&self) -> Style {
        Style::new()
            .font_size(13.0)
            .font_weight(floem::text::FontWeight::BOLD)
            .color(floem::peniko::Color::WHITE)
    }

    pub fn rich_tooltip_desc_style(&self) -> Style {
        Style::new()
            .font_size(12.0)
            .color(floem::peniko::Color::from_rgb8(200, 200, 210))
    }

    pub fn rich_tooltip_panel_style(&self) -> Style {
        Style::new()
            .flex_col()
            .gap(4.0)
            .padding(10.0)
            .border_radius(8.0)
            .background(floem::peniko::Color::from_rgb8(30, 30, 35))
            .box_shadow(crate::theme::make_shadow(
                2.0,
                8.0,
                floem::peniko::Color::BLACK.with_alpha(0.3),
                4.0,
            ))
            .absolute()
            .inset_bottom(100.0)
            .inset_left(0.0)
            .z_index(200)
            .max_width(250.0)
            .opacity(0.0)
    }

    pub fn avatar_style(
        &self,
        size: Size,
        bordered: bool,
        radius: f32,
        bg_color: floem::peniko::Color,
    ) -> Style {
        let dim = match size {
            Size::Sm => 28.0,
            Size::Md => 36.0,
            Size::Lg => 48.0,
        };
        let font_size = match size {
            Size::Sm => 12.0,
            Size::Md => 14.0,
            Size::Lg => 18.0,
        };

        let mut s = Style::new()
            .width(dim)
            .height(dim)
            .background(bg_color)
            .color(floem::peniko::Color::WHITE)
            .font_size(font_size)
            .font_weight(floem::text::FontWeight::BOLD)
            .border_radius(radius)
            .flex_row()
            .items_center()
            .justify_center();
        if bordered {
            s = s.border(2.0).border_color(self.border);
        }
        s
    }

    pub fn avatar_group_style(&self) -> Style {
        Style::new().flex_row().gap(-8.0)
    }

    pub fn tab_button_style(&self, is_active: bool) -> Style {
        let mut s = Style::new()
            .padding_xy(12.0, 8.0)
            .font_size(14.0)
            .cursor(floem::style::CursorStyle::Pointer)
            .color(if is_active {
                self.foreground
            } else {
                self.foreground_secondary
            });

        if is_active {
            s = s
                .font_weight(floem::text::FontWeight::BOLD)
                .border_bottom(2.0)
                .border_bottom_color(self.primary.d500);
        } else {
            s = s
                .border_bottom(2.0)
                .border_bottom_color(floem::peniko::Color::TRANSPARENT);
        }

        s.transition_colors()
    }

    pub fn tab_button_hover_style(&self) -> Style {
        Style::new().color(self.foreground)
    }

    pub fn tab_content_style(&self, is_active: bool) -> Style {
        Style::new()
            .apply_if(!is_active, |s| s.hide())
            .padding(16.0)
            .width_full()
    }

    pub fn tabs_header_style(&self) -> Style {
        Style::new()
            .flex_row()
            .gap(4.0)
            .border_bottom(1.0)
            .border_bottom_color(self.border)
            .width_full()
    }

    pub fn tabs_content_container_style(&self) -> Style {
        Style::new().flex_col().width_full()
    }

    pub fn tabs_container_style(&self) -> Style {
        Style::new().flex_col().width_full()
    }

    pub fn drawer_overlay_style(&self) -> Style {
        Style::new()
            .width_full()
            .height_full()
            .background(self.overlay)
            .absolute()
            .inset_left(0.0)
            .inset_top(0.0)
            .z_index(50)
    }

    pub fn drawer_panel_style(
        &self,
        bordered: bool,
        side: &crate::components::base::overlay::drawer::DrawerSide,
    ) -> Style {
        let mut s = Style::new()
            .background(self.background_elevated)
            .box_shadow(crate::theme::make_shadow(
                8.0,
                32.0,
                self.shadow_color,
                16.0,
            ))
            .flex_col();

        match side {
            crate::components::base::overlay::drawer::DrawerSide::Left => {
                s = s.absolute().inset_left(0.0).inset_top(0.0).z_index(51);
                if bordered {
                    s = s.border_right(1.0).border_color(self.border);
                }
            }
            crate::components::base::overlay::drawer::DrawerSide::Right => {
                s = s.absolute().inset_right(0.0).inset_top(0.0).z_index(51);
                if bordered {
                    s = s.border_left(1.0).border_color(self.border);
                }
            }
            crate::components::base::overlay::drawer::DrawerSide::Top => {
                s = s.absolute().inset_left(0.0).inset_top(0.0).z_index(51);
                if bordered {
                    s = s.border_bottom(1.0).border_color(self.border);
                }
            }
            crate::components::base::overlay::drawer::DrawerSide::Bottom => {
                s = s.absolute().inset_left(0.0).inset_bottom(0.0).z_index(51);
                if bordered {
                    s = s.border_top(1.0).border_color(self.border);
                }
            }
        }
        s
    }

    pub fn drawer_container_style(&self) -> Style {
        Style::new()
            .width_full()
            .height_full()
            .absolute()
            .inset_left(0.0)
            .inset_top(0.0)
            .z_index(50)
    }

    pub fn popover_panel_style(&self) -> Style {
        Style::new()
            .background(self.background_elevated)
            .border(1.0)
            .border_color(self.border)
            .border_radius(self.radius_md)
            .padding(12.0)
            .box_shadow(crate::theme::make_shadow(4.0, 16.0, self.shadow_color, 8.0))
            .absolute()
            .inset_top(100.0)
            .inset_left(0.0)
            .z_index(40)
            .min_width(200.0)
    }

    pub fn popover_header_style(&self) -> Style {
        Style::new()
            .font_size(14.0)
            .font_weight(floem::text::FontWeight::BOLD)
            .color(self.foreground)
            .margin_bottom(8.0)
    }

    pub fn sidebar_item_icon_style(&self, is_active: bool) -> Style {
        Style::new()
            .font_size(16.0)
            .width(20.0)
            .color(if is_active {
                self.primary.d500
            } else {
                self.foreground_secondary
            })
    }

    pub fn sidebar_item_label_style(&self, is_active: bool, show: bool) -> Style {
        Style::new()
            .font_size(14.0)
            .color(if is_active {
                self.foreground
            } else {
                self.foreground_secondary
            })
            .font_weight(if is_active {
                floem::text::FontWeight::BOLD
            } else {
                floem::text::FontWeight::NORMAL
            })
            .apply_if(!show, |s| s.hide())
    }

    pub fn sidebar_item_container_style(&self, is_active: bool) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .gap(10.0)
            .width_full()
            .padding_xy(12.0, 10.0)
            .border_radius(self.radius_sm)
            .cursor(floem::style::CursorStyle::Pointer)
            .background(if is_active {
                self.primary.d50
            } else {
                floem::peniko::Color::TRANSPARENT
            })
            .transition_colors()
    }

    pub fn sidebar_item_container_hover_style(&self, is_active: bool) -> Style {
        Style::new().background(if is_active {
            self.primary.d100
        } else {
            self.content2
        })
    }

    pub fn sidebar_toggle_button_style(&self) -> Style {
        Style::new()
            .font_size(12.0)
            .color(self.foreground_secondary)
            .cursor(floem::style::CursorStyle::Pointer)
            .padding(4.0)
    }

    pub fn sidebar_toggle_button_hover_style(&self) -> Style {
        Style::new().color(self.foreground)
    }

    pub fn sidebar_container_style(&self, width: f32, bordered: bool) -> Style {
        let mut s = Style::new()
            .width(width)
            .height_full()
            .flex_col()
            .background(self.background_elevated)
            .transition(
                floem::style::Width,
                floem::style::Transition::ease_in_out(std::time::Duration::from_secs_f64(0.2)),
            );
        if bordered {
            s = s.border_right(1.0).border_right_color(self.border);
        }
        s
    }

    pub fn sidebar_section_title_style(&self) -> Style {
        Style::new()
            .font_size(11.0)
            .font_weight(floem::text::FontWeight::BOLD)
            .color(self.foreground_secondary)
            .padding_xy(12.0, 4.0)
    }

    pub fn command_palette_overlay_style(&self) -> Style {
        Style::new()
            .width_full()
            .height_full()
            .background(self.overlay)
            .absolute()
            .inset_left(0.0)
            .inset_top(0.0)
            .z_index(60)
    }

    pub fn command_palette_shortcut_style(&self) -> Style {
        Style::new()
            .font_size(11.0)
            .color(self.foreground_secondary)
            .padding_xy(6.0, 2.0)
            .border_radius(4.0)
            .background(self.content3)
    }

    pub fn command_palette_item_label_style(&self) -> Style {
        Style::new()
            .font_size(14.0)
            .color(self.foreground)
            .flex_grow(1.0)
    }

    pub fn command_palette_item_container_style(&self, is_sel: bool) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .width_full()
            .padding_xy(12.0, 10.0)
            .background(if is_sel {
                self.content2
            } else {
                floem::peniko::Color::TRANSPARENT
            })
            .cursor(floem::style::CursorStyle::Pointer)
    }

    pub fn command_palette_item_container_hover_style(&self) -> Style {
        Style::new().background(self.content2)
    }

    pub fn command_palette_list_container_style(&self) -> Style {
        Style::new()
            .flex_col()
            .border_top(1.0)
            .border_color(self.border)
            .max_height(400.0)
    }

    pub fn command_palette_panel_style(&self) -> Style {
        Style::new()
            .width(500.0)
            .background(self.background_elevated)
            .border_radius(self.radius_lg)
            .border(1.0)
            .border_color(self.border)
            .box_shadow(crate::theme::make_shadow(
                8.0,
                32.0,
                self.shadow_color,
                16.0,
            ))
            .flex_col()
            .overflow_x(floem::taffy::style::Overflow::Hidden)
            .overflow_y(floem::taffy::style::Overflow::Hidden)
    }

    pub fn command_palette_wrapper_style(&self) -> Style {
        Style::new()
            .absolute()
            .inset_left(0.0)
            .inset_top(0.0)
            .inset_right(0.0)
            .inset_bottom(0.0)
            .margin_left(floem::style::LengthAuto::Auto)
            .margin_right(floem::style::LengthAuto::Auto)
            .margin_top(floem::style::LengthAuto::Auto)
            .margin_bottom(floem::style::LengthAuto::Auto)
            .z_index(61)
    }

    pub fn command_palette_container_style(&self) -> Style {
        Style::new()
            .width_full()
            .height_full()
            .absolute()
            .inset_left(0.0)
            .inset_top(0.0)
            .z_index(60)
    }

    pub fn date_picker_label_style(&self) -> Style {
        Style::new()
            .font_size(14.0)
            .color(self.foreground)
            .flex_grow(1.0)
    }

    pub fn date_picker_icon_style(&self) -> Style {
        Style::new()
            .font_size(14.0)
            .color(self.foreground_secondary)
    }

    pub fn date_picker_trigger_style(&self) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .gap(8.0)
            .cursor(floem::style::CursorStyle::Pointer)
    }

    pub fn date_picker_popover_style(&self) -> Style {
        Style::new()
            .absolute()
            .inset_top(100.0)
            .inset_left(0.0)
            .z_index(40)
            .box_shadow(crate::theme::make_shadow(4.0, 16.0, self.shadow_color, 8.0))
    }

    pub fn file_upload_icon_style(&self) -> Style {
        Style::new()
            .font_size(40.0)
            .color(self.foreground_secondary)
            .opacity(0.5)
    }

    pub fn file_upload_title_style(&self) -> Style {
        Style::new()
            .font_size(14.0)
            .color(self.foreground_secondary)
            .margin_top(8.0)
    }

    pub fn file_upload_subtitle_style(&self) -> Style {
        Style::new()
            .font_size(12.0)
            .color(self.foreground_secondary)
            .opacity(0.7)
            .margin_top(4.0)
    }

    pub fn file_upload_item_icon_style(&self) -> Style {
        Style::new().font_size(16.0)
    }

    pub fn file_upload_item_name_style(&self) -> Style {
        Style::new()
            .font_size(13.0)
            .color(self.foreground)
            .flex_grow(1.0)
    }

    pub fn file_upload_item_delete_style(&self) -> Style {
        Style::new()
            .font_size(14.0)
            .color(self.foreground_secondary)
            .cursor(floem::style::CursorStyle::Pointer)
    }

    pub fn file_upload_item_delete_hover_style(&self) -> Style {
        Style::new().color(self.danger.d500)
    }

    pub fn file_upload_item_container_style(&self) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .gap(8.0)
            .padding_xy(12.0, 8.0)
            .border(1.0)
            .border_color(self.border)
            .border_radius(self.radius_sm)
            .background(self.background_elevated)
    }

    pub fn file_upload_list_container_style(&self) -> Style {
        Style::new()
            .flex_col()
            .gap(4.0)
            .width_full()
            .margin_top(8.0)
    }

    pub fn file_upload_container_style(&self) -> Style {
        Style::new().flex_col().gap(8.0).width_full()
    }

    pub fn card_header_style(&self) -> Style {
        Style::new()
            .font_size(16.0)
            .font_weight(floem::text::FontWeight::BOLD)
            .color(self.foreground)
            .margin_bottom(8.0)
    }

    pub fn card_body_style(&self) -> Style {
        Style::new()
            .color(self.foreground_secondary)
            .font_size(14.0)
    }

    pub fn card_footer_style(&self) -> Style {
        Style::new()
            .margin_top(12.0)
            .padding_top(12.0)
            .border_top(1.0)
            .border_color(self.divider)
            .flex_row()
            .items_center()
            .justify_end()
            .gap(8.0)
    }

    pub fn image_card_style(&self, bordered: bool, shadow: bool) -> Style {
        let mut card_style = Style::new()
            .background(self.background_elevated)
            .border_radius(self.radius_md)
            .flex_col()
            .overflow_x(floem::taffy::style::Overflow::Hidden)
            .overflow_y(floem::taffy::style::Overflow::Hidden);

        if bordered {
            card_style = card_style.border(1.0).border_color(self.border);
        }
        if shadow {
            card_style =
                card_style.box_shadow(crate::theme::make_shadow(2.0, 8.0, self.shadow_color, 6.0));
        }
        card_style
    }

    pub fn image_card_image_style(&self, bg: floem::peniko::Color, height: f32) -> Style {
        Style::new().width_full().height(height).background(bg)
    }

    pub fn image_card_title_style(&self) -> Style {
        Style::new()
            .font_size(16.0)
            .font_weight(floem::text::FontWeight::BOLD)
            .color(self.foreground)
    }

    pub fn image_card_desc_style(&self) -> Style {
        Style::new()
            .font_size(13.0)
            .color(self.foreground_secondary)
            .margin_top(4.0)
    }

    pub fn image_card_content_container_style(&self) -> Style {
        Style::new().flex_col().gap(0.0).padding(16.0)
    }

    pub fn toast_icon_style(&self, color: ColorRole) -> Style {
        let scale = self.scale_for(color);
        Style::new().font_size(16.0).color(scale.d500)
    }

    pub fn toast_message_style(&self) -> Style {
        Style::new().font_size(14.0).color(self.foreground)
    }

    pub fn toast_item_container_style(&self, color: ColorRole) -> Style {
        let scale = self.scale_for(color);
        Style::new()
            .flex_row()
            .items_center()
            .gap(10.0)
            .padding_vert(12.0)
            .padding_horiz(16.0)
            .border_radius(self.radius_md)
            .background(self.background_elevated)
            .border(1.0)
            .border_color(self.border)
            .border_left(4.0)
            .border_left_color(scale.d500)
            .box_shadow(crate::theme::make_shadow(4.0, 12.0, self.shadow_color, 8.0))
            .width(360.0)
    }

    pub fn toast_list_container_style(&self) -> Style {
        Style::new()
            .absolute()
            .width_full()
            .height_full()
            .flex_col()
            .justify_end()
            .items_end()
            .padding(20.0)
            .gap(8.0)
            .z_index(100)
            // Ensure pointer events only hit the toasts themselves, not the invisible full-screen container
            // (floem doesn't have pointer_events(None) but it ignores clicks on transparent background anyway)
    }

    pub fn alert_header_container_style(&self) -> Style {
        Style::new().flex_row().items_center().gap(6.0)
    }

    pub fn alert_dismissible_icon_container_style(&self) -> Style {
        Style::new().flex_col()
    }

    pub fn alert_dismissible_content_container_style(&self) -> Style {
        Style::new().flex_col().gap(2.0)
    }

    pub fn alert_dismissible_spacer_style(&self) -> Style {
        Style::new().flex_grow(1.0)
    }

    pub fn breadcrumb_item_style(&self, is_last: bool) -> Style {
        Style::new()
            .font_size(13.0)
            .color(if is_last {
                self.foreground
            } else {
                self.foreground_secondary
            })
            .cursor(floem::style::CursorStyle::Pointer)
    }

    pub fn breadcrumb_item_hover_style(&self) -> Style {
        Style::new().color(self.foreground)
    }

    pub fn breadcrumb_separator_style(&self) -> Style {
        Style::new()
            .font_size(13.0)
            .color(self.foreground_secondary)
            .opacity(0.5)
    }

    pub fn breadcrumb_container_style(&self) -> Style {
        Style::new().flex_row().items_center().gap(8.0)
    }

    pub fn chip_close_button_style(&self, close_fg: floem::peniko::Color) -> Style {
        Style::new()
            .font_size(12.0)
            .color(close_fg)
            .cursor(floem::style::CursorStyle::Pointer)
            .transition_colors()
    }

    pub fn chip_close_button_hover_style(&self) -> Style {
        Style::new().opacity(0.7)
    }

    pub fn chip_label_style(&self, fg: floem::peniko::Color) -> Style {
        Style::new().font_size(13.0).color(fg)
    }

    pub fn chip_container_style(
        &self,
        bg: floem::peniko::Color,
        border_color: floem::peniko::Color,
        radius: f32,
    ) -> Style {
        Style::new()
            .padding_xy(8.0, 4.0)
            .border_radius(radius)
            .background(bg)
            .border(1.0)
            .border_color(border_color)
            .flex_row()
            .items_center()
            .gap(4.0)
    }

    pub fn chip_group_container_style(&self) -> Style {
        Style::new()
            .flex_row()
            .gap(6.0)
            .flex_wrap(floem::taffy::style::FlexWrap::Wrap)
    }

    pub fn user_name_style(&self, name_font: f32) -> Style {
        Style::new()
            .font_size(name_font)
            .font_weight(floem::text::FontWeight::MEDIUM)
            .color(self.foreground)
    }

    pub fn user_desc_style(&self, desc_font: f32) -> Style {
        Style::new()
            .font_size(desc_font)
            .color(self.foreground_secondary)
    }

    pub fn user_text_col_style(&self) -> Style {
        Style::new().flex_col().gap(1.0)
    }

    pub fn user_container_style(&self) -> Style {
        Style::new().flex_row().items_center().gap(8.0)
    }

    pub fn carousel_slide_style(&self, is_current: bool) -> Style {
        Style::new()
            .width_full()
            .height_full()
            .apply_if(!is_current, |s| s.hide())
    }

    pub fn carousel_nav_button_style(&self) -> Style {
        Style::new()
            .font_size(24.0)
            .color(floem::peniko::Color::WHITE)
            .size(36.0, 36.0)
            .border_radius(9999.0)
            .background(floem::peniko::Color::BLACK.with_alpha(0.4))
            .flex_row()
            .items_center()
            .justify_center()
            .cursor(floem::style::CursorStyle::Pointer)
    }

    pub fn carousel_nav_button_hover_style(&self) -> Style {
        Style::new().background(floem::peniko::Color::BLACK.with_alpha(0.6))
    }

    pub fn carousel_dot_style(&self, is_current: bool) -> Style {
        Style::new()
            .size(if is_current { 24.0 } else { 8.0 }, 8.0)
            .border_radius(9999.0)
            .background(if is_current {
                self.primary.d500
            } else {
                self.content3
            })
            .cursor(floem::style::CursorStyle::Pointer)
    }

    pub fn carousel_slides_container_style(&self) -> Style {
        Style::new().flex_col().width_full().height_full()
    }

    pub fn carousel_nav_container_style(&self) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .width_full()
            .padding_xy(16.0, 0.0)
            .absolute()
            .inset_top(0.0)
            .height_full()
    }

    pub fn carousel_nav_spacer_style(&self) -> Style {
        Style::new().flex_grow(1.0)
    }

    pub fn carousel_dots_container_style(&self) -> Style {
        Style::new()
            .flex_row()
            .gap(6.0)
            .absolute()
            .inset_bottom(12.0)
            .inset_left(0.0)
            .width_full()
            .justify_center()
    }

    pub fn carousel_container_style(&self) -> Style {
        Style::new()
            .width_full()
            .height(300.0)
            .border_radius(self.radius_md)
            .overflow_x(floem::taffy::style::Overflow::Hidden)
            .overflow_y(floem::taffy::style::Overflow::Hidden)
            .background(self.content2)
    }

    pub fn color_picker_trigger_style(&self, selected: floem::peniko::Color) -> Style {
        Style::new()
            .size(36.0, 36.0)
            .border_radius(self.radius_sm)
            .background(selected)
            .border(1.0)
            .border_color(self.border)
            .cursor(floem::style::CursorStyle::Pointer)
    }

    pub fn color_picker_header_style(&self) -> Style {
        Style::new()
            .font_size(12.0)
            .font_weight(floem::text::FontWeight::BOLD)
            .color(self.foreground)
            .margin_bottom(8.0)
    }

    pub fn color_picker_row_style(&self) -> Style {
        Style::new().flex_row().gap(6.0).padding(4.0)
    }

    pub fn color_picker_swatch_grid_style(&self) -> Style {
        Style::new().flex_col().gap(6.0)
    }

    pub fn color_picker_panel_style(&self) -> Style {
        Style::new()
            .flex_col()
            .padding(12.0)
            .background(self.background_elevated)
            .border(1.0)
            .border_color(self.border)
            .border_radius(self.radius_md)
            .box_shadow(crate::theme::make_shadow(4.0, 16.0, self.shadow_color, 8.0))
            .absolute()
            .inset_top(100.0)
            .inset_left(0.0)
            .z_index(40)
    }

    pub fn container_style(&self, max_width: f32, centered: bool, padding: f32) -> Style {
        let mut s = Style::new().max_width(max_width).padding(padding);
        if centered {
            s = s
                .margin_left(floem::style::LengthAuto::Auto)
                .margin_right(floem::style::LengthAuto::Auto);
        }
        s
    }

    pub fn container_fluid_style(&self, padding: f32) -> Style {
        Style::new().width_full().padding(padding)
    }

    pub fn container_section_style(&self) -> Style {
        Style::new()
            .width_full()
            .padding(48.0)
            .background(self.background)
            .flex_col()
    }

    pub fn debug_overlay_header_style(&self) -> Style {
        Style::new()
            .font_size(12.0)
            .font_weight(floem::text::FontWeight::BOLD)
            .color(self.foreground)
    }

    pub fn debug_overlay_label_style(&self) -> Style {
        Style::new()
            .font_size(12.0)
            .color(self.foreground_secondary)
            .margin_right(4.0)
    }

    pub fn debug_overlay_value_style(&self, color: floem::peniko::Color) -> Style {
        Style::new()
            .font_size(12.0)
            .font_weight(floem::text::FontWeight::BOLD)
            .color(color)
    }

    pub fn debug_overlay_container_style(&self) -> Style {
        Style::new()
            .position(floem::style::Position::Absolute)
            .inset_top(16.0)
            .inset_right(16.0)
            .padding_xy(12.0, 8.0)
            .background(self.background)
            .border(1.0)
            .border_color(self.border)
            .border_radius(self.radius_md)
            .box_shadow_blur(8.0)
            .box_shadow_color(floem::peniko::Color::from_rgba8(0, 0, 0, 40))
            .z_index(1000)
            .flex_col()
            .gap(4.0)
    }

    pub fn heatmap_cell_style(&self, alpha: f32, scale_d500: floem::peniko::Color) -> Style {
        Style::new()
            .size(12.0, 12.0)
            .border_radius(2.0)
            .background(if alpha == 0.0 {
                self.content2
            } else {
                scale_d500.with_alpha(alpha)
            })
    }

    pub fn heatmap_row_style(&self) -> Style {
        Style::new().flex_row().gap(3.0)
    }

    pub fn heatmap_container_style(&self) -> Style {
        Style::new().flex_col().gap(3.0)
    }

    pub fn heatmap_legend_label_style(&self) -> Style {
        Style::new()
            .font_size(10.0)
            .color(self.foreground_secondary)
    }

    pub fn heatmap_legend_cells_style(&self) -> Style {
        Style::new().flex_row().gap(2.0)
    }

    pub fn heatmap_legend_container_style(&self) -> Style {
        Style::new().flex_row().items_center().gap(4.0)
    }

    pub fn data_table_cell_container_style(&self) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .padding(12.0)
            .flex_grow(1.0)
    }

    pub fn data_table_checkbox_container_style(&self) -> Style {
        Style::new().padding(12.0).width(40.0)
    }

    pub fn dropdown_label_style(&self) -> Style {
        Style::new()
            .font_size(14.0)
            .color(self.foreground)
            .flex_grow(1.0)
    }

    pub fn dropdown_icon_style(&self) -> Style {
        Style::new()
            .font_size(12.0)
            .color(self.foreground_secondary)
    }

    pub fn dropdown_simple_label_style(&self) -> Style {
        Style::new().font_size(14.0).color(self.foreground)
    }

    pub fn sidebar_header_container_style(&self) -> Style {
        Style::new().flex_row().padding(8.0)
    }

    pub fn sidebar_items_container_style(&self) -> Style {
        Style::new().flex_col().gap(2.0).padding(8.0)
    }

    pub fn timeline_dot_style(&self, scale_d500: floem::peniko::Color) -> Style {
        Style::new()
            .size(12.0, 12.0)
            .border_radius(9999.0)
            .background(scale_d500)
            .border(2.0)
            .border_color(self.background)
            .margin_top(4.0)
    }

    pub fn timeline_line_style(&self, is_last: bool) -> Style {
        Style::new()
            .width(2.0)
            .flex_grow(1.0)
            .background(self.divider)
            .apply_if(is_last, |s| s.hide())
    }

    pub fn timeline_left_container_style(&self) -> Style {
        Style::new()
            .flex_col()
            .items_center()
            .gap(4.0)
            .height_full()
    }

    pub fn timeline_title_style(&self) -> Style {
        Style::new()
            .font_size(14.0)
            .font_weight(floem::text::FontWeight::BOLD)
            .color(self.foreground)
    }

    pub fn timeline_desc_style(&self) -> Style {
        Style::new()
            .font_size(13.0)
            .color(self.foreground_secondary)
    }

    pub fn timeline_timestamp_style(&self) -> Style {
        Style::new()
            .font_size(11.0)
            .color(self.foreground_secondary)
            .margin_top(4.0)
    }

    pub fn timeline_right_container_style(&self) -> Style {
        Style::new().flex_col().gap(2.0).padding_bottom(20.0)
    }

    pub fn timeline_item_container_style(&self) -> Style {
        Style::new().flex_row().items_start().gap(16.0).width_full()
    }

    pub fn timeline_list_container_style(&self) -> Style {
        Style::new().flex_col().gap(0.0).width_full()
    }

    pub fn tree_view_arrow_style(&self) -> Style {
        Style::new()
            .font_size(10.0)
            .color(self.foreground_secondary)
            .width(16.0)
    }

    pub fn tree_view_icon_style(&self) -> Style {
        Style::new()
            .font_size(14.0)
            .color(self.foreground_secondary)
            .width(20.0)
    }

    pub fn tree_view_label_style(&self, is_sel: bool) -> Style {
        Style::new()
            .font_size(14.0)
            .color(if is_sel {
                self.primary.d500
            } else {
                self.foreground
            })
            .font_weight(if is_sel {
                floem::text::FontWeight::BOLD
            } else {
                floem::text::FontWeight::NORMAL
            })
            .cursor(floem::style::CursorStyle::Pointer)
    }

    pub fn tree_view_row_style(&self, depth: usize, is_sel: bool) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .gap(4.0)
            .padding_xy(8.0, 4.0)
            .margin_left(depth as f32 * 20.0)
            .border_radius(self.radius_sm)
            .background(if is_sel {
                self.primary.d50
            } else {
                floem::peniko::Color::TRANSPARENT
            })
    }

    pub fn tree_view_row_hover_style(&self) -> Style {
        Style::new().background(self.content2)
    }

    pub fn tree_view_children_container_style(&self) -> Style {
        Style::new().flex_col().gap(2.0).width_full()
    }

    pub fn tree_view_node_container_style(&self) -> Style {
        Style::new().flex_col().gap(2.0).width_full()
    }

    pub fn tree_view_container_style(&self) -> Style {
        Style::new().flex_col().gap(2.0).width_full()
    }

    pub fn listbox_title_style(&self) -> Style {
        Style::new().font_size(14.0).color(self.foreground)
    }

    pub fn listbox_desc_style(&self) -> Style {
        Style::new()
            .font_size(12.0)
            .color(self.foreground_secondary)
    }

    pub fn listbox_label_style(&self) -> Style {
        Style::new()
            .font_size(14.0)
            .color(self.foreground)
            .flex_grow(1.0)
    }

    pub fn listbox_check_style(&self, is_selected: bool) -> Style {
        Style::new()
            .font_size(13.0)
            .color(self.primary.d500)
            .apply_if(!is_selected, |s| s.hide())
    }

    pub fn listbox_row_style(&self, disabled: bool) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .gap(8.0)
            .width_full()
            .padding_xy(12.0, 8.0)
            .border_radius(self.radius_sm)
            .transition_colors()
            .apply_if(disabled, |s| s.opacity(0.5))
            .apply_if(!disabled, |s| s.cursor(floem::style::CursorStyle::Pointer))
    }

    pub fn listbox_row_hover_style(&self) -> Style {
        Style::new().background(self.content2)
    }

    pub fn listbox_container_style(&self) -> Style {
        Style::new().flex_col().gap(1.0).width_full()
    }

    pub fn menu_separator_style(&self) -> Style {
        Style::new()
            .width_full()
            .height(1.0)
            .background(self.divider)
            .margin_vert(4.0)
            .margin_horiz(0.0)
    }

    pub fn menu_icon_style(&self, text_color: floem::peniko::Color) -> Style {
        Style::new().font_size(14.0).color(text_color).width(20.0)
    }

    pub fn menu_label_style(&self, text_color: floem::peniko::Color) -> Style {
        Style::new()
            .font_size(13.0)
            .color(text_color)
            .flex_grow(1.0)
    }

    pub fn menu_row_style(&self) -> Style {
        Style::new()
            .flex_row()
            .items_center()
            .gap(8.0)
            .width_full()
            .padding_xy(12.0, 8.0)
            .border_radius(self.radius_sm)
            .cursor(floem::style::CursorStyle::Pointer)
            .transition_colors()
    }

    pub fn menu_row_hover_style(&self, danger: bool) -> Style {
        Style::new().background(if danger {
            self.danger.d50
        } else {
            self.content2
        })
    }

    pub fn menu_panel_style(&self) -> Style {
        Style::new()
            .flex_col()
            .padding(4.0)
            .background(self.background_elevated)
            .border(1.0)
            .border_color(self.border)
            .border_radius(self.radius_md)
            .box_shadow(crate::theme::make_shadow(4.0, 16.0, self.shadow_color, 8.0))
            .absolute()
            .inset_top(100.0)
            .inset_left(0.0)
            .z_index(80)
            .min_width(180.0)
    }
}
