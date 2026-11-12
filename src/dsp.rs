#![allow(dead_code)]

use crate::model::{DelayDisplayUnit, FilterType, PeqFilter};

/// Speed of sound in dry air at 20°C in m/s (extracted from iomath.dll & Contour.dll)
pub const SPEED_OF_SOUND_MPS: f32 = 343.6;

/// 28 ISO Standard 1/3-octave center frequencies (Hz) for Lake Controller GEQ
pub const GEQ_FREQUENCIES: [f32; 28] = [
    31.5, 40.0, 50.0, 63.0, 80.0, 100.0, 125.0, 160.0, 200.0, 250.0, 315.0, 400.0, 500.0, 630.0,
    800.0, 1000.0, 1250.0, 1600.0, 2000.0, 2500.0, 3150.0, 4000.0, 5000.0, 6300.0, 8000.0,
    10000.0, 12500.0, 16000.0,
];

/// Convert delay in milliseconds to distance in meters
pub fn ms_to_meters(ms: f32) -> f32 {
    ms * (SPEED_OF_SOUND_MPS / 1000.0)
}

/// Convert distance in meters to delay in milliseconds
pub fn meters_to_ms(meters: f32) -> f32 {
    if SPEED_OF_SOUND_MPS <= 0.0 {
        0.0
    } else {
        (meters / SPEED_OF_SOUND_MPS) * 1000.0
    }
}

/// Convert delay in milliseconds to distance in feet
pub fn ms_to_feet(ms: f32) -> f32 {
    ms_to_meters(ms) * 3.28084
}

/// Convert distance in feet to delay in milliseconds
pub fn feet_to_ms(feet: f32) -> f32 {
    meters_to_ms(feet / 3.28084)
}

/// Format delay based on current display unit
pub fn format_delay(ms: f32, unit: DelayDisplayUnit) -> String {
    match unit {
        DelayDisplayUnit::Milliseconds => format!("{:.2} ms", ms),
        DelayDisplayUnit::Meters => format!("{:.2} m", ms_to_meters(ms)),
        DelayDisplayUnit::Feet => format!("{:.2} ft", ms_to_feet(ms)),
    }
}

/// Evaluate individual PEQ filter magnitude response in dB at a given frequency
pub fn eval_filter_magnitude_db(f: f32, filter: &PeqFilter) -> f32 {
    if !filter.enabled {
        return 0.0;
    }

    let f0 = filter.freq.max(10.0).min(20000.0);
    let gain = filter.gain.clamp(-15.0, 15.0);
    let bw = filter.bandwidth.clamp(0.1, 10.0);

    // Q approximation from bandwidth in octaves: Q = 1 / (2 * sinh(ln(2)/2 * bw))
    let q = 1.0 / (2.0 * ((0.34657359 * bw).sinh()).max(0.05));

    match filter.filter_type {
        FilterType::ParametricBell => {
            // Standard peaking EQ magnitude approximation:
            // Delta log-ratio of freq
            let ratio = f / f0;
            let log_dist = (ratio.ln()).abs() * q;
            gain / (1.0 + log_dist * log_dist)
        }
        FilterType::LowShelf => {
            // Smooth 2nd-order low shelf
            let ratio = f / f0;
            gain / (1.0 + ratio.powf(2.0 * q.clamp(0.5, 2.5)))
        }
        FilterType::HighShelf => {
            // Smooth 2nd-order high shelf
            let ratio = f0 / f;
            gain / (1.0 + ratio.powf(2.0 * q.clamp(0.5, 2.5)))
        }
        FilterType::HighPass => {
            // 24 dB/oct HPF Butterworth curve
            let ratio = (f / f0).max(1e-4);
            let mag = ratio.powi(4) / (1.0 + ratio.powi(4));
            (20.0 * mag.log10()).max(-48.0)
        }
        FilterType::LowPass => {
            // 24 dB/oct LPF Butterworth curve
            let ratio = (f0 / f).max(1e-4);
            let mag = ratio.powi(4) / (1.0 + ratio.powi(4));
            (20.0 * mag.log10()).max(-48.0)
        }
        FilterType::MesaFilter => {
            // Lake's signature Mesa filter: Flat top plateau with raised-cosine shoulders
            let half_bw = (2.0f32.powf(bw * 0.5) - 1.0) * f0;
            let f_low = (f0 - half_bw).max(10.0);
            let f_high = (f0 + half_bw).min(20000.0);

            if f >= f_low && f <= f_high {
                gain
            } else if f < f_low {
                let dist = (f_low - f) / (f0 * 0.3 * bw.max(0.2));
                gain * (1.0 / (1.0 + dist * dist))
            } else {
                let dist = (f - f_high) / (f0 * 0.3 * bw.max(0.2));
                gain * (1.0 / (1.0 + dist * dist))
            }
        }
    }
}

/// Evaluate 28-band Raised Cosine GEQ response in dB at frequency f
pub fn eval_geq_magnitude_db(f: f32, bands: &[f32; 28]) -> f32 {
    let mut total_db = 0.0;
    for (i, &f0) in GEQ_FREQUENCIES.iter().enumerate() {
        let gain = bands[i];
        if gain.abs() > 0.05 {
            // Raised cosine filter skirt
            let ratio = f / f0;
            let oct_dist = ratio.log2().abs();
            if oct_dist < 1.0 {
                let weight = 0.5 * (1.0 + (std::f32::consts::PI * oct_dist).cos());
                total_db += gain * weight;
            }
        }
    }
    total_db
}

/// Compute overall EQ response at frequency f (sum of all enabled PEQ filters + GEQ)
pub fn eval_total_eq_db(f: f32, filters: &[PeqFilter], geq_bands: &[f32; 28], geq_enabled: bool) -> f32 {
    let mut total_db = 0.0;
    for flt in filters {
        total_db += eval_filter_magnitude_db(f, flt);
    }
    if geq_enabled {
        total_db += eval_geq_magnitude_db(f, geq_bands);
    }
    total_db.clamp(-30.0, 30.0)
}

/// Generate curve points (freq, dB) across the audio spectrum (10 Hz to 20,000 Hz)
pub fn generate_curve_points(
    filters: &[PeqFilter],
    geq_bands: &[f32; 28],
    geq_enabled: bool,
    steps: usize,
) -> Vec<(f32, f32)> {
    let mut points = Vec::with_capacity(steps);
    let min_log = 10.0f32.log10();
    let max_log = 20000.0f32.log10();

    for i in 0..steps {
        let t = i as f32 / (steps - 1) as f32;
        let log_f = min_log + t * (max_log - min_log);
        let freq = 10.0f32.powf(log_f);
        let db = eval_total_eq_db(freq, filters, geq_bands, geq_enabled);
        points.push((freq, db));
    }

    points
}
