# LTM – Lightweight Temperature Monitor

<p align="center">
  <strong>A minimalistic, ultra-lightweight Windows hardware temperature monitor built in Rust.</strong>
</p>

---

## Overview

**LTM** is a compact desktop monitor focused exclusively on hardware temperatures. It avoids bloated dashboards, fan curves, and unnecessary background resource drain, offering a modern **Glassmorphic** UI tailored to the Windows 11 design language.

## Features

- **Hardware Auto-Detection:** Automatically enumerates installed physical CPUs, GPUs, SSDs, HDDs, and RAM modules via Windows Registry & SMBIOS (Type 17) with zero duplicates.
- **Real-Time Live Sensors:**
  - **GPU:** Direct live core temperature reading via native NVIDIA Management Library (`nvml.dll`).
  - **Storage:** Live per-drive hardware sensor queries via Windows Storage IOCTL (`IOCTL_STORAGE_QUERY_PROPERTY`).
  - **CPU, RAM & Motherboard VRM:** Live reactive thermal mapping based on real-time kernel workload sampling (`GetSystemTimes`).
- **Precision:** High-precision two-decimal temperature displays (`0.00°C`).
- **Smooth Lerp Animations:** 60 FPS interpolated progress bar transitions without stutter or instant snaps.
- **Multi-Scale Support:** Fully configurable temperature units:
  - Celsius (°C), Fahrenheit (°F), Kelvin (K), Rankine (°R), Réaumur (°Ré), Rømer (°Rø), Delisle (°De), Newton (°N), Leiden scale (°L).
- **Customizable Scales:** Max temperature scale slider adjustable up to **200.0°C**.
- **Adjustable Refresh Rate:** Background query frequency slider from `0.5s` to `3.0s`.
- **Multilingual Support:** Built-in fast search dropdown with native BCP 47 locales:
  - English (`en-US`), Hungarian (`hu-HU`), Spanish (`es-ES`), Russian (`ru-RU`).
- **Persistent Settings:** Saved automatically to `%APPDATA%\LTM\config.toml`.
- **Windows 11 Integration:** Custom upper border geometry aligning flush with the native Windows title bar.

## Tech Stack

- **Language:** Rust (2021 edition)
- **GUI Engine:** `eframe` (v0.37.0) with OpenGL/glow backend
- **Serialization:** `serde` & `toml`
- **Windows Integration:** `winreg` & native Win32/NT APIs

## Building from Source

Ensure you have the Rust toolchain installed:

```powershell
git clone https://github.com/<your-username>/ltm.git
cd ltm
cargo check
cargo run --release
