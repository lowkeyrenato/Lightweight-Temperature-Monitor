use eframe::egui::Color32;

pub struct Theme;

impl Theme {
    pub const BACKGROUND: Color32 = Color32::from_rgb(19, 23, 32);
    pub const PANEL: Color32 = Color32::from_rgb(28, 34, 46);
    pub const TEXT: Color32 = Color32::from_rgb(235, 239, 247);
    pub const MUTED: Color32 = Color32::from_rgb(135, 147, 165);
    pub const TRACK: Color32 = Color32::from_rgb(48, 55, 69);
    pub const ACCENT: Color32 = Color32::from_rgb(34, 197, 94);

    /// Háttérszín dinamikus átlátszósággal az opacity csúszka alapján
    pub fn background_with_opacity(opacity: f32) -> Color32 {
        let alpha = (opacity.clamp(0.2, 1.0) * 255.0) as u8;
        Color32::from_rgba_unmultiplied(
            Self::BACKGROUND.r(),
            Self::BACKGROUND.g(),
            Self::BACKGROUND.b(),
            alpha,
        )
    }

    /// Kiemelt panel háttér a beállításokhoz
    pub fn panel_with_opacity(opacity: f32) -> Color32 {
        let alpha = (opacity.clamp(0.2, 1.0) * 255.0) as u8;
        Color32::from_rgba_unmultiplied(
            Self::PANEL.r(),
            Self::PANEL.g(),
            Self::PANEL.b(),
            alpha,
        )
    }
}