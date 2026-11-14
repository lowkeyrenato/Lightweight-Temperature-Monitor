use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
    Reaumur,
    Romer,
    Delisle,
    Newton,
    Leiden,
}

impl TemperatureUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
            TemperatureUnit::Kelvin => "K",
            TemperatureUnit::Rankine => "°R",
            TemperatureUnit::Reaumur => "°Ré",
            TemperatureUnit::Romer => "°Rø",
            TemperatureUnit::Delisle => "°De",
            TemperatureUnit::Newton => "°N",
            TemperatureUnit::Leiden => "°L",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TemperatureUnit::Celsius => "Celsius (°C)",
            TemperatureUnit::Fahrenheit => "Fahrenheit (°F)",
            TemperatureUnit::Kelvin => "Kelvin (K)",
            TemperatureUnit::Rankine => "Rankine (°R)",
            TemperatureUnit::Reaumur => "Réaumur (°Ré)",
            TemperatureUnit::Romer => "Rømer (°Rø)",
            TemperatureUnit::Delisle => "Delisle (°De)",
            TemperatureUnit::Newton => "Newton (°N)",
            TemperatureUnit::Leiden => "Leiden-skála (°L)",
        }
    }

    pub fn from_celsius(&self, c: f32) -> f32 {
        match self {
            TemperatureUnit::Celsius => c,
            TemperatureUnit::Fahrenheit => c * 1.8 + 32.0,
            TemperatureUnit::Kelvin => c + 273.15,
            TemperatureUnit::Rankine => (c + 273.15) * 1.8,
            TemperatureUnit::Reaumur => c * 0.8,
            TemperatureUnit::Romer => c * 0.525 + 7.5,
            TemperatureUnit::Delisle => (100.0 - c) * 1.5,
            TemperatureUnit::Newton => c * 0.33,
            TemperatureUnit::Leiden => c + 273.15,
        }
    }

    pub fn all() -> &'static [TemperatureUnit] {
        &[
            TemperatureUnit::Celsius,
            TemperatureUnit::Fahrenheit,
            TemperatureUnit::Kelvin,
            TemperatureUnit::Rankine,
            TemperatureUnit::Reaumur,
            TemperatureUnit::Romer,
            TemperatureUnit::Delisle,
            TemperatureUnit::Newton,
            TemperatureUnit::Leiden,
        ]
    }
}

fn default_true() -> bool {
    true
}

fn default_refresh_interval() -> f32 {
    1.5
}

fn default_max_scale() -> f32 {
    100.0
}

fn default_unit() -> TemperatureUnit {
    TemperatureUnit::Celsius
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub language: String,
    pub opacity: f32,
    pub always_on_top: bool,
    #[serde(default = "default_true")]
    pub animate_gear: bool,
    #[serde(default = "default_refresh_interval")]
    pub refresh_interval: f32,
    #[serde(default = "default_max_scale")]
    pub max_scale: f32,
    #[serde(default = "default_unit")]
    pub temp_unit: TemperatureUnit,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            language: "hu-HU".to_string(),
            opacity: 0.95,
            always_on_top: false,
            animate_gear: true,
            refresh_interval: 1.5,
            max_scale: 100.0,
            temp_unit: TemperatureUnit::Celsius,
        }
    }
}

impl AppConfig {
    pub fn file_path() -> PathBuf {
        if let Ok(appdata) = std::env::var("APPDATA") {
            let mut path = PathBuf::from(appdata);
            path.push("LTM");
            path.push("config.toml");
            path
        } else {
            PathBuf::from("config.toml")
        }
    }

    pub fn load() -> Self {
        let path = Self::file_path();
        if path.exists() {
            if let Ok(contents) = fs::read_to_string(&path) {
                if let Ok(config) = toml::from_str(&contents) {
                    return config;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::file_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let serialized = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(path, serialized)?;
        Ok(())
    }
}