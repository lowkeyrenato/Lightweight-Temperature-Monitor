pub mod hardware;
pub mod mock;
pub mod real;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensorId {
    Cpu,
    Gpu,
    Ssd,
    Hdd,
    Ram,
    MotherboardVrm,
}

impl SensorId {
    pub fn thresholds(&self, user_max_scale: f32) -> (f32, f32, f32) {
        let (base_warn, base_crit, default_max) = match self {
            SensorId::Cpu => (70.0, 85.0, 100.0),
            SensorId::Gpu => (68.0, 80.0, 95.0),
            SensorId::Ssd => (50.0, 65.0, 75.0),
            SensorId::Hdd => (42.0, 50.0, 60.0),
            SensorId::Ram => (50.0, 65.0, 75.0),
            SensorId::MotherboardVrm => (80.0, 105.0, 100.0),
        };

        // A maximális skála arányosítása a felhasználó által választott határhoz (akár 200°C-ig)
        let scale_factor = user_max_scale / 100.0;
        let effective_max = (default_max * scale_factor).max(base_crit + 5.0);
        (base_warn, base_crit, effective_max)
    }
}

pub struct Sensor {
    pub id: SensorId,
    pub hardware_name: String,
    pub temperature: f32,
    pub displayed_temperature: f32,
}