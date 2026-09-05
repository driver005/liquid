use floem::{
    action::{exec_after, exec_after_animation_frame},
    peniko::Color,
    reactive::RwSignal,
    reactive::{SignalGet, SignalUpdate},
    view::View,
    views::Decorators,
};
use std::time::Duration;
use sysinfo::System;

use crate::theme::Theme;

#[derive(Default, Clone)]
pub struct DebugOverlay {}
impl DebugOverlay {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn debug_overlay(self, theme: Theme) -> impl View {
        let fps = floem::reactive::RwSignal::new(0.0f32);
        let ram_usage = floem::reactive::RwSignal::new(0u64); // in MB

        // Initialize RAM tracking
        let sys = System::new_all();
        Self::update_ram(ram_usage, sys);

        let frames = floem::reactive::RwSignal::new(0u32);
        Self::count_frames(frames);
        Self::report_fps(fps, frames);

        floem::views::Stack::vertical((
            floem::views::Label::new("Performance Monitor")
                .style(move |s| s.apply(theme.debug_overlay_header_style())),
            floem::views::Stack::horizontal((
                floem::views::Label::new("FPS:")
                    .style(move |s| s.apply(theme.debug_overlay_label_style())),
                floem::views::Label::derived(move || {
                    if fps.get() == 0.0 {
                        "0.0 (Idle)".to_string()
                    } else {
                        format!("{:.1}", fps.get())
                    }
                })
                .style(move |s| {
                    let f = fps.get();
                    let color = if f >= 50.0 {
                        Color::from_rgb8(16, 185, 129) // Green
                    } else if f >= 30.0 {
                        Color::from_rgb8(245, 158, 11) // Yellow
                    } else {
                        Color::from_rgb8(239, 68, 68) // Red
                    };
                    s.font_size(12.0)
                        .font_weight(floem::text::FontWeight::BOLD)
                        .color(color)
                }),
            )),
            floem::views::Stack::horizontal((
                floem::views::Label::new("RAM:")
                    .style(move |s| s.apply(theme.debug_overlay_label_style())),
                floem::views::Label::derived(move || format!("{} MB", ram_usage.get())).style({
                    let primary_d500 = theme.primary.d500.clone();
                    move |s| s.apply(theme.debug_overlay_value_style(primary_d500))
                }),
            )),
        ))
        .style(move |s| s.apply(theme.debug_overlay_container_style()))
    }

    fn update_ram(ram_signal: RwSignal<u64>, mut sys: System) {
        exec_after(Duration::from_millis(500), move |_| {
            sys.refresh_memory();
            ram_signal.set(sys.used_memory() / 1024 / 1024);
            Self::update_ram(ram_signal, sys);
        });
    }

    fn count_frames(frames: RwSignal<u32>) {
        exec_after_animation_frame(move |_| {
            frames.update(|c| *c += 1);
            Self::count_frames(frames);
        });
    }

    fn report_fps(fps_signal: RwSignal<f32>, frames: RwSignal<u32>) {
        floem::action::exec_after(std::time::Duration::from_secs(1), move |_| {
            let count = frames.get_untracked();
            frames.set(0);

            // Only update if frames were actually drawn
            // If 0 frames were drawn, Floem is idling to save power, so FPS isn't truly 0,
            // but we can report it as 0 or leave it at the last known FPS. Let's just report the idle state as 0.0 for now, or you could skip updating it.
            fps_signal.set(count as f32);

            Self::report_fps(fps_signal, frames);
        });
    }
}
