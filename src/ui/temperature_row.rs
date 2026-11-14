use eframe::egui;
use crate::config::TemperatureUnit;
use crate::sensors::SensorId;
use crate::theme::Theme;
use super::progress_bar::draw_temperature_bar;

pub fn draw_sensor(
    ui: &mut egui::Ui,
    sensor_id: SensorId,
    name: &str,
    hardware_name: &str,
    temperature: f32,
    displayed_temp: f32,
    user_max_scale: f32,
    temp_unit: TemperatureUnit,
) {
    ui.horizontal(|ui| {
        ui.add_space(2.0);

        // Hőmérséklet a felhasználó által választott mértékegységben és szimbólummal
        let converted_val = temp_unit.from_celsius(temperature);
        let temp_text = format!("{:.2}{}", converted_val, temp_unit.symbol());
        let temp_label = egui::RichText::new(temp_text).size(14.0).strong().color(Theme::TEXT);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(temp_label);

            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(format!("{name}:"))
                        .size(13.0)
                        .strong()
                        .color(Theme::TEXT),
                );

                if !hardware_name.is_empty() && hardware_name != "-" {
                    ui.add_space(4.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(hardware_name)
                                .size(11.0)
                                .color(Theme::MUTED),
                        )
                        .truncate(),
                    )
                    .on_hover_text(hardware_name);
                }
            });
        });
    });

    ui.add_space(6.0);
    draw_temperature_bar(ui, sensor_id, displayed_temp, user_max_scale);
    ui.add_space(14.0);
}