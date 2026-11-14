use serde::Deserialize;
use crate::sensors::SensorId;

#[derive(Debug, Clone, Deserialize)]
pub struct Translations {
    pub language_name: String,
    pub subtitle: String,
    pub footer_sensors: String,
    pub footer_mock: String,
    pub language_label: String,
    pub search_hint: String,
    pub opacity_label: String,
    pub refresh_label: String,
    pub max_scale_label: String,
    pub temp_unit_label: String,
    pub always_on_top: String,
    pub gear_animation: String,
    pub save_button: String,
    pub back_button: String,
    pub saved_success: String,
    pub sensor_cpu: String,
    pub sensor_gpu: String,
    pub sensor_ssd: String,
    pub sensor_hdd: String,
    pub sensor_ram: String,
    pub sensor_mobo_vrm: String,
}

impl Translations {
    pub fn sensor_name(&self, id: SensorId) -> &str {
        match id {
            SensorId::Cpu => &self.sensor_cpu,
            SensorId::Gpu => &self.sensor_gpu,
            SensorId::Ssd => &self.sensor_ssd,
            SensorId::Hdd => &self.sensor_hdd,
            SensorId::Ram => &self.sensor_ram,
            SensorId::MotherboardVrm => &self.sensor_mobo_vrm,
        }
    }
}

const EN_US_STR: &str = include_str!("../locales/en-US.toml");
const HU_HU_STR: &str = include_str!("../locales/hu-HU.toml");
const ES_ES_STR: &str = include_str!("../locales/es-ES.toml");
const RU_RU_STR: &str = include_str!("../locales/ru-RU.toml");

pub fn available_languages() -> &'static [(&'static str, &'static str)] {
    &[
        ("hu-HU", "Magyar"),
        ("en-US", "English"),
        ("es-ES", "Español"),
        ("ru-RU", "Русский"),
    ]
}

pub fn get_translations(lang: &str) -> Translations {
    let toml_str = match lang {
        "hu-HU" => HU_HU_STR,
        "es-ES" => ES_ES_STR,
        "ru-RU" => RU_RU_STR,
        _ => EN_US_STR,
    };

    toml::from_str(toml_str).unwrap_or_else(|_| {
        toml::from_str(EN_US_STR).expect("Fallback en-US translation file is invalid")
    })
}