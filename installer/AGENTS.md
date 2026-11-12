# AGENTS.md - Development Rules & Guidelines for Open DolbyLake Software

This document defines mandatory protocols, version management rules, and development standards for all AI agents and contributors working on **Open DolbyLake Software**.

---

## 📌 1. Automatic Version Increment Rule (Mandatory)

Whenever the software evolves (new features, visual redesigns, bug fixes, hardware protocols, DSP updates):
1. **Semantic Versioning Scheme**:
   - Format: `MAJOR.MINOR.PATCH` (starting at `0.0.1` for initial release).
   - `PATCH` (+0.0.1): UI refinements, small fixes, styling, preset updates.
   - `MINOR` (+0.1.0): New functional views, network communication changes, new DSP tools.
   - `MAJOR` (+1.0.0): Real hardware validation completed on a physical Dolby Lake Processor or breaking architectural rewrite.
2. **Synchronization Checklist**:
   Always bump the version simultaneously in:
   - `Cargo.toml` (`[package] version = "X.Y.Z"`)
   - `src/updater.rs` (`pub const CURRENT_VERSION: &str = "X.Y.Z";`)
   - `src/main.rs` (`with_title` and console start text)
   - `Makefile` (`VERSION = X.Y.Z`)
   - `README.md` and release notes

---

## ⚠️ 2. Real Hardware Notice & Safety Warning

Until testing and certification are conducted against physical DSP hardware:
- Every build, release, UI header, and documentation MUST clearly display:
  > **"Aviso: Versão experimental vX.Y.Z - Ainda não foi testada em equipamento físico/real."**
  > **("Warning: Experimental build vX.Y.Z - Not yet verified on physical Dolby Lake hardware.")**
- Do not transmit hazardous high-gain commands or unvalidated limiter bypasses over live UDP 6004 without user confirmation.

---

## 🪟 3. Windowed Architecture Standards

- The software must always launch in native modern windowed mode by default.
- Never reinstate legacy exclusive fullscreen DirectDraw or DDraw surface flips that lock up Windows 10/11 DWM.
- Support smooth window resizing, minimization, and multi-monitor movement without frame loss or crashes.

---

## 🛠️ 4. Build, PE Patch & Toolchain Integrity

- The binary uses MinGW/GCC toolchain with dynamic PE import patching via `patch_pe.py`.
- After modifying imports or crates, ensure `cargo build` and `patch_pe.py` succeed cleanly.
- Maintain support for `--screenshot <filename.png>` for automated headless verification.
