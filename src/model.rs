#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleType {
    Contour,
    MesaEq,
    LM26,
    LM44,
    PLM10000Q,
    PLM20000Q,
}

impl ModuleType {
    pub fn display_name(&self) -> &'static str {
        match self {
            ModuleType::Contour => "Dolby Lake Contour",
            ModuleType::MesaEq => "Lake Mesa EQ",
            ModuleType::LM26 => "Lake LM 26",
            ModuleType::LM44 => "Lake LM 44",
            ModuleType::PLM10000Q => "Lab.gruppen PLM 10000Q",
            ModuleType::PLM20000Q => "Lab.gruppen PLM 20000Q",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            ModuleType::Contour => "Contour",
            ModuleType::MesaEq => "Mesa EQ",
            ModuleType::LM26 => "LM 26",
            ModuleType::LM44 => "LM 44",
            ModuleType::PLM10000Q => "PLM 10000Q",
            ModuleType::PLM20000Q => "PLM 20000Q",
        }
    }

    pub fn input_count(&self) -> usize {
        match self {
            ModuleType::Contour => 2,
            ModuleType::MesaEq => 4,
            ModuleType::LM26 => 2,
            ModuleType::LM44 => 4,
            ModuleType::PLM10000Q => 4,
            ModuleType::PLM20000Q => 4,
        }
    }

    pub fn output_count(&self) -> usize {
        match self {
            ModuleType::Contour => 6,
            ModuleType::MesaEq => 4,
            ModuleType::LM26 => 6,
            ModuleType::LM44 => 4,
            ModuleType::PLM10000Q => 4,
            ModuleType::PLM20000Q => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterType {
    ParametricBell,
    LowShelf,
    HighShelf,
    HighPass,
    LowPass,
    MesaFilter,
}

impl FilterType {
    pub fn display_name(&self) -> &'static str {
        match self {
            FilterType::ParametricBell => "PEQ Bell",
            FilterType::LowShelf => "Low Shelf",
            FilterType::HighShelf => "High Shelf",
            FilterType::HighPass => "HPF (High Pass)",
            FilterType::LowPass => "LPF (Low Pass)",
            FilterType::MesaFilter => "Mesa Raised-Cosine",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeqFilter {
    pub filter_type: FilterType,
    pub freq: f32,       // 10 Hz - 20000 Hz
    pub gain: f32,       // -15 dB - +15 dB
    pub bandwidth: f32,  // 0.1 - 10.0 octaves
    pub enabled: bool,
}

impl Default for PeqFilter {
    fn default() -> Self {
        Self {
            filter_type: FilterType::ParametricBell,
            freq: 1000.0,
            gain: 0.0,
            bandwidth: 0.5,
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelStrip {
    pub name: String,
    pub gain_db: f32,         // -60 to +12 dB
    pub delay_ms: f32,        // 0 to 2000 ms
    pub muted: bool,
    pub inverted_polarity: bool,
    pub meter_level: f32,     // 0.0 to 1.0 (live VU)
    pub peak_hold: f32,
    pub is_clip: bool,
}

impl ChannelStrip {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            gain_db: 0.0,
            delay_ms: 0.0,
            muted: false,
            inverted_polarity: false,
            meter_level: 0.15,
            peak_hold: 0.2,
            is_clip: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimiterMaxSettings {
    pub max_rms_level_dbu: f32, // -10 to +24 dBu
    pub corner_soft: bool,
    pub attack_ms: f32,         // 1 to 100 ms
    pub release_ms: f32,        // 10 to 2000 ms
    pub auto_release: bool,     // 160 ms auto Tc
    pub max_peak_level_dbu: f32,// 0 to 25 dBu (>=25 disabled)
    pub is_disabled: bool,
}

impl Default for LimiterMaxSettings {
    fn default() -> Self {
        Self {
            max_rms_level_dbu: 18.0,
            corner_soft: true,
            attack_ms: 10.0,
            release_ms: 160.0,
            auto_release: true,
            max_peak_level_dbu: 22.0,
            is_disabled: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputSource {
    Analog,
    AES3,
    Dante,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SampleRate {
    Rate44_1k,
    Rate48k,
    Rate96k,
}

impl SampleRate {
    pub fn display(&self) -> &'static str {
        match self {
            SampleRate::Rate44_1k => "44.1 kHz",
            SampleRate::Rate48k => "48.0 kHz",
            SampleRate::Rate96k => "96.0 kHz",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: usize,
    pub name: String,
    pub frame_name: String,
    pub ip_address: String,
    pub module_type: ModuleType,
    pub module_letter: char,
    pub preset_name: String,
    pub frame_index: usize,
    pub pos: (f32, f32),
    pub inputs: Vec<ChannelStrip>,
    pub outputs: Vec<ChannelStrip>,
    pub peq_filters: Vec<PeqFilter>,
    pub geq_bands: [f32; 28], // -12 dB to +12 dB
    pub geq_enabled: bool,
    pub limiter: LimiterMaxSettings,
    pub input_source: InputSource,
    pub sample_rate: SampleRate,
    pub group_id: Option<usize>,
    pub is_selected: bool,
    pub is_virtual: bool,
    pub is_online: bool,
}

impl Module {
    pub fn new(id: usize, name: &str, frame_name: &str, ip: &str, mtype: ModuleType, pos: (f32, f32)) -> Self {
        let input_count = mtype.input_count();
        let output_count = mtype.output_count();

        let inputs = (1..=input_count)
            .map(|i| ChannelStrip::new(format!("In {}", i)))
            .collect();

        let outputs = (1..=output_count)
            .map(|i| ChannelStrip::new(format!("Out {}", i)))
            .collect();

        // Standard default filters
        let peq_filters = vec![
            PeqFilter { filter_type: FilterType::HighPass, freq: 35.0, gain: 0.0, bandwidth: 0.7, enabled: true },
            PeqFilter { filter_type: FilterType::LowShelf, freq: 120.0, gain: 2.5, bandwidth: 0.8, enabled: true },
            PeqFilter { filter_type: FilterType::ParametricBell, freq: 350.0, gain: -1.5, bandwidth: 0.4, enabled: true },
            PeqFilter { filter_type: FilterType::ParametricBell, freq: 2200.0, gain: 1.0, bandwidth: 0.35, enabled: true },
            PeqFilter { filter_type: FilterType::HighShelf, freq: 10000.0, gain: 2.0, bandwidth: 0.7, enabled: true },
            PeqFilter { filter_type: FilterType::LowPass, freq: 18500.0, gain: 0.0, bandwidth: 0.7, enabled: false },
        ];

        let module_letter = match id % 4 {
            1 => 'A',
            2 => 'B',
            3 => 'C',
            _ => 'D',
        };

        Self {
            id,
            name: name.to_string(),
            frame_name: frame_name.to_string(),
            ip_address: ip.to_string(),
            module_type: mtype,
            module_letter,
            preset_name: "CL3Way".to_string(),
            frame_index: (id + 1) / 2,
            pos,
            inputs,
            outputs,
            peq_filters,
            geq_bands: [0.0; 28],
            geq_enabled: false,
            limiter: LimiterMaxSettings::default(),
            input_source: InputSource::Dante,
            sample_rate: SampleRate::Rate96k,
            group_id: None,
            is_selected: false,
            is_virtual: false,
            is_online: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: usize,
    pub name: String,
    pub color_rgb: [u8; 3],
    pub module_ids: Vec<usize>,
    pub muted: bool,
    pub gain_offset_db: f32,
    pub delay_offset_ms: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DelayDisplayUnit {
    Milliseconds,
    Meters,
    Feet,
}

impl DelayDisplayUnit {
    pub fn label(&self) -> &'static str {
        match self {
            DelayDisplayUnit::Milliseconds => "ms",
            DelayDisplayUnit::Meters => "m",
            DelayDisplayUnit::Feet => "ft",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppView {
    HomeWorkarea,
    ModuleEditor,
    SoloMuteMatrix,
    SystemStoreRecall,
    UserPreferences,
    NetworkConfig,
    ConnectedDevices,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAdapterInfo {
    pub name: String,
    pub ip: String,
    pub subnet: String,
    pub is_connected: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleEditorTab {
    SuperEq,
    GraphicEq,
    LevelsDelay,
    LimiterMax,
    IOConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BottomMenuContext {
    Home,
    SystemStoreRecall,
    Modules,
    Groups,
    SoloMute,
    Pages,
    UserPreferences,
    IconControl,
    Network,
}
