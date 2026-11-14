use eframe::egui;
use crate::sensors::SensorId;
use crate::theme::Theme;

pub fn draw_temperature_bar(ui: &mut egui::Ui, sensor_id: SensorId, value: f32, user_max_scale: f32) {
    let desired_size = egui::vec2(ui.available_width(), 9.0);
    let (rect, _) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

    let painter = ui.painter();

    // Háttér sáv
    painter.rect_filled(rect, 5.0, Theme::TRACK);

    let (warn_temp, crit_temp, max_scale) = sensor_id.thresholds(user_max_scale);

    let progress = (value / max_scale).clamp(0.0, 1.0);
    let segments = 60;

    let warn_ratio = (warn_temp / max_scale).clamp(0.0, 1.0);
    let crit_ratio = (crit_temp / max_scale).clamp(0.0, 1.0);

    for i in 0..segments {
        let start = i as f32 / segments as f32;
        let end = (i + 1) as f32 / segments as f32;

        if start >= progress {
            break;
        }

        let color = if start < warn_ratio {
            let t = (start / warn_ratio).clamp(0.0, 1.0);
            egui::Color32::from_rgb(
                (34.0 + 200.0 * t) as u8,
                197,
                (94.0 - 75.0 * t) as u8,
            )
        } else if start < crit_ratio {
            let t = ((start - warn_ratio) / (crit_ratio - warn_ratio)).clamp(0.0, 1.0);
            egui::Color32::from_rgb(
                234,
                (197.0 - 129.0 * t) as u8,
                (19.0 - 10.0 * t) as u8,
            )
        } else {
            egui::Color32::from_rgb(234, 40, 20)
        };

        let x1 = rect.left() + rect.width() * start;
        let x2 = rect.left() + rect.width() * end;

        let segment = egui::Rect::from_min_max(
            egui::pos2(x1, rect.top()),
            egui::pos2(x2 + 0.5, rect.bottom()),
        );

        painter.rect_filled(segment, 0.0, color);
    }
}