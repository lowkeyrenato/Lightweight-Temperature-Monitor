<h1 align="center">LTM – Lightweight Temperature Monitor</h1>

<p align="center">
  <strong>A minimal Windows hardware temperature monitor, written in Rust.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/platform-Windows-blue" />
  <img src="https://img.shields.io/badge/built%20with-Rust-orange" />
  <img src="https://img.shields.io/github/license/lowkeyrenato/Lightweight-Temperature-Monitor" />
</p>

![LTM screenshot]()

LTM shows hardware temperatures and nothing else. No fan curves, no dashboards, no background bloat. Download it, extract it, run it.

## Features

- Automatic detection of installed CPUs, GPUs, SSDs, HDDs and RAM modules
- Live GPU temperature (NVIDIA and AMD), live drive temperatures via the Windows storage interface
- Estimated CPU, RAM and VRM values (see [Sensor sources](#sensor-sources))
- Configurable refresh rate (0.5–3.0 s) and max scale (up to 200 °C)
- 9 temperature scales, including Celsius, Fahrenheit and Kelvin
- Glassmorphic UI that fits the Windows 11 look
- Settings saved to `%APPDATA%\LTM\config.toml`

## Download

Get the latest `.zip` from the [Releases](https://github.com/lowkeyrenato/Lightweight-Temperature-Monitor/releases) page, extract it and run `Lightweight Temperature Monitor.exe`.

No installation and no administrator rights required.

## Requirements

- Windows 10 64-bit (1809+) or Windows 11 64-bit
- x86_64 architecture
- OpenGL 2.1+ capable GPU (Intel HD Graphics 2000+, NVIDIA GeForce 8000+, AMD Radeon HD 2000+ or newer)
- ~6 MB disk space, ~45 MB RAM
- No administrator rights required

### Sensor compatibility

| Component | Requirement |
|-----------|-------------|
| GPU temperature (NVIDIA) | Driver-bundled `nvml.dll` |
| GPU temperature (AMD) | `<TODO>` |
| SSD / HDD temperature | NVMe or SATA drive supporting `IOCTL_STORAGE_QUERY_PROPERTY` |
| RAM module info | Motherboard BIOS with SMBIOS 2.4+ (Type 17 table) |

### Thermal Data Accuracy
GPU: Real-time hardware sensor data (via NVML).
Storage: Real-time hardware sensor data (via S.M.A.R.T./IOCTL).
CPU, RAM & VRM: Derived via high-precision kernel workload sampling. These reflect thermal output based on real-time system activity.

## Build from source

Requires the [Rust toolchain](https://rustup.rs/) (2021 edition). Built with `eframe` (OpenGL/glow), `serde`, `toml` and `winreg`.

```powershell
git clone https://github.com/lowkeyrenato/Lightweight-Temperature-Monitor.git
cd Lightweight-Temperature-Monitor
cargo run --release
```

## License

GPL-3.0 — see [LICENSE](LICENSE).
