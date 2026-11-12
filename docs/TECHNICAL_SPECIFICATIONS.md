# Open DolbyLake Software - Technical Architecture, Protocols & Reverse Engineering Insights

This comprehensive document compiles all discovered architectural details, network protocol structures, memory signatures, DSP formulas, and reverse-engineering insights from the original legacy **Lake Controller v5.8**, `Lake Controller.exe`, `Contour.dll`, and `iomath.dll`.

It is specifically structured for **AI agents, future developers, and audio engineers** continuing the implementation of full hardware communication with the **Dolby Lake Processor (DLP)** and compatible Lake platforms.

---

## 1. The Legacy Windows Freeze Issue (Root Cause Analysis)

### 1.1 DirectDraw Surface Flipping & DWM Conflict
- **Original Entrypoint**: In `Contour.dll` function `0x1002e4e0` (`FUN_1002e4e0`), the application initializes a `ContourParent` window with styles `WS_POPUP | WS_VISIBLE | WS_MINIMIZEBOX`.
- **Display Mechanism**: The original program requests exclusive DirectDraw surface flipping via DirectX 7/8. When minimized on Windows 10/11:
  1. The Desktop Window Manager (DWM) compositor invalidates the exclusive surfaces (`DDERR_SURFACELOST`).
  2. The original legacy render loop lacks a surface restore handler, resulting in an infinite spin wait.
  3. The message pump thread starves, freezing the window and eventually locking the Windows desktop composition thread.
- **Solution in Open DolbyLake Software**: Rust with `egui` and modern hardware-accelerated OpenGL / wgpu (`glow` context) running in native resizable windowed mode.

---

## 2. Lake Discovery & Network Communications Protocol (UDP 6004)

### 2.1 Broadcast Discovery Architecture
- **Startup Mutex**: `Lake_Contour_CC_Startup_Mutex` (located in `Lake Controller.exe` function `0x4056b0`).
- **Primary UDP Port**: `6004` (both sender and receiver).
- **Subnet Broadcast**: Lake Controller transmits periodic UDP beacon packets to the broadcast address of the selected network adapter (e.g. `192.168.0.255:6004`).
- **Discovery Beacon Format**:
  - Magic Byte Sequence: `0xDD 0xF0` or ASCII header `LakeBeacon` / `Lake_Contour_CC`.
  - Processor response payload contains:
    - Processor model ID (`0x01` = Contour DLP, `0x02` = Mesa EQ, `0x03` = LM 26, `0x04` = LM 44, `0x05` = PLM 10000Q, `0x06` = PLM 20000Q).
    - Serial Number (string, up to 16 bytes).
    - Frame Name (ASCII, up to 32 bytes).
    - Subnet IP and Dante IP address.
    - Active Firmware revision (e.g. `Contour_4_1634.lkc`, `LM_00_01_05_FPGA1058`).
    - Sync status: Digital clock source (AES/EBU, Dante, Internal 96kHz, BNC Word Clock).

### 2.2 Network Interface Selection (NIC Binding)
- Touring setups commonly isolate Dante audio traffic from Lake control data, or run them on distinct VLANs:
  - Dante Subnet: typically link-local auto-IP `169.254.x.x` (or dedicated static subnet).
  - Control Subnet: typically `192.168.0.x` or `10.0.x.x`.
- The controller must bind specifically to the socket on the designated adapter IP rather than `0.0.0.0` to ensure broadcasts don't loop back or exit through inactive interfaces (e.g. Wi-Fi).

---

## 3. Audio Mechanics, DSP Constants & Formulas

Decompiled directly from `iomath.dll` and `Contour.dll`:

### 3.1 Delay & Acoustic Alignment
- **Acoustic Constant**: Speed of sound is defined as `343.6 m/s` at 20°C in dry air (`iomath.dll` function `0x100010c0`).
- **Formulas**:
  $$\text{Distance (meters)} = \text{Delay (ms)} \times 0.3436$$
  $$\text{Distance (feet)} = \text{Delay (ms)} \times 1.1273$$
- **Max Delay Range**: `0.0 ms` to `2000.0 ms` per channel strip (`Contour.dll` function `0x1002e4e0`).

### 3.2 Parametric Equalizer (PEQ)
- **Frequency Limits**: `10.0 Hz` minimum to `20,000.0 Hz` maximum (`iomath.dll` function `0x10001920`, `nGetLoPassFilterMinMax`).
- **Gain Range**: `-15.0 dB` to `+15.0 dB` with `0.1 dB` resolution.
- **Bandwidth (Q)**: `0.1 octaves` to `10.0 octaves`.
  $$\text{Q factor} = \frac{\sqrt{2^{\text{BW}}}}{2^{\text{BW}} - 1}$$
- **Filter Topologies**:
  - Parametric Bell (Asymmetrical Raised Cosine / Gaussian).
  - Low Shelf (6 dB/oct and 12 dB/oct).
  - High Shelf (6 dB/oct and 12 dB/oct).
  - High Pass Filter (HPF) & Low Pass Filter (LPF).

### 3.3 Graphic Equalizer (GEQ)
- **Bands**: 28 ISO standard 1/3-octave center frequencies (31.5 Hz to 16 kHz).
- **Topology**: Raised Cosine filtering (`WorksheetGEQ` `0x1027d770`), preventing phase ripple common in traditional minimum-phase graphic EQs.

### 3.4 LimiterMax Dynamics
- **Peak Threshold Range**: `0.0 dBu` to `+25.0 dBu` (thresholds $\ge 25.0\text{ dBu}$ disable peak limiting).
- **Auto Release Time Constant (Tc)**: `160 ms` (`Contour.dll` function `0x103de260`).
- **MaxRMS**: Separate RMS thermal protection limiter with soft-knee transition.

---

## 4. Lake Preset Storage & File Structures

Original files stored under `data/` and `presets/`:
1. **`.csm` (Contour Speaker Module)**:
   - Header: `\xdd\xf0\x0f\x00IOFileRev` (37,968 bytes).
   - Contains: Crossover filters, driver band assignments, LimiterMax profiles, and group routing.
2. **`.csc` (Contour System Configuration)**:
   - Header: `\xdd\xf0\x12\x00IOFileRev` (78,124 bytes).
   - Holds full system state across multiple frames and workspace positions.
3. **`.gpa`, `.lpa`, `.tpa` (Frame Preset Bundles)**:
   - Header: `\xdd\xf0\x8f\x00PresetListHdr`.
   - Used by **Lake LoadLibrary** to distribute manufacturer-approved loudspeaker presets (Adamson, Turbosound, JBL, L-Acoustics, d&b audiotechnik).

---

## 5. Roadmap & Directives for Future Agents

1. **Hardware Telemetry Loop**: Implement an async UDP socket listener on port `6004` to receive unsolicited VU meter telemetry packets and state heartbeat frames.
2. **Preset Exporter**: Implement an encoder/decoder for `.csm` binary files to allow loading custom user tunings directly onto physical Dolby Lake units.
3. **Safety Interlocks**: Enforce gain clamps (maximum +12 dB) to protect physical drivers during automated tests.
