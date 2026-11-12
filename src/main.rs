mod dsp;
mod model;
mod theme;
mod ui;

use eframe::egui::{self, CentralPanel, TopBottomPanel};
use model::{AppView, BottomMenuContext, DelayDisplayUnit, Group, Module, ModuleEditorTab, ModuleType};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use theme::LakeTheme;

pub struct LakeControllerApp {
    modules: Vec<Module>,
    groups: Vec<Group>,
    pages: Vec<String>,
    active_page_idx: usize,
    selected_module_id: Option<usize>,
    selected_filter_idx: usize,
    app_view: AppView,
    active_tab: ModuleEditorTab,
    bottom_menu_context: BottomMenuContext,
    delay_unit: DelayDisplayUnit,
    daylight_mode: bool,
    windowed_mode: bool,
    is_online: bool,
    simulate_audio: bool,
    selected_adapter_idx: usize,
    network_adapters: Vec<crate::model::NetworkAdapterInfo>,
    start_time: Instant,
    frame_count: u32,
    screenshot_path: Option<PathBuf>,
    screenshot_requested: bool,
    screenshot_taken: bool,
}

impl LakeControllerApp {
    pub fn new(screenshot_path: Option<PathBuf>, daylight_mode: bool, windowed_mode: bool) -> Self {
        // Initial authentic Lake frames and module icons
        let mut m1 = Module::new(1, "CL3way", "DEMO", "192.168.1.101", ModuleType::LM26, (60.0, 50.0));
        m1.module_letter = 'A';
        m1.preset_name = "CL3way".to_string();
        m1.frame_index = 3;

        let mut m2 = Module::new(2, "CL3way", "DEMO", "192.168.1.101", ModuleType::LM26, (225.0, 50.0));
        m2.module_letter = 'B';
        m2.preset_name = "CL3way".to_string();
        m2.frame_index = 3;

        let mut m3 = Module::new(3, "Mesa EQ", "LM 44 Rack 1", "192.168.1.102", ModuleType::LM44, (410.0, 50.0));
        m3.module_letter = 'A';
        m3.preset_name = "Mesa EQ".to_string();
        m3.frame_index = 1;

        let mut m4 = Module::new(4, "Mesa EQ", "LM 44 Rack 1", "192.168.1.102", ModuleType::LM44, (575.0, 50.0));
        m4.module_letter = 'B';
        m4.preset_name = "Mesa EQ".to_string();
        m4.frame_index = 1;

        let mut m5 = Module::new(5, "Sub Array L", "PLM 20000Q", "192.168.1.103", ModuleType::PLM20000Q, (760.0, 50.0));
        m5.module_letter = 'A';
        m5.preset_name = "Sub Array L".to_string();
        m5.frame_index = 2;

        let mut m6 = Module::new(6, "Sub Array R", "PLM 20000Q", "192.168.1.103", ModuleType::PLM20000Q, (925.0, 50.0));
        m6.module_letter = 'B';
        m6.preset_name = "Sub Array R".to_string();
        m6.frame_index = 2;

        let modules = vec![m1, m2, m3, m4, m5, m6];

        let groups = vec![
            Group {
                id: 1,
                name: "Main PA".to_string(),
                color_rgb: [0, 210, 235],
                module_ids: vec![1, 2],
                muted: false,
                gain_offset_db: 0.0,
                delay_offset_ms: 0.0,
            },
            Group {
                id: 2,
                name: "Subwoofers".to_string(),
                color_rgb: [255, 155, 20],
                module_ids: vec![3, 4],
                muted: false,
                gain_offset_db: 0.0,
                delay_offset_ms: 0.0,
            },
            Group {
                id: 3,
                name: "Fills & Delays".to_string(),
                color_rgb: [60, 235, 75],
                module_ids: vec![5, 6],
                muted: false,
                gain_offset_db: 0.0,
                delay_offset_ms: 0.0,
            },
        ];

        let pages = vec![
            "FOH Main Arena".to_string(),
            "Delay Towers & VIP".to_string(),
            "Monitors & Stage".to_string(),
        ];

        let network_adapters = vec![
            crate::model::NetworkAdapterInfo {
                name: "Ethernet Principal (Placa de Rede Cabeada)".to_string(),
                ip: "192.168.0.25".to_string(),
                subnet: "255.255.255.0".to_string(),
                is_connected: true,
            },
            crate::model::NetworkAdapterInfo {
                name: "Dante Audio Network (Rede de Áudio Dedicada)".to_string(),
                ip: "169.254.56.150".to_string(),
                subnet: "255.255.0.0".to_string(),
                is_connected: true,
            },
            crate::model::NetworkAdapterInfo {
                name: "Rede Sem Fio / Wi-Fi".to_string(),
                ip: "192.168.0.105".to_string(),
                subnet: "255.255.255.0".to_string(),
                is_connected: false,
            },
            crate::model::NetworkAdapterInfo {
                name: "Loopback Local (127.0.0.1)".to_string(),
                ip: "127.0.0.1".to_string(),
                subnet: "255.0.0.0".to_string(),
                is_connected: true,
            },
        ];

        Self {
            modules,
            groups,
            pages,
            active_page_idx: 0,
            selected_module_id: Some(1),
            selected_filter_idx: 0,
            app_view: AppView::HomeWorkarea,
            active_tab: ModuleEditorTab::SuperEq,
            bottom_menu_context: BottomMenuContext::Home,
            delay_unit: DelayDisplayUnit::Milliseconds,
            daylight_mode,
            windowed_mode,
            is_online: false,
            simulate_audio: false, // Default false: meters stay at resting 0.0 when not connected
            selected_adapter_idx: 0,
            network_adapters,
            start_time: Instant::now(),
            frame_count: 0,
            screenshot_path,
            screenshot_requested: false,
            screenshot_taken: false,
        }
    }

    /// Simulate audio signal activity and meter dynamics when connected or explicitly requested
    fn update_simulated_audio(&mut self, elapsed_secs: f32) {
        if !self.simulate_audio && !self.is_online {
            // When disconnected / offline without simulation, keep meters in silent resting state (0.0)
            for module in self.modules.iter_mut() {
                for ch in module.inputs.iter_mut() {
                    ch.meter_level = 0.0;
                    ch.peak_hold = 0.0;
                    ch.is_clip = false;
                }
                for ch in module.outputs.iter_mut() {
                    ch.meter_level = 0.0;
                    ch.peak_hold = 0.0;
                    ch.is_clip = false;
                }
            }
            return;
        }

        for (m_idx, module) in self.modules.iter_mut().enumerate() {
            let offset = m_idx as f32 * 1.35;
            for (ch_idx, ch) in module.inputs.iter_mut().enumerate() {
                let wave = ((elapsed_secs * 4.0 + offset + ch_idx as f32 * 0.7).sin() * 0.5 + 0.5)
                    * ((elapsed_secs * 1.5 + offset).cos() * 0.3 + 0.7);
                let target_lvl = if ch.muted { 0.0 } else { (wave * 0.75 + 0.1).clamp(0.05, 0.98) };
                ch.meter_level = ch.meter_level * 0.7 + target_lvl * 0.3;
                ch.peak_hold = ch.peak_hold.max(ch.meter_level);
                ch.is_clip = ch.meter_level > 0.92;
            }

            for (ch_idx, ch) in module.outputs.iter_mut().enumerate() {
                let wave = ((elapsed_secs * 4.5 + offset * 1.2 + ch_idx as f32 * 0.5).sin() * 0.5 + 0.5)
                    * ((elapsed_secs * 2.1).cos() * 0.3 + 0.7);
                let target_lvl = if ch.muted { 0.0 } else { (wave * 0.8 + 0.08).clamp(0.05, 0.98) };
                ch.meter_level = ch.meter_level * 0.7 + target_lvl * 0.3;
                ch.peak_hold = ch.peak_hold.max(ch.meter_level);
                ch.is_clip = ch.meter_level > 0.92;
            }
        }
    }
}

impl eframe::App for LakeControllerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame_count += 1;
        let elapsed = self.start_time.elapsed().as_secs_f32();
        self.update_simulated_audio(elapsed);

        // Keep continuous animation rendering (60 FPS)
        ctx.request_repaint_after(Duration::from_millis(16));

        // Trigger screenshot after 120 frames of smooth rendering
        if self.frame_count >= 120 && !self.screenshot_requested && self.screenshot_path.is_some() {
            self.screenshot_requested = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
        }

        // Screenshot capture logic
        for event in &ctx.input(|i| i.raw.events.clone()) {
            if let egui::Event::Screenshot { image, .. } = event {
                if !self.screenshot_taken {
                    self.screenshot_taken = true;
                    if let Some(path) = &self.screenshot_path {
                        let width = image.width() as u32;
                        let height = image.height() as u32;
                        let mut rgba = Vec::with_capacity((width * height * 4) as usize);
                        for p in &image.pixels {
                            rgba.extend_from_slice(&p.to_array());
                        }
                        if let Some(parent) = path.parent() {
                            if !parent.as_os_str().is_empty() {
                                let _ = std::fs::create_dir_all(parent);
                            }
                        }
                        if let Some(img) = image::RgbaImage::from_raw(width, height, rgba) {
                            if let Err(e) = img.save(path) {
                                eprintln!("Failed to save screenshot: {}", e);
                            } else {
                                println!("Saved window screenshot to {:?} ({}x{})", path, width, height);
                                std::process::exit(0);
                            }
                        }
                    }
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }

        let theme = if self.daylight_mode {
            LakeTheme::daylight()
        } else {
            LakeTheme::dark()
        };

        // Top Navigation & Status Bar
        TopBottomPanel::top("lake_header")
            .frame(egui::Frame::none().fill(theme.bg_canvas))
            .show(ctx, |ui| {
                ui::header::render_header(
                    ui,
                    &theme,
                    &mut self.app_view,
                    &mut self.daylight_mode,
                    &mut self.windowed_mode,
                    elapsed,
                    &mut self.active_page_idx,
                    &self.pages,
                    self.is_online,
                );
            });

        // Bottom Pill Menu Bar
        TopBottomPanel::bottom("lake_bottom_bar")
            .frame(egui::Frame::none().fill(theme.bg_canvas))
            .show(ctx, |ui| {
                let sel_id = self.selected_module_id;
                let mut selected_mod = self.modules.iter_mut().find(|m| Some(m.id) == sel_id);

                ui::bottom_bar::render_bottom_bar(
                    ui,
                    &theme,
                    &mut self.app_view,
                    &mut self.active_tab,
                    &mut self.bottom_menu_context,
                    selected_mod.as_deref_mut(),
                    &mut self.daylight_mode,
                    &mut self.windowed_mode,
                );
            });

        // Main Center Panel
        CentralPanel::default()
            .frame(egui::Frame::none().fill(theme.bg_canvas))
            .show(ctx, |ui| {
                match self.app_view {
                    AppView::HomeWorkarea => {
                        ui::workarea::render_workarea(
                            ui,
                            &theme,
                            &mut self.modules,
                            &self.groups,
                            &mut self.selected_module_id,
                            &mut self.app_view,
                            self.active_page_idx,
                            &self.bottom_menu_context,
                        );
                    }
                    AppView::ModuleEditor => {
                        let sel_id = self.selected_module_id;
                        if let Some(module) = self.modules.iter_mut().find(|m| Some(m.id) == sel_id) {
                            match self.active_tab {
                                ModuleEditorTab::SuperEq | ModuleEditorTab::GraphicEq => {
                                    ui::worksheet_eq::render_worksheet_eq(
                                        ui,
                                        &theme,
                                        module,
                                        &mut self.selected_filter_idx,
                                        &mut self.active_tab,
                                    );
                                }
                                ModuleEditorTab::LevelsDelay | ModuleEditorTab::LimiterMax => {
                                    ui::worksheet_levels::render_worksheet_levels(
                                        ui,
                                        &theme,
                                        module,
                                        &mut self.delay_unit,
                                        &mut self.active_tab,
                                    );
                                }
                                ModuleEditorTab::IOConfig => {
                                    ui::worksheet_io::render_worksheet_io(ui, &theme, module);
                                }
                            }
                        } else {
                            ui.label("No module selected. Click Home to return to the workspace.");
                        }
                    }
                AppView::SoloMuteMatrix => {
                    ui::panels::render_solo_mute_matrix(
                        ui,
                        &theme,
                        &mut self.modules,
                        &mut self.groups,
                        &mut self.app_view,
                    );
                }
                AppView::SystemStoreRecall => {
                    ui::panels::render_store_recall(
                        ui,
                        &theme,
                        &mut self.modules,
                        &mut self.app_view,
                    );
                }
                AppView::UserPreferences => {
                    ui::panels::render_user_preferences(
                        ui,
                        &theme,
                        &mut self.windowed_mode,
                        &mut self.daylight_mode,
                        &mut self.app_view,
                    );
                }
                AppView::NetworkConfig => {
                    ui::panels::render_network_config(
                        ui,
                        &theme,
                        &self.network_adapters,
                        &mut self.selected_adapter_idx,
                        &mut self.is_online,
                        &mut self.simulate_audio,
                        &mut self.app_view,
                    );
                }
            }
        });
    }
}

fn main() {
    eprintln!(">>> ENTERED MAIN <<<");
    let args: Vec<String> = std::env::args().collect();
    let mut screenshot_path = None;
    let mut daylight_mode = false;
    let mut windowed_mode = true; // Default to windowed mode as requested by user

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--screenshot" || arg == "-screenshot" || arg == "/screenshot" {
            if i + 1 < args.len() {
                screenshot_path = Some(PathBuf::from(&args[i + 1]));
                i += 1;
            }
        } else if let Some(stripped) = arg.strip_prefix("--screenshot=") {
            screenshot_path = Some(PathBuf::from(stripped));
        } else if let Some(stripped) = arg.strip_prefix("-screenshot=") {
            screenshot_path = Some(PathBuf::from(stripped));
        } else if let Some(stripped) = arg.strip_prefix("/screenshot=") {
            screenshot_path = Some(PathBuf::from(stripped));
        } else if arg == "--fullscreen" || arg == "-fullscreen" {
            windowed_mode = false;
        } else if arg == "--windowed" || arg == "--nonfullscreen" || arg == "-nonfullscreen" {
            windowed_mode = true;
        } else if arg == "--daylight" || arg == "-daylight" {
            daylight_mode = true;
        }
        i += 1;
    }

    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Lake Controller v5.8 - Modern Windowed Audio Processor")
        .with_inner_size([1360.0, 840.0])
        .with_min_inner_size([1024.0, 680.0])
        .with_resizable(true);

    if screenshot_path.is_some() {
        viewport = viewport
            .with_active(false)
            .with_window_level(egui::WindowLevel::AlwaysOnBottom);
    } else if !windowed_mode {
        viewport = viewport.with_fullscreen(true);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    println!("Starting Lake Controller v5.8 Modern Windowed Edition...");
    let _ = eframe::run_native(
        "Lake Controller v5.8",
        native_options,
        Box::new(move |_cc| {
            Ok(Box::new(LakeControllerApp::new(
                screenshot_path,
                daylight_mode,
                windowed_mode,
            )))
        }),
    );
}
