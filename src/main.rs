mod app;
mod config;
mod i18n;
mod sensors;
mod theme;
mod ui;

use app::LtmApp;
use config::AppConfig;
use eframe::egui::{self, viewport::WindowLevel};
use std::sync::Arc;

fn load_app_icon() -> Option<Arc<egui::IconData>> {
    let icon_path = std::path::Path::new("assets/icons/app_icon.png");
    if icon_path.exists() {
        if let Ok(bytes) = std::fs::read(icon_path) {
            if let Ok(img) = image::load_from_memory(&bytes) {
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                return Some(Arc::new(egui::IconData {
                    rgba: rgba.into_raw(),
                    width,
                    height,
                }));
            }
        }
    }
    None
}

fn main() -> eframe::Result {
    let config = AppConfig::load();
    let initial_level = if config.always_on_top {
        WindowLevel::AlwaysOnTop
    } else {
        WindowLevel::Normal
    };

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([380.0, 500.0])
        .with_min_inner_size([320.0, 400.0])
        .with_transparent(true)
        .with_window_level(initial_level)
        .with_title("LTM - Lightweight Temperature Monitor");

    if let Some(icon) = load_app_icon() {
        viewport = viewport.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "LTM",
        options,
        Box::new(|_cc| Ok(Box::new(LtmApp::default()))),
    )
}