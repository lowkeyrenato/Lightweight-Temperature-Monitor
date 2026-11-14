use eframe::egui::{
    self, viewport::WindowLevel, CornerRadius, Margin, PopupCloseBehavior, RichText, ViewportCommand,
};
use crate::config::{AppConfig, TemperatureUnit};
use crate::i18n::{self, Translations};
use crate::theme::Theme;

pub fn draw_settings(
    ui: &mut egui::Ui,
    config: &mut AppConfig,
    t: &Translations,
    search_query: &mut String,
    save_status: &mut Option<String>,
    show_settings: &mut bool,
) {
    egui::Frame::new()
        .fill(Theme::panel_with_opacity(config.opacity))
        .inner_margin(Margin::same(12))
        .corner_radius(CornerRadius::same(10))
        .show(ui, |ui| {
            // 1. Nyelv választó
            ui.label(RichText::new(&t.language_label).size(12.0).color(Theme::MUTED));
            ui.add_space(4.0);

            let popup_focus_id = ui.scope_id().with("lang_search_focus_state");
            let was_open: bool = ui.data_mut(|d| d.get_temp(popup_focus_id).unwrap_or(false));

            let combo_response = egui::ComboBox::from_id_salt("language_dropdown")
                .width(ui.available_width() - 8.0)
                .selected_text(RichText::new(&t.language_name).color(Theme::TEXT))
                .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                .show_ui(ui, |ui| {
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        ui.close();
                    }

                    let text_edit_response = ui.add(
                        egui::TextEdit::singleline(search_query)
                            .hint_text(RichText::new(&t.search_hint).color(Theme::MUTED))
                            .desired_width(ui.available_width()),
                    );

                    if !was_open {
                        search_query.clear();
                        text_edit_response.request_focus();
                    }

                    ui.data_mut(|d| d.insert_temp(popup_focus_id, true));

                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(2.0);

                    let query = search_query.to_lowercase();
                    let mut found = 0;

                    for (code, name) in i18n::available_languages() {
                        if query.is_empty()
                            || name.to_lowercase().contains(&query)
                            || code.to_lowercase().contains(&query)
                        {
                            found += 1;
                            let is_selected = config.language == *code;
                            if ui.selectable_label(is_selected, *name).clicked() {
                                config.language = code.to_string();
                                search_query.clear();
                                ui.close();
                            }
                        }
                    }

                    if found == 0 {
                        ui.label(RichText::new("-").color(Theme::MUTED));
                    }
                });

            if combo_response.inner.is_none() {
                ui.data_mut(|d| d.insert_temp(popup_focus_id, false));
            }

            ui.add_space(10.0);

            // 2. Mértékegység választó (mind a 9 skála)
            ui.label(RichText::new(&t.temp_unit_label).size(12.0).color(Theme::MUTED));
            ui.add_space(4.0);

            egui::ComboBox::from_id_salt("unit_dropdown")
                .width(ui.available_width() - 8.0)
                .selected_text(RichText::new(config.temp_unit.display_name()).color(Theme::TEXT))
                .show_ui(ui, |ui| {
                    for unit in TemperatureUnit::all() {
                        let is_selected = config.temp_unit == *unit;
                        if ui.selectable_label(is_selected, unit.display_name()).clicked() {
                            config.temp_unit = *unit;
                        }
                    }
                });

            ui.add_space(10.0);

            // 3. Maximális skála csúszka (70.0 ..= 200.0 °C tartományban)
            let scale_conv = config.temp_unit.from_celsius(config.max_scale);
            let max_scale_label = format!("{}: {:.1}{}", t.max_scale_label, scale_conv, config.temp_unit.symbol());
            ui.label(RichText::new(max_scale_label).size(12.0).color(Theme::MUTED));
            ui.add_space(4.0);
            ui.add(egui::Slider::new(&mut config.max_scale, 70.0..=200.0).step_by(5.0).show_value(false));

            ui.add_space(10.0);

            // 4. Frissítési időköz csúszka
            let refresh_label = format!("{}: {:.1}s", t.refresh_label, config.refresh_interval);
            ui.label(RichText::new(refresh_label).size(12.0).color(Theme::MUTED));
            ui.add_space(4.0);
            ui.add(
                egui::Slider::new(&mut config.refresh_interval, 0.5..=3.0)
                    .step_by(0.1)
                    .show_value(false),
            );

            ui.add_space(10.0);

            // 5. Átlátszóság csúszka
            let opacity_label = format!("{}: {:.0}%", t.opacity_label, config.opacity * 100.0);
            ui.label(RichText::new(opacity_label).size(12.0).color(Theme::MUTED));
            ui.add_space(4.0);
            ui.add(egui::Slider::new(&mut config.opacity, 0.35..=1.0).show_value(false));

            ui.add_space(10.0);

            // 6. Kapcsolók
            if ui.checkbox(&mut config.always_on_top, RichText::new(&t.always_on_top).size(12.0).color(Theme::TEXT)).changed() {
                let level = if config.always_on_top {
                    WindowLevel::AlwaysOnTop
                } else {
                    WindowLevel::Normal
                };
                ui.ctx().send_viewport_cmd(ViewportCommand::WindowLevel(level));
            }

            ui.add_space(4.0);

            ui.checkbox(
                &mut config.animate_gear,
                RichText::new(&t.gear_animation).size(12.0).color(Theme::TEXT),
            );

            ui.add_space(14.0);

            // Mentés és Vissza gombok
            ui.horizontal(|ui| {
                if ui.button(RichText::new(&t.save_button).size(12.0).strong()).clicked() {
                    match config.save() {
                        Ok(_) => {
                            *save_status = Some(t.saved_success.clone());
                        }
                        Err(e) => {
                            *save_status = Some(format!("Hiba: {e}"));
                        }
                    }
                }

                if ui.button(RichText::new(&t.back_button).size(12.0)).clicked() {
                    *show_settings = false;
                    *save_status = None;
                }

                if let Some(msg) = save_status {
                    ui.label(RichText::new(msg.as_str()).size(11.0).color(Theme::ACCENT));
                }
            });
        });
}