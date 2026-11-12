use crate::model::{InputSource, Module, SampleRate};
use crate::theme::LakeTheme;
use eframe::egui::{self, RichText, Rounding, Ui};

pub fn render_worksheet_io(ui: &mut Ui, theme: &LakeTheme, module: &mut Module) {
    ui.vertical(|ui| {
        ui.label(
            RichText::new("FRAME CONFIGURATION & I/O ROUTER")
                .font(egui::FontId::proportional(15.0))
                .strong()
                .color(theme.text_primary),
        );
        ui.label(
            RichText::new("Configure physical input sources, digital clock sync, and input router mutes.")
                .font(egui::FontId::proportional(11.0))
                .color(theme.text_muted),
        );

        ui.add_space(14.0);

        egui::Frame::none()
            .fill(theme.bg_card)
            .rounding(Rounding::same(6.0))
            .inner_margin(egui::Margin::symmetric(16.0, 14.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Input Source Selection
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("INPUT ROUTER SOURCE")
                                .font(egui::FontId::proportional(13.0))
                                .strong()
                                .color(theme.cyan_accent),
                        );
                        ui.add_space(6.0);

                        ui.radio_value(&mut module.input_source, InputSource::Analog, "Analog Inputs (Auto-Detect)");
                        ui.radio_value(&mut module.input_source, InputSource::AES3, "AES/EBU Digital (AES3)");
                        ui.radio_value(&mut module.input_source, InputSource::Dante, "Audinate Dante IP Audio Stream");
                    });

                    ui.add_space(40.0);

                    // Sample Rate Selection
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("SYSTEM SAMPLE RATE")
                                .font(egui::FontId::proportional(13.0))
                                .strong()
                                .color(theme.lime_accent),
                        );
                        ui.add_space(6.0);

                        ui.radio_value(&mut module.sample_rate, SampleRate::Rate44_1k, "44.1 kHz (Standard Red Book)");
                        ui.radio_value(&mut module.sample_rate, SampleRate::Rate48k, "48.0 kHz (Broadcast Standard)");
                        ui.radio_value(&mut module.sample_rate, SampleRate::Rate96k, "96.0 kHz (High-Resolution Studio/Live)");
                    });

                    ui.add_space(40.0);

                    // Clock Sync & Termination
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("DIGITAL CLOCK & TERMINATION")
                                .font(egui::FontId::proportional(13.0))
                                .strong()
                                .color(theme.orange_accent),
                        );
                        ui.add_space(6.0);

                        ui.label("Sync Source: Dante Master Clock (Locked)");
                        ui.label("AES In Termination: 110 Ω Terminated");
                        ui.label("AES Output Sync: 96 kHz Internal");
                    });
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(10.0);

                // Reset Action
                ui.horizontal(|ui| {
                    if ui.button(RichText::new("RESTORE FACTORY DEFAULTS").color(theme.red_clip)).clicked() {
                        module.peq_filters = vec![
                            crate::model::PeqFilter {
                                filter_type: crate::model::FilterType::HighPass,
                                freq: 35.0,
                                gain: 0.0,
                                bandwidth: 0.7,
                                enabled: true,
                            },
                            crate::model::PeqFilter {
                                filter_type: crate::model::FilterType::LowShelf,
                                freq: 120.0,
                                gain: 0.0,
                                bandwidth: 0.8,
                                enabled: true,
                            },
                        ];
                        module.geq_bands = [0.0; 28];
                        module.geq_enabled = false;
                        for ch in &mut module.inputs {
                            ch.gain_db = 0.0;
                            ch.delay_ms = 0.0;
                            ch.muted = false;
                        }
                        for ch in &mut module.outputs {
                            ch.gain_db = 0.0;
                            ch.delay_ms = 0.0;
                            ch.muted = false;
                        }
                    }

                    ui.label(
                        RichText::new("Resets all PEQ filters, delays, and channel gains for this module to factory calibration.")
                            .font(egui::FontId::proportional(11.0))
                            .color(theme.text_muted),
                    );
                });
            });
    });
}
