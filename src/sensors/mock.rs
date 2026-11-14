use super::hardware::DetectedHardware;
use super::{Sensor, SensorId};

pub fn get_mock_sensors() -> Vec<Sensor> {
    let hw = DetectedHardware::detect();
    let mut list = Vec::new();

    // CPU-k
    for (i, cpu_name) in hw.cpus.into_iter().enumerate() {
        let temp = 44.50 + (i as f32 * 2.25);
        list.push(Sensor {
            id: SensorId::Cpu,
            hardware_name: cpu_name,
            temperature: temp,
            displayed_temperature: temp,
        });
    }

    // GPU-k
    for (i, gpu_name) in hw.gpus.into_iter().enumerate() {
        let temp = 50.15 + (i as f32 * 3.10);
        list.push(Sensor {
            id: SensorId::Gpu,
            hardware_name: gpu_name,
            temperature: temp,
            displayed_temperature: temp,
        });
    }

    // SSD-k
    for (i, ssd_name) in hw.ssds.into_iter().enumerate() {
        let temp = 38.45 + ((i as f32 * 2.80) % 6.0);
        list.push(Sensor {
            id: SensorId::Ssd,
            hardware_name: ssd_name,
            temperature: temp,
            displayed_temperature: temp,
        });
    }

    // HDD-k
    for (i, hdd_name) in hw.hdds.into_iter().enumerate() {
        let temp = 31.20 + (i as f32 * 1.75);
        list.push(Sensor {
            id: SensorId::Hdd,
            hardware_name: hdd_name,
            temperature: temp,
            displayed_temperature: temp,
        });
    }

    // RAM modulok
    for (i, ram_name) in hw.rams.into_iter().enumerate() {
        let temp = 39.80 + (i as f32 * 1.60);
        list.push(Sensor {
            id: SensorId::Ram,
            hardware_name: ram_name,
            temperature: temp,
            displayed_temperature: temp,
        });
    }

    // Alaplap VRM
    let vrm_temp = 52.40;
    list.push(Sensor {
        id: SensorId::MotherboardVrm,
        hardware_name: hw.motherboard,
        temperature: vrm_temp,
        displayed_temperature: vrm_temp,
    });

    list
}