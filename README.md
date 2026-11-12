# Open Lake Controller v5.8 (Modern Windowed Edition)

[![Release](https://img.shields.io/github/v/release/gabrielpgava/open-lake-controller-5.8?include_prereleases&style=flat-square)](https://github.com/gabrielpgava/open-lake-controller-5.8/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/gabrielpgava/open-lake-controller-5.8/release.yml?branch=main&style=flat-square)](https://github.com/gabrielpgava/open-lake-controller-5.8/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](LICENSE)

A modern, open-source reconstruction of **Lake Controller v5.8**, engineered to keep legacy and modern Lake DSP hardware running seamlessly on modern operating systems (Windows 10, Windows 11, macOS, and Linux).

Specifically crafted to preserve mission-critical live sound and touring hardware including the **Dolby Lake Processor (DLP)**, Lake Mesa Quad EQ, LM 26, LM 44, and Lab.gruppen PLM Series amplifiers.

---

## 🎯 Purpose & Mission

The original Lake Controller was built during the Windows XP and Windows 7 era, relying on legacy DirectDraw exclusive fullscreen surfaces and Win32 display APIs. On modern operating systems, the original binary suffers from:
- Graphical freezes and unresponsive displays.
- White screen on minimize/restore due to lost DirectDraw surfaces without recovery handlers.
- Entire computer lockups caused by exclusive display mode conflicts with modern Desktop Window Manager (DWM).

**Open Lake Controller** solves this by:
1. **Preserving Dolby Lake Hardware**: Ensuring touring sound engineers, system technicians, and venues can continue operating their **Dolby Lake Processors** and Lake hardware reliably on current laptops without maintaining obsolete Windows 7 machines or unreliable virtual machines.
2. **Native Windowed Mode**: Providing a clean, resizable, fully responsive desktop window with hardware-accelerated rendering.
3. **Faithful Visual Design**: Recreating the authentic Lake tactile user experience (continuous rounded yellow frames, glossy 3D module spheres, tactile F1–F10 gel capsules, and 14-segment LED bar graph meters).
4. **Stable Audio Metering & Offline Operation**: Medidores and EQ curves remain at proper resting state (0.0 dB) when offline, eliminating artificial autonomous meter movements.
5. **Network Interface Management**: Native UDP port 6004 broadcast discovery with dedicated network adapter (NIC) selection for Dante and control subnets.

---

## 🖥️ Live Project Showcase

Visit our GitHub Pages presentation site:  
👉 **[https://gabrielpgava.github.io/open-lake-controller-5.8/](https://gabrielpgava.github.io/open-lake-controller-5.8/)**

---

## 📦 Features

- **Full Cross-Platform Support**: Ready for Windows, macOS (Apple Silicon & Intel), and Linux.
- **Decompiled Parameter Accuracy**: Exact filter limits (10 Hz – 20,000 Hz), gain ranges (-15 dB to +15 dB), and limiter dynamics decompiled directly from original Lake libraries (`iomath.dll`, `Contour.dll`, and `Lake Controller.exe`).
- **Interactive EQ Worksheets**: Parametric EQ (PEQ), Graphic EQ (GEQ), crossover filter slopes, and delay alignment.
- **Automated Multi-Platform Release Pipeline**: GitHub Actions workflow and Makefile to build, package, and publish releases automatically for all major platforms.

---

## 🚀 Building from Source

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.75+ recommended)
- Standard build tools (`make` or cargo)

### Using Makefile
```bash
# Build for current host platform
make build

# Build optimized release binary
make release

# Run the controller
make run

# Take an automated verification screenshot
make screenshot
```

### Multi-Platform Cross-Compilation (via Makefile)
```bash
# Build all platforms (requires cross: cargo install cross)
make all-platforms

# Package release archives for all platforms
make package-all
```

---

## 🤖 GitHub Releases Automation

This repository includes a pre-configured GitHub Actions workflow (`.github/workflows/release.yml`).  
Whenever you push a version tag (e.g. `git tag v5.8.0 && git push origin v5.8.0`), GitHub Actions will automatically:
1. Compile optimized binaries for:
   - **Windows** (`x86_64-pc-windows-msvc` / `.exe` in `.zip`)
   - **macOS** (`x86_64-apple-darwin` & `aarch64-apple-darwin` / Universal `.tar.gz`)
   - **Linux** (`x86_64-unknown-linux-gnu` / `.tar.gz`)
2. Generate SHA-256 checksums.
3. Publish a new GitHub Release with all downloadable assets attached.

---

## 📄 License & Community Notice

This project is an independent open-source hardware preservation effort created for the audio community to ensure legacy **Dolby Lake** hardware remains functional and maintainable. All trademarks belong to their respective holders.
