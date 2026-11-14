use eframe::egui::{self, ColorImage, CornerRadius, Margin, RichText, TextureHandle, TextureOptions};
use std::collections::HashMap;
use std::time::Instant;
use crate::config::AppConfig;
use crate::i18n;
use crate::sensors::mock::get_mock_sensors;
use crate::sensors::real::RealSensorReader;
use crate::sensors::Sensor;
use crate::theme::Theme;
use crate::ui::{draw_sensor, draw_settings};

pub struct LtmApp {
    pub sensors: Vec<Sensor>,
    pub config: AppConfig,
    pub show_settings: bool,
    pub language_search: String,
    pub save_status: Option<String>,
    pub settings_texture: Option<TextureHandle>,
    pub settings_icon_angle: f32,
    pub sensor_reader: RealSensorReader,
    pub last_update: Instant,
}

impl Default for LtmApp {
    fn default() -> Self {
        let mut sensors = get_mock_sensors();
        let mut reader = RealSensorReader::new();
        reader.update_temperatures(&mut sensors);

        Self {
            sensors,
            config: AppConfig::load(),
            show_settings: false,
            language_search: String::new(),
            save_status: None,
            settings_texture: None,
            settings_icon_angle: 0.0,
            sensor_reader: reader,
            last_update: Instant::now(),
        }
    }
}

impl LtmApp {
    fn load_settings_icon(&mut self, ctx: &egui::Context) {
        if self.settings_texture.is_some() {
            return;
        }

        let icon_path = std::path::Path::new("assets/icons/settings.png");
        if icon_path.exists() {
            if let Ok(bytes) = std::fs::read(icon_path) {
                if let Ok(img) = image::load_from_memory(&bytes) {
                    let rgba = img.to_rgba8();
                    let (w, h) = (rgba.width() as usize, rgba.height() as usize);
                    let color_image = ColorImage::from_rgba_unmultiplied([w, h], rgba.as_raw());
                    self.settings_texture = Some(ctx.load_texture(
                        "settings_icon",
                        color_image,
                        TextureOptions::LINEAR,
                    ));
                }
            }
        }
    }
}

impl eframe::App for LtmApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let refresh_ms = (self.config.refresh_interval * 1000.0) as u128;

        // Hardveres mintavételezés a megadott időközönként
        if now.duration_since(self.last_update).as_millis() >= refresh_ms {
            self.sensor_reader.update_temperatures(&mut self.sensors);
            self.last_update = now;
        }

        // Lerp animáció a sávok sima siklásához: a dt korlátozásával sosem ugrik
        let dt = ui.input(|i| i.stable_dt).clamp(0.001, 0.04);
        let mut animating = false;
        let lerp_speed = 5.0;

        for sensor in &mut self.sensors {
            let diff = sensor.temperature - sensor.displayed_temperature;
            if diff.abs() > 0.01 {
                let factor = 1.0 - (-lerp_speed * dt).exp();
                sensor.displayed_temperature += diff * factor;
                animating = true;
            } else {
                sensor.displayed_temperature = sensor.temperature;
            }
        }

        // Ha van mozgás, azonnal kérjük a következő 60 FPS képkockát, különben a beállított időközre alszik
        if animating {
            ui.ctx().request_repaint();
        } else {
            ui.ctx().request_repaint_after(std::time::Duration::from_millis(refresh_ms as u64));
        }

        self.load_settings_icon(ui.ctx());
        let t = i18n::get_translations(&self.config.language);

        let frame = egui::Frame::new()
            .fill(Theme::background_with_opacity(self.config.opacity))
            .inner_margin(Margin::same(18))
            .corner_radius(CornerRadius {
                nw: 0,
                ne: 0,
                sw: 8,
                se: 8,
            });

        egui::CentralPanel::default().frame(frame).show(ui, |ui| {
            // Fejléc
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("LTM")
                        .size(19.0)
                        .strong()
                        .color(Theme::TEXT),
                );

                ui.label(
                    RichText::new(&t.subtitle)
                        .size(9.0)
                        .color(Theme::MUTED),
                );

                // Beállítások gomb animációval
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let angle = if self.config.animate_gear {
                        self.settings_icon_angle
                    } else {
                        0.0
                    };

                    let (clicked, is_hovered) = if let Some(texture) = &self.settings_texture {
                        let img = egui::Image::new(texture)
                            .fit_to_exact_size(egui::vec2(16.0, 16.0))
                            .rotate(angle, egui::vec2(0.5, 0.5))
                            .tint(Theme::TEXT);
                        let response = ui.add(egui::Button::image(img).frame(false));
                        (response.clicked(), response.hovered())
                    } else {
                        let response = ui.add(
                            egui::Button::new(RichText::new("⚙").size(15.0).color(Theme::TEXT))
                                .frame(false),
                        );
                        (response.clicked(), response.hovered())
                    };

                    let rotation_speed = std::f32::consts::TAU / 2.5;

                    if self.config.animate_gear {
                        if is_hovered {
                            if self.settings_icon_angle < std::f32::consts::TAU {
                                self.settings_icon_angle =
                                    (self.settings_icon_angle + rotation_speed * dt).min(std::f32::consts::TAU);
                                ui.ctx().request_repaint();
                            }
                        } else if self.settings_icon_angle > 0.0 {
                            self.settings_icon_angle =
                                (self.settings_icon_angle - rotation_speed * dt).max(0.0);
                            ui.ctx().request_repaint();
                        }
                    } else {
                        self.settings_icon_angle = 0.0;
                    }

                    if clicked {
                        self.show_settings = !self.show_settings;
                        self.save_status = None;
                        self.language_search.clear();
                    }
                });
            });

            ui.add_space(14.0);

            // Fő nézet vagy Beállítások nézet
            if self.show_settings {
                draw_settings(
                    ui,
                    &mut self.config,
                    &t,
                    &mut self.language_search,
                    &mut self.save_status,
                    &mut self.show_settings,
                );
            } else {
                let mut counts = HashMap::new();
                for s in &self.sensors {
                    *counts.entry(s.id).or_insert(0usize) += 1;
                }

                let mut current_indices = HashMap::new();

                let available_h = ui.available_height() - 32.0;
                egui::ScrollArea::vertical()
                    .max_height(available_h)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for sensor in &self.sensors {
                            let total = counts.get(&sensor.id).copied().unwrap_or(1);
                            let idx = current_indices.entry(sensor.id).or_insert(0usize);
                            *idx += 1;

                            let base_name = t.sensor_name(sensor.id);
                            let label = if total > 1 {
                                format!("{base_name} {idx}")
                            } else {
                                base_name.to_string()
                            };

                            draw_sensor(
                                ui,
                                sensor.id,
                                &label,
                                &sensor.hardware_name,
                                sensor.temperature,
                                sensor.displayed_temperature,
                                self.config.max_scale,
                                self.config.temp_unit,
                            );
                        }
                    });
            }

            ui.add_space(2.0);
            ui.separator();
            ui.add_space(5.0);

            // Alsó információs sáv
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("{} {}", self.sensors.len(), t.footer_sensors))
                        .size(9.0)
                        .color(Theme::MUTED),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(&t.footer_mock)
                            .size(9.0)
                            .color(Theme::MUTED),
                    );
                });
            });
        });
    }
}