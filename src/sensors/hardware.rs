use winreg::enums::*;
use winreg::RegKey;

#[link(name = "kernel32")]
extern "system" {
    fn GetSystemFirmwareTable(
        firmware_table_provider_signature: u32,
        firmware_table_id: u32,
        p_firmware_table_buffer: *mut u8,
        buffer_size: u32,
    ) -> u32;

    fn GlobalMemoryStatusEx(lp_buffer: *mut MemoryStatusEx) -> i32;
}

#[repr(C)]
struct MemoryStatusEx {
    dw_length: u32,
    dw_memory_load: u32,
    ull_total_phys: u64,
    ull_avail_phys: u64,
    ull_total_page_file: u64,
    ull_avail_page_file: u64,
    ull_total_virtual: u64,
    ull_avail_virtual: u64,
    ull_avail_extended_virtual: u64,
}

#[derive(Debug, Clone)]
pub struct DetectedHardware {
    pub cpus: Vec<String>,
    pub gpus: Vec<String>,
    pub ssds: Vec<String>,
    pub hdds: Vec<String>,
    pub rams: Vec<String>,
    pub motherboard: String,
}

impl DetectedHardware {
    pub fn detect() -> Self {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

        let (smbios_cpus, smbios_rams) = detect_smbios_data();

        let cpus = if !smbios_cpus.is_empty() {
            smbios_cpus
        } else {
            let cpu = detect_cpu_registry(&hklm).unwrap_or_else(|| "CPU".to_string());
            vec![cpu]
        };

        let gpus = detect_gpus(&hklm);
        let (ssds, hdds) = detect_disks(&hklm);

        let rams = if !smbios_rams.is_empty() {
            smbios_rams
        } else {
            detect_ram_fallback()
        };

        let motherboard = detect_motherboard(&hklm).unwrap_or_else(|| "Motherboard".to_string());

        Self {
            cpus,
            gpus,
            ssds,
            hdds,
            rams,
            motherboard,
        }
    }
}

fn detect_smbios_data() -> (Vec<String>, Vec<String>) {
    let mut cpus = Vec::new();
    let mut rams = Vec::new();

    let size = unsafe { GetSystemFirmwareTable(0x52534D42, 0, std::ptr::null_mut(), 0) };
    if size == 0 {
        return (cpus, rams);
    }

    let mut buffer = vec![0u8; size as usize];
    let written = unsafe { GetSystemFirmwareTable(0x52534D42, 0, buffer.as_mut_ptr(), size) };
    if written == 0 || buffer.len() < 8 {
        return (cpus, rams);
    }

    let data = &buffer[8..];
    let mut offset = 0;

    while offset + 4 <= data.len() {
        let table_type = data[offset];
        let formatted_len = data[offset + 1] as usize;
        if formatted_len < 4 || offset + formatted_len > data.len() {
            break;
        }

        let struct_data = &data[offset..offset + formatted_len];
        let mut str_offset = offset + formatted_len;
        let mut strings = Vec::new();

        while str_offset < data.len() {
            if data[str_offset] == 0 {
                str_offset += 1;
                break;
            }
            let start = str_offset;
            while str_offset < data.len() && data[str_offset] != 0 {
                str_offset += 1;
            }
            if let Ok(s) = std::str::from_utf8(&data[start..str_offset]) {
                strings.push(s.trim().to_string());
            }
            if str_offset < data.len() && data[str_offset] == 0 {
                str_offset += 1;
            }
        }

        if str_offset < data.len() && data[str_offset] == 0 {
            str_offset += 1;
        }
        offset = str_offset;

        // Type 4: Processor Information
        if table_type == 4 && struct_data.len() > 0x18 {
            let status = struct_data[0x18];
            if (status & 0x40) != 0 {
                let name_idx = struct_data[0x10];
                if let Some(name) = get_smbios_string(&strings, name_idx) {
                    let cleaned = clean_cpu_string(name);
                    if !cleaned.is_empty() && !cpus.contains(&cleaned) {
                        cpus.push(cleaned);
                    }
                }
            }
        }

        // Type 17: Memory Device
        if table_type == 17 && struct_data.len() >= 0x14 {
            let size_raw = u16::from_le_bytes([struct_data[0x0C], struct_data[0x0D]]);
            if size_raw != 0 && size_raw != 0xFFFF {
                let size_mb = if size_raw == 0x7FFF && struct_data.len() >= 0x20 {
                    u32::from_le_bytes([
                        struct_data[0x1C],
                        struct_data[0x1D],
                        struct_data[0x1E],
                        struct_data[0x1F],
                    ])
                } else if (size_raw & 0x8000) == 0 {
                    size_raw as u32
                } else {
                    (size_raw & 0x7FFF) as u32 / 1024
                };

                if size_mb > 0 {
                    let size_gb = (size_mb + 512) / 1024;
                    let speed = if struct_data.len() >= 0x19 {
                        u16::from_le_bytes([struct_data[0x17], struct_data[0x18]])
                    } else {
                        0
                    };

                    let mem_type_str = if struct_data.len() > 0x12 {
                        match struct_data[0x12] {
                            0x1A => "DDR4",
                            0x1E => "DDR5",
                            0x18 => "DDR3",
                            _ => "",
                        }
                    } else {
                        ""
                    };

                    let mfr = if struct_data.len() > 0x17 {
                        get_smbios_string(&strings, struct_data[0x17]).unwrap_or("")
                    } else {
                        ""
                    };

                    let clean_mfr = clean_ram_mfr(mfr);
                    let mut desc = String::new();
                    if !clean_mfr.is_empty() {
                        desc.push_str(&clean_mfr);
                        desc.push(' ');
                    }
                    desc.push_str(&format!("{size_gb}GB"));
                    if !mem_type_str.is_empty() {
                        desc.push_str(&format!(" {mem_type_str}"));
                    }
                    if speed > 0 {
                        desc.push_str(&format!(" {speed}MHz"));
                    }

                    rams.push(desc.trim().to_string());
                }
            }
        }
    }

    (cpus, rams)
}

fn get_smbios_string(strings: &[String], idx: u8) -> Option<&str> {
    if idx == 0 {
        None
    } else {
        strings.get((idx - 1) as usize).map(|s| s.as_str())
    }
}

fn clean_ram_mfr(raw: &str) -> String {
    let lower = raw.to_lowercase();
    if lower.contains("corsair") {
        "Corsair".to_string()
    } else if lower.contains("kingston") {
        "Kingston".to_string()
    } else if lower.contains("g.skill") || lower.contains("gskill") {
        "G.Skill".to_string()
    } else if lower.contains("crucial") || lower.contains("micron") {
        "Crucial".to_string()
    } else if lower.contains("samsung") {
        "Samsung".to_string()
    } else if lower.contains("hynix") {
        "SK Hynix".to_string()
    } else if lower.contains("teamgroup") || lower.contains("team") {
        "TeamGroup".to_string()
    } else if raw.len() > 2 && !raw.starts_with("0x") {
        raw.trim().to_string()
    } else {
        String::new()
    }
}

fn detect_ram_fallback() -> Vec<String> {
    let mut mem = MemoryStatusEx {
        dw_length: std::mem::size_of::<MemoryStatusEx>() as u32,
        dw_memory_load: 0,
        ull_total_phys: 0,
        ull_avail_phys: 0,
        ull_total_page_file: 0,
        ull_avail_page_file: 0,
        ull_total_virtual: 0,
        ull_avail_virtual: 0,
        ull_avail_extended_virtual: 0,
    };

    if unsafe { GlobalMemoryStatusEx(&mut mem) } != 0 {
        let gb = (mem.ull_total_phys + (1024 * 1024 * 512)) / (1024 * 1024 * 1024);
        vec![format!("{gb}GB System RAM")]
    } else {
        vec!["16GB RAM".to_string()]
    }
}

fn detect_cpu_registry(hklm: &RegKey) -> Option<String> {
    let key = hklm
        .open_subkey("HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0")
        .ok()?;
    let raw: String = key.get_value("ProcessorNameString").ok()?;
    Some(clean_cpu_string(&raw))
}

fn clean_cpu_string(raw: &str) -> String {
    let s = raw
        .replace("(R)", "")
        .replace("(TM)", "")
        .replace("Processor", "")
        .replace("8-Core", "")
        .replace("12-Core", "")
        .replace("16-Core", "")
        .replace("24-Core", "")
        .replace("32-Core", "");

    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn detect_gpus(hklm: &RegKey) -> Vec<String> {
    let mut gpus = Vec::new();
    let class_key = match hklm
        .open_subkey("SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}")
    {
        Ok(k) => k,
        Err(_) => return vec!["GPU".to_string()],
    };

    for subkey_name in class_key.enum_keys().map_while(Result::ok) {
        if let Ok(dev_key) = class_key.open_subkey(&subkey_name) {
            if let Ok(desc) = dev_key.get_value::<String, _>("DriverDesc") {
                let lower = desc.to_lowercase();
                if lower.contains("basic display")
                    || lower.contains("remote")
                    || lower.contains("virtual")
                {
                    continue;
                }
                let clean = clean_gpu_string(&desc);
                if !clean.is_empty() && !is_duplicate(&clean, &gpus) {
                    gpus.push(clean);
                }
            }
        }
    }

    if gpus.is_empty() {
        vec!["GPU".to_string()]
    } else {
        gpus
    }
}

fn clean_gpu_string(raw: &str) -> String {
    let s = raw.replace("(R)", "").replace("(TM)", "");
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn detect_motherboard(hklm: &RegKey) -> Option<String> {
    let bios_key = hklm.open_subkey("HARDWARE\\DESCRIPTION\\System\\BIOS").ok()?;
    let product: String = bios_key.get_value("BaseBoardProduct").ok()?;
    let mfr: String = bios_key.get_value("BaseBoardManufacturer").unwrap_or_default();

    let clean_mfr = if mfr.contains("ASUSTeK") {
        "ASUS"
    } else if mfr.contains("Micro-Star") || mfr.contains("MSI") {
        "MSI"
    } else if mfr.contains("Gigabyte") {
        "Gigabyte"
    } else if mfr.contains("ASRock") {
        "ASRock"
    } else {
        ""
    };

    let trimmed_product = product.trim();
    if !clean_mfr.is_empty() && !trimmed_product.to_lowercase().contains(&clean_mfr.to_lowercase()) {
        Some(format!("{clean_mfr} {trimmed_product}"))
    } else {
        Some(trimmed_product.to_string())
    }
}

pub fn detect_disks(hklm: &RegKey) -> (Vec<String>, Vec<String>) {
    let mut ssds = Vec::new();
    let mut hdds = Vec::new();

    // 1. Elsődleges és megbízható: HARDWARE\DEVICEMAP\Scsi
    if let Ok(scsi_key) = hklm.open_subkey("HARDWARE\\DEVICEMAP\\Scsi") {
        for port in scsi_key.enum_keys().map_while(Result::ok) {
            if let Ok(port_key) = scsi_key.open_subkey(&port) {
                let port_driver: String = port_key.get_value("Driver").unwrap_or_default();
                let is_nvme_port = port.to_lowercase().contains("nvme")
                    || port_driver.to_lowercase().contains("nvme")
                    || port_driver.to_lowercase().contains("stornvme");

                for bus in port_key.enum_keys().map_while(Result::ok) {
                    if let Ok(bus_key) = port_key.open_subkey(&bus) {
                        for target in bus_key.enum_keys().map_while(Result::ok) {
                            if let Ok(target_key) = bus_key.open_subkey(&target) {
                                for lu in target_key.enum_keys().map_while(Result::ok) {
                                    if let Ok(lu_key) = target_key.open_subkey(&lu) {
                                        let dev_type: String =
                                            lu_key.get_value("DeviceType").unwrap_or_default();
                                        if dev_type.eq_ignore_ascii_case("DiskPeripheral") {
                                            if let Ok(ident) =
                                                lu_key.get_value::<String, _>("Identifier")
                                            {
                                                classify_and_add(
                                                    &ident,
                                                    is_nvme_port,
                                                    &mut ssds,
                                                    &mut hdds,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 2. Csak akkor nyúlunk a disk\Enum-hoz, ha az elsődleges lista teljesen üres maradt
    if ssds.is_empty() && hdds.is_empty() {
        if let Ok(disk_enum) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Services\\disk\\Enum") {
            if let Ok(count) = disk_enum.get_value::<u32, _>("Count") {
                for i in 0..count {
                    if let Ok(entry) = disk_enum.get_value::<String, _>(&i.to_string()) {
                        classify_enum_entry(&entry, &mut ssds, &mut hdds);
                    }
                }
            }
        }
    }

    // Végső fallback
    if ssds.is_empty() && hdds.is_empty() {
        ssds.push("SSD".to_string());
    }

    (ssds, hdds)
}

fn classify_and_add(ident: &str, is_nvme: bool, ssds: &mut Vec<String>, hdds: &mut Vec<String>) {
    let clean = clean_disk_name(ident);
    if clean.is_empty() {
        return;
    }

    if is_duplicate(&clean, ssds) || is_duplicate(&clean, hdds) {
        return;
    }

    let lower = clean.to_lowercase();
    let is_ssd = is_nvme
        || lower.contains("ssd")
        || lower.contains("nvme")
        || lower.contains("optane")
        || lower.contains("sn850")
        || lower.contains("sn770")
        || lower.contains("sn570")
        || lower.contains("980 pro")
        || lower.contains("990 pro")
        || lower.contains("970 evo")
        || lower.contains("crucial")
        || lower.contains("kingston");

    let is_hdd = !is_ssd
        && (lower.contains("wd")
            || lower.contains("barracuda")
            || lower.contains("ironwolf")
            || lower.contains("toshiba")
            || lower.contains("hdd")
            || lower.contains("st1000")
            || lower.contains("st2000")
            || lower.contains("st4000"));

    if is_ssd {
        ssds.push(clean);
    } else if is_hdd {
        hdds.push(clean);
    } else {
        ssds.push(clean);
    }
}

fn classify_enum_entry(entry: &str, ssds: &mut Vec<String>, hdds: &mut Vec<String>) {
    let lower = entry.to_lowercase();
    let parts: Vec<&str> = entry.split('\\').collect();
    if parts.len() >= 2 {
        let dev_id = parts[1];
        let clean = dev_id
            .replace("Disk&Ven_", "")
            .replace("Prod_", "")
            .replace("Disk", "")
            .replace('&', " ")
            .replace('_', " ");
        let cleaned = clean_disk_name(&clean);
        if !cleaned.is_empty() {
            let is_nvme = lower.contains("nvme");
            classify_and_add(&cleaned, is_nvme, ssds, hdds);
        }
    }
}

fn clean_disk_name(raw: &str) -> String {
    let s = raw.trim();
    let without_nvme = if let Some(stripped) = s.strip_prefix("NVMe") {
        stripped.trim()
    } else {
        s
    };

    let without_firmware = if let Some(idx) = without_nvme.find("___") {
        &without_nvme[..idx]
    } else {
        without_nvme
    };

    without_firmware.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_duplicate(name: &str, list: &[String]) -> bool {
    let n = name.to_lowercase();
    list.iter().any(|item| {
        let it = item.to_lowercase();
        it == n || it.starts_with(&n) || n.starts_with(&it)
    })
}