# Lake Controller v5.8 Decompiled Mechanics & System Constants

Evidence gathered from deconstruction and disassembly of `Lake Controller.exe`, `Contour.dll`, and `iomath.dll` in Deep mode via Ghidra.

## 1. Graphics & Window Management
- **Issue in Legacy Binary**: The original binary defaults to exclusive fullscreen and hides the taskbar (`ShowWindow(FindWindowA("Shell_TrayWnd", 0), SW_HIDE)`). In `Contour.dll` function `0x1002e4e0` (`FUN_1002e4e0`), it creates a `ContourParent` popup window (`WS_POPUP | WS_MINIMIZEBOX`, 1024x768 / `0x400` x `0x300`) with DirectDraw exclusive surfaces. On modern Windows (Windows 10/11 DWM), minimizing causes lost surfaces (`DDERR_SURFACELOST`) without proper restore recovery, freezing display threads into an infinite loop.
- **CLI Options Parsed in `0x1002e4e0`**:
  - `fullscreen`: sets `DAT_10c8aa04 = 2; DAT_106bb258 = 1`
  - `nonfullscreen`: sets `DAT_10c8aa04 = 1; DAT_106bb258 = 0`
  - `windowed`: CLI flag reference at `0x10464f50`
  - `offline`: sets `DAT_10c8a8bc = 1`
  - `arbitrary`: sets `DAT_10c8a8e8 = 1`
  - `rate=%d`: frame rate override (`DAT_106bb298`)
- **Modern Solution**: Rust with `eframe`/`egui` in native windowed mode, hardware accelerated via OpenGL/wgpu, fully stable under Windows minimize, restore, and resize events.

## 2. DSP & Audio Mechanics
- **Speed of Sound**: `343.6 m/s` at 20°C (`iomath.dll` function `0x100010c0` and `Contour.dll` string table references).
- **Delay Range**: `0.0 ms` to `2000.0 ms` (`Contour.dll` function `0x1002e4e0`). Display units switchable between milliseconds, meters, and feet.
- **Parametric EQ (PEQ)**:
  - Minimum Frequency: `10.0 Hz` (`iomath.dll` function `0x10001920`, `nGetLoPassFilterMinMax`).
  - Maximum Frequency: `20000.0 Hz` (`iomath.dll` function `0x10001920`, `nGetLoPassFilterMinMax`).
  - Gain Range: `-15.0 dB` to `+15.0 dB` (`iomath.dll` function `0x100010c0`).
  - Bandwidth (Q): `0.1 octaves` to `10.0 octaves` (`iomath.dll` function `0x100010c0`).
  - Filter Types: Bell Parametric, Low Shelf, High Shelf, HPF (High-Pass), LPF (Low-Pass).
- **Graphic Equalizer (GEQ)**:
  - 28-band Raised Cosine graphic equalizer filters (`Contour.dll` WorksheetGEQ `0x1027d770`).
- **LimiterMax Parameters**:
  - MaxRMS Level, Corner, Attack, Release.
  - Automatic release time constant (Tc Auto): `160 ms` (`Contour.dll` function `0x103de260`).
  - MaxPeak Threshold disable level: `>= 25.0 dBu` (`Contour.dll` `0x104607bc`).
- **Crossover Slopes**:
  - Butterworth, Bessel, Linkwitz-Riley from 6 dB/oct to 48 dB/oct (`iomath.dll` function `0x10003260`).
  - Linear Phase crossovers: LP24, LP48, LPBW.

## 3. Network & System Architecture
- **Startup Mutex**: `Lake_Contour_CC_Startup_Mutex` (`Lake Controller.exe` function `0x4056b0`, `FUN_004056b0`).
- **Default UDP Port**: `6004` (`Lake Controller.exe` function `0x4056b0`, plugin init via vtable+0x18).
- **Sample Rates**: `44.1 kHz`, `48.0 kHz`, `96.0 kHz` (`Contour.dll` function `0x1002e4e0`).
