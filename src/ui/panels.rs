use crate::model::{AppView, Group, Module};
use crate::theme::LakeTheme;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Ui};

pub fn render_solo_mute_matrix(
    ui: &mut Ui,
    theme: &LakeTheme,
    modules: &mut [Module],
    _groups: &mut [Group],
    app_view: &mut AppView,
) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("GLOBAL SOLO / MUTE CONTROL MATRIX")
                    .font(egui::FontId::proportional(16.0))
                    .strong()
                    .color(theme.text_primary),
            );

            ui.add_space(20.0);

            if ui.button(RichText::new("🔴 ALL MUTE").color(theme.red_clip)).clicked() {
                for m in modules.iter_mut() {
                    for out in &mut m.outputs {
                        out.muted = true;
                    }
                }
            }

            if ui.button("🟢 UNMUTE ALL").clicked() {
                for m in modules.iter_mut() {
                    for out in &mut m.outputs {
                        out.muted = false;
                    }
                }
            }

            ui.add_space(20.0);
            if ui.button("<< BACK TO WORKAREA").clicked() {
                *app_view = AppView::HomeWorkarea;
            }
        });

        ui.add_space(14.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            for m in modules.iter_mut() {
                egui::Frame::none()
                    .fill(theme.bg_card)
                    .rounding(Rounding::same(6.0))
                    .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("{} ({})", m.name, m.module_type.short_name()))
                                    .strong()
                                    .color(theme.cyan_accent),
                            );

                            ui.add_space(15.0);

                            for out in &mut m.outputs {
                                let mute_bg = if out.muted { theme.red_clip } else { theme.pill_btn_bg };
                                let mute_text_col = if out.muted { Color32::WHITE } else { theme.text_muted };
                                let btn = egui::Button::new(
                                    RichText::new(format!("{}: {}", out.name, if out.muted { "MUTED" } else { "ON" }))
                                        .font(egui::FontId::monospace(10.0))
                                        .color(mute_text_col),
                                )
                                .fill(mute_bg);

                                if ui.add(btn).clicked() {
                                    out.muted = !out.muted;
                                }
                            }
                        });
                    });
                ui.add_space(8.0);
            }
        });
    });
}

pub fn render_store_recall(
    ui: &mut Ui,
    theme: &LakeTheme,
    modules: &mut [Module],
    app_view: &mut AppView,
) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("SYSTEM STORE / RECALL PRESETS & LOUDSPEAKER LOADLIBRARY")
                    .font(egui::FontId::proportional(16.0))
                    .strong()
                    .color(theme.text_primary),
            );

            ui.add_space(20.0);
            if ui.button("<< BACK TO WORKAREA").clicked() {
                *app_view = AppView::HomeWorkarea;
            }
        });

        ui.add_space(10.0);

        // Explanatory Banner about Lake LoadLibrary & Famous Loudspeakers
        egui::Frame::none()
            .fill(theme.bg_card)
            .rounding(Rounding::same(6.0))
            .inner_margin(egui::Margin::symmetric(14.0, 10.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("🔊 BIBLIOTECA DE CAIXAS DE SOM FAMOSAS (LAKE LOADLIBRARY 2.7+)")
                            .font(egui::FontId::proportional(12.5))
                            .strong()
                            .color(theme.frame_yellow),
                    );
                });
                ui.label(
                    RichText::new(
                        "Sim! O software original possui a suíte oficial Lake LoadLibrary com arquivos de presets (.gpa, .lpa, .tpa, .csm) \
                        homologados para marcas consagradas como ADAMSON (Linhas Y10, Y18, SpekTrix, Metrix, Série M), TURBOSOUND (Linha Flex, Aspect, TFM), \
                        além de suportar módulos universais Contour/Mesa para L-Acoustics, JBL Pro, d&b audiotechnik e Clair Global. \
                        Abaixo você pode carregar diretamente os presets de fábrica de cada sistema nas caixas e módulos."
                    )
                    .font(egui::FontId::proportional(11.0))
                    .color(theme.text_muted),
                );
            });

        ui.add_space(12.0);

        let system_presets = [
            ("Adamson Y10 Line Array (4-Way Tri-amp)", "ADAMSON Y10 + T21 Subwoofer Crossover & LimiterMax Profile", "Adamson"),
            ("Adamson SpekTrix / Metrix Line Array", "ADAMSON Metrix Stereo + MSUB Cardioid Alignment", "Adamson"),
            ("Adamson M12 / M15 Stage Wedge Monitors", "ADAMSON M15 Active/Passive Bright & Flat Monitor Curves", "Adamson"),
            ("Turbosound Flex Array TFA-600", "Turbosound Flex FIR RevA Line Array Presets with TSW218 Sub", "Turbosound"),
            ("Turbosound Aspect & Flashlight Touring", "Turbosound Aspect Modular Touring PA System Curves", "Turbosound"),
            ("JBL VTX / VerTec Line Array Contour", "JBL Pro Touring Crossover (LP24 LinPhase, 80Hz Xo, LimiterMax)", "JBL"),
            ("L-Acoustics K-Standard Universal MEQ", "L-Acoustics Contour Alignment & 4-Band Contour Matrix", "L-Acoustics"),
            ("d&b audiotechnik J-Series Universal Contour", "d&b audiotechnik Touring System Contour + Mesa EQ Profile", "d&b"),
            ("Contour Classic Crossovers CL3way", "Contour 3-Way Classic Crossover Setup with LimiterMax", "Lake Factory"),
            ("Contour Linear Phase Crossovers LP4way", "Contour 4-Way Linear Phase Crossover Setup (Zero Phase Distortion)", "Lake Factory"),
            ("2x Contour 4x MEQ Modules", "2x Contour Loudspeaker Processors with 4x Mesa EQ Modules", "Lake Factory"),
            ("2x LM26 Modules", "2x Lake LM26 Main PA and Delay System", "Lake Factory"),
            ("2x LM44 MESA Modules", "2x Lake LM44 Multi-Engine Audio System", "Lake Factory"),
            ("2x PLM 10000Q Modules", "2x Lab.gruppen PLM 10000Q 4-channel Powered Loudspeaker Processors", "Lab.gruppen"),
            ("2x PLM 20000Q Modules", "2x Lab.gruppen PLM 20000Q Touring Sub/Main High Power Arrays", "Lab.gruppen"),
        ];

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (title, desc, brand) in system_presets {
                egui::Frame::none()
                    .fill(theme.bg_card)
                    .rounding(Rounding::same(6.0))
                    .inner_margin(egui::Margin::symmetric(16.0, 10.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    let badge_color = match brand {
                                        "Adamson" => Color32::from_rgb(245, 168, 28),
                                        "Turbosound" => Color32::from_rgb(60, 185, 240),
                                        "JBL" | "L-Acoustics" | "d&b" => Color32::from_rgb(255, 100, 100),
                                        _ => theme.lime_accent,
                                    };
                                    ui.label(
                                        RichText::new(format!("[{}]", brand))
                                            .font(egui::FontId::monospace(10.5))
                                            .color(badge_color),
                                    );
                                    ui.label(
                                        RichText::new(title)
                                            .font(egui::FontId::proportional(13.5))
                                            .strong()
                                            .color(theme.text_primary),
                                    );
                                });
                                ui.label(
                                    RichText::new(desc)
                                        .font(egui::FontId::proportional(11.0))
                                        .color(theme.text_muted),
                                );
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(RichText::new("CARREGAR PRESET").color(theme.frame_yellow).strong()).clicked() {
                                    // Preset recall feedback
                                    for m in modules.iter_mut() {
                                        m.preset_name = title.to_string();
                                        for out in &mut m.outputs {
                                            out.muted = false;
                                        }
                                    }
                                }
                            });
                        });
                    });
                ui.add_space(6.0);
            }
        });
    });
}

pub fn render_user_preferences(
    ui: &mut Ui,
    theme: &LakeTheme,
    windowed_mode: &mut bool,
    daylight_mode: &mut bool,
    app_view: &mut AppView,
    updater: &crate::updater::AutoUpdater,
) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("USER PREFERENCES & SYSTEM UPDATES")
                    .font(egui::FontId::proportional(16.0))
                    .strong()
                    .color(theme.text_primary),
            );

            ui.add_space(20.0);
            if ui.button("<< BACK TO WORKAREA").clicked() {
                *app_view = AppView::HomeWorkarea;
            }
        });

        ui.add_space(14.0);

        egui::Frame::none()
            .fill(theme.bg_card)
            .rounding(Rounding::same(6.0))
            .inner_margin(egui::Margin::symmetric(18.0, 16.0))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Auto-Update Section
                    ui.label(
                        RichText::new("GITHUB AUTO-UPDATE & VERSION MANAGER")
                            .font(egui::FontId::proportional(14.0))
                            .strong()
                            .color(theme.frame_yellow),
                    );

                    ui.horizontal(|ui| {
                        ui.label(format!("Current Version: v{}", crate::updater::CURRENT_VERSION));
                        ui.add_space(15.0);

                        match updater.get_status() {
                            crate::updater::UpdateStatus::Idle => {
                                if ui.button("Check for Updates Now").clicked() {
                                    updater.check_for_updates();
                                }
                            }
                            crate::updater::UpdateStatus::Checking => {
                                ui.spinner();
                                ui.label(RichText::new("Checking GitHub releases...").color(theme.frame_cyan));
                            }
                            crate::updater::UpdateStatus::UpToDate(msg) => {
                                ui.label(RichText::new(format!("✔ Up to date ({})", msg)).color(theme.meter_green).strong());
                                if ui.button("Re-check").clicked() {
                                    updater.check_for_updates();
                                }
                            }
                            crate::updater::UpdateStatus::UpdateAvailable { version, download_url, release_notes } => {
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new(format!("🚀 New Version Available: v{}", version))
                                            .color(theme.frame_yellow)
                                            .strong(),
                                    );
                                    if !release_notes.is_empty() {
                                        ui.label(format!("Notes: {}", release_notes));
                                    }
                                    if ui.button(RichText::new("Download & Install Update").color(theme.meter_green).strong()).clicked() {
                                        updater.download_and_install(download_url);
                                    }
                                });
                            }
                            crate::updater::UpdateStatus::Downloading(p) => {
                                ui.label(format!("Downloading update: {:.0}%", p * 100.0));
                            }
                            crate::updater::UpdateStatus::Downloaded(msg) => {
                                ui.label(RichText::new(msg).color(theme.meter_green));
                            }
                            crate::updater::UpdateStatus::Error(err) => {
                                ui.label(RichText::new(format!("⚠ Update check notice: {}", err)).color(theme.red_clip));
                                if ui.button("Try Again").clicked() {
                                    updater.check_for_updates();
                                }
                            }
                        }
                    });

                    ui.add_space(14.0);
                    ui.separator();
                    ui.add_space(12.0);

                    ui.label(
                        RichText::new("MODERN WINDOWED DISPLAY ENGINE")
                            .font(egui::FontId::proportional(14.0))
                            .strong()
                            .color(theme.cyan_accent),
                    );
                    ui.label(
                        "Resolved issue where legacy DirectX 7/8 exclusive mode froze on modern Windows 10/11 when minimizing or reopening. \
                        This rebuilt engine uses modern GPU-accelerated windowed graphics (egui / wgpu / glow) with full minimize/restore safety."
                    );

                    ui.add_space(12.0);

                    ui.horizontal(|ui| {
                        if ui.checkbox(windowed_mode, "Windowed Mode (Recommended for Windows 10/11)").changed() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Fullscreen(!*windowed_mode));
                        }

                        ui.add_space(30.0);

                        ui.checkbox(daylight_mode, "Daylight Graphics Mode (High-contrast outdoor theme)");
                    });

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(10.0);

                    ui.label(RichText::new("NETWORK & SYSTEM CONSTANTS").strong().color(theme.lime_accent));
                    ui.label("• Default UDP Discovery Port: 6004");
                    ui.label("• Startup Mutex Name: Lake_Contour_CC_Startup_Mutex");
                    ui.label("• Speed of Sound: 343.6 m/s (20°C in dry air)");
                    ui.label("• Max Delay Range: 2000.0 ms");
                    ui.label("• LimiterMax Auto Release (Tc): 160 ms");
                    ui.label("• Supported Sample Rates: 44.1 kHz, 48.0 kHz, 96.0 kHz");
                });
            });
    });
}

pub fn render_network_config(
    ui: &mut Ui,
    theme: &LakeTheme,
    adapters: &[crate::model::NetworkAdapterInfo],
    selected_adapter: &mut usize,
    is_online: &mut bool,
    simulate_audio: &mut bool,
    app_view: &mut AppView,
) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("LAKE CONTROLLER NETWORK CONFIGURATION")
                    .font(egui::FontId::proportional(16.0))
                    .strong()
                    .color(theme.text_primary),
            );

            ui.add_space(20.0);
            if ui.button(RichText::new("<< BACK TO WORKAREA").color(theme.frame_yellow)).clicked() {
                *app_view = AppView::HomeWorkarea;
            }
        });

        ui.add_space(14.0);

        egui::Frame::none()
            .fill(theme.bg_card)
            .rounding(Rounding::same(6.0))
            .inner_margin(egui::Margin::symmetric(20.0, 16.0))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // 1. Connection Mode & Explanation
                    ui.label(
                        RichText::new("STATUS DA CONEXÃO COM PROCESSADORES")
                            .font(egui::FontId::proportional(14.0))
                            .strong()
                            .color(theme.frame_cyan),
                    );

                    ui.horizontal(|ui| {
                        let status_text = if *is_online {
                            "ONLINE (Varrendo rede e sincronizando com processadores)"
                        } else {
                            "OFFLINE (Modo Bancada / Desconectado - Medidores em Repouso)"
                        };
                        let status_color = if *is_online { theme.meter_green } else { theme.meter_yellow };
                        ui.label(RichText::new(status_text).strong().color(status_color));

                        ui.add_space(20.0);
                        let toggle_label = if *is_online { "Mudar para Modo Offline" } else { "Conectar à Rede (Online)" };
                        if ui.button(toggle_label).clicked() {
                            *is_online = !*is_online;
                        }
                    });

                    ui.add_space(10.0);
                    ui.label(
                        RichText::new("Por que os gráficos e medidores não se mexem sozinhos agora?\n\
                        No software original, quando o sistema está offline ou sem receber áudio real dos processadores, \
                        os medidores permanecem em repouso (silêncio total / 0.0 dB). A animação simulada foi desativada por padrão \
                        para não causar confusão de leitura de sinal.")
                            .font(egui::FontId::proportional(11.5))
                            .color(theme.text_muted),
                    );

                    ui.add_space(6.0);
                    ui.checkbox(simulate_audio, "Ativar Simulação de Áudio de Teste nos Medidores (Modo Demonstração)");

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(12.0);

                    // 2. Network Adapter Selection
                    ui.label(
                        RichText::new("SELEÇÃO DE INTERFACE DE REDE (PLACA DE REDE / NIC)")
                            .font(egui::FontId::proportional(14.0))
                            .strong()
                            .color(theme.frame_yellow),
                    );
                    ui.label(
                        "Selecione qual adaptador de rede do computador deve ser utilizado pelo Lake Controller \
                        para se comunicar via broadcast UDP (porta 6004) com os processadores Contour, Mesa, LM e PLM:"
                    );

                    ui.add_space(8.0);

                    for (idx, adapter) in adapters.iter().enumerate() {
                        let is_selected = *selected_adapter == idx;
                        let card_stroke = if is_selected {
                            Stroke::new(1.5_f32, theme.frame_yellow)
                        } else {
                            Stroke::new(1.0_f32, Color32::from_rgb(40, 50, 60))
                        };

                        let card_bg = if is_selected {
                            Color32::from_rgb(20, 28, 38)
                        } else {
                            Color32::from_rgb(12, 16, 22)
                        };

                        egui::Frame::none()
                            .fill(card_bg)
                            .stroke(card_stroke)
                            .rounding(Rounding::same(4.0))
                            .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    if ui.radio(*selected_adapter == idx, "").clicked() {
                                        *selected_adapter = idx;
                                    }

                                    ui.vertical(|ui| {
                                        let name_color = if is_selected { theme.frame_yellow } else { theme.text_primary };
                                        ui.label(RichText::new(&adapter.name).strong().color(name_color));
                                        ui.label(
                                            RichText::new(format!("Endereço IPv4: {}   |   Máscara de Sub-rede: {}", adapter.ip, adapter.subnet))
                                                .font(egui::FontId::monospace(11.0))
                                                .color(theme.text_muted),
                                        );
                                    });

                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if is_selected {
                                            ui.label(RichText::new("INTERFACE ATIVA").strong().color(theme.meter_green));
                                        } else {
                                            if ui.button("Selecionar").clicked() {
                                                *selected_adapter = idx;
                                            }
                                        }
                                    });
                                });
                            });

                        ui.add_space(6.0);
                    }

                    ui.add_space(14.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // 3. UDP Discovery Details
                    ui.label(
                        RichText::new("PROTOCOLO DE DESCOBERTA LAKE (UDP 6004)")
                            .font(egui::FontId::proportional(13.0))
                            .strong()
                            .color(theme.frame_cyan),
                    );
                    ui.label(format!(
                        "• Porta UDP de Comunicação: 6004 (Vinculada)\n\
                         • IP de Transmissão de Descoberta: 192.168.0.255:6004\n\
                         • Mutex de Sistema: Lake_Contour_CC_Startup_Mutex\n\
                         • Frames Detectados na Sub-rede: {} frame(s)",
                        if *is_online { "3 (LM 26, LM 44, PLM 20k)" } else { "0 (Modo Offline)" }
                    ));

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button(RichText::new("🔍 Varrer Rede por Novos Processadores (Broadcast)").color(theme.frame_cyan)).clicked() {
                            *is_online = true;
                        }
                        ui.add_space(15.0);
                        if ui.button(RichText::new("📋 Ver Lista de Dispositivos Conectados").color(theme.frame_yellow).strong()).clicked() {
                            *app_view = AppView::ConnectedDevices;
                        }
                    });
                });
            });
    });
}

pub fn render_connected_devices(
    ui: &mut Ui,
    theme: &LakeTheme,
    modules: &mut Vec<Module>,
    is_online: &mut bool,
    app_view: &mut AppView,
) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("DISPOSITIVOS E PROCESSADORES LAKE CONECTADOS NA REDE")
                    .font(egui::FontId::proportional(16.0))
                    .strong()
                    .color(theme.text_primary),
            );

            ui.add_space(20.0);
            if ui.button("<< VOLTAR AO WORKSPACE").clicked() {
                *app_view = AppView::HomeWorkarea;
            }

            ui.add_space(10.0);
            if ui.button("🌐 CONFIGURAÇÃO DE REDE").clicked() {
                *app_view = AppView::NetworkConfig;
            }
        });

        ui.add_space(10.0);

        egui::Frame::none()
            .fill(theme.bg_card)
            .rounding(Rounding::same(6.0))
            .inner_margin(egui::Margin::symmetric(16.0, 10.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let online_txt = if *is_online { "ONLINE - Hardware físico sincronizado via UDP 6004" } else { "OFFLINE - Apenas dispositivos virtuais e bancada" };
                    let online_col = if *is_online { theme.meter_green } else { theme.meter_yellow };
                    ui.label(RichText::new(online_txt).strong().color(online_col));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(RichText::new("➕ Inserir Novo Frame Virtual").color(theme.lime_accent).strong()).clicked() {
                            let new_id = modules.iter().map(|m| m.id).max().unwrap_or(0) + 1;
                            let mut new_m = Module::new(
                                new_id,
                                "Mesa Virtual",
                                "Virtual Rack",
                                "192.168.1.150",
                                crate::model::ModuleType::LM44,
                                (100.0 + (new_id as f32 * 40.0) % 400.0, 100.0 + (new_id as f32 * 30.0) % 250.0),
                            );
                            new_m.is_virtual = true;
                            new_m.is_online = false;
                            modules.push(new_m);
                        }
                    });
                });
            });

        ui.add_space(12.0);

        // Header table
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("Total de Módulos no Sistema: {}", modules.len())).strong().color(theme.text_primary));
            ui.add_space(20.0);
            ui.label(RichText::new("(Aqui você pode gerenciar, auditar o status de rede ou apagar qualquer dispositivo do setup)").color(theme.text_muted));
        });

        ui.add_space(8.0);

        let mut to_remove: Option<usize> = None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (idx, module) in modules.iter_mut().enumerate() {
                egui::Frame::none()
                    .fill(if module.is_virtual { Color32::from_rgb(18, 22, 32) } else { theme.bg_card })
                    .stroke(Stroke::new(1.0_f32, if module.is_selected { theme.frame_yellow } else { Color32::from_rgb(35, 45, 60) }))
                    .rounding(Rounding::same(6.0))
                    .inner_margin(egui::Margin::symmetric(16.0, 10.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Badge status
                            let (badge_txt, badge_col) = if module.is_virtual {
                                ("VIRTUAL / OFFLINE", Color32::from_rgb(140, 160, 240))
                            } else if *is_online && module.is_online {
                                ("CONECTADO / ONLINE", theme.meter_green)
                            } else {
                                ("DESCONECTADO", theme.meter_yellow)
                            };

                            ui.label(RichText::new(format!("[{}]", badge_txt)).font(egui::FontId::monospace(10.5)).color(badge_col));
                            ui.add_space(10.0);

                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(&module.name).strong().color(theme.text_primary));
                                    ui.label(format!("(Módulo {}, {})", module.module_letter, module.module_type.display_name()));
                                    ui.label(RichText::new(format!("Frame: {}", module.frame_name)).color(theme.lime_accent));
                                });
                                ui.label(
                                    RichText::new(format!("IP: {}   |   Preset: {}   |   Entradas: {}   |   Saídas: {}",
                                        module.ip_address, module.preset_name, module.inputs.len(), module.outputs.len()
                                    ))
                                    .font(egui::FontId::monospace(11.0))
                                    .color(theme.text_muted),
                                );
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(RichText::new("🗑 APAGAR").color(theme.red_clip).strong()).clicked() {
                                    to_remove = Some(idx);
                                }

                                ui.add_space(10.0);

                                if ui.button("Abrir EQ / Filtros").clicked() {
                                    module.is_selected = true;
                                    *app_view = AppView::ModuleEditor;
                                }

                                ui.add_space(10.0);

                                if module.is_virtual {
                                    if ui.button("Parear com Físico").clicked() {
                                        module.is_virtual = false;
                                        module.is_online = true;
                                    }
                                }
                            });
                        });
                    });
                ui.add_space(6.0);
            }
        });

        if let Some(rm_idx) = to_remove {
            modules.remove(rm_idx);
        }
    });
}
