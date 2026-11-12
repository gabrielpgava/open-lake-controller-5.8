use crate::dsp::{generate_curve_points, GEQ_FREQUENCIES};
use crate::model::{FilterType, Module, ModuleEditorTab};
use crate::theme::LakeTheme;
use crate::ui::widgets::draw_lake_pill_button;
use eframe::egui::{self, Color32, Pos2, Rect, RichText, Rounding, Sense, Slider, Stroke, Ui, Vec2};

pub fn render_worksheet_eq(
    ui: &mut Ui,
    theme: &LakeTheme,
    module: &mut Module,
    selected_filter_idx: &mut usize,
    active_tab: &mut ModuleEditorTab,
) {
    ui.vertical(|ui| {
        // 1. Top Worksheet Tabs & Module Info Bar
        let tab_h = 32.0;
        let (tab_resp, painter) = ui.allocate_painter(Vec2::new(ui.available_width(), tab_h), Sense::hover());
        let tab_bounds = tab_resp.rect;

        // Black canvas
        painter.rect_filled(tab_bounds, 0.0, theme.bg_canvas);

        // Worksheet tabs: Levels, XOVER, PEQ1, GEQ2, I/O Config
        let ws_tabs = [
            ("Levels", ModuleEditorTab::LevelsDelay),
            ("XOVER", ModuleEditorTab::SuperEq),
            ("PEQ1", ModuleEditorTab::SuperEq),
            ("GEQ2", ModuleEditorTab::GraphicEq),
            ("I/O Config", ModuleEditorTab::IOConfig),
        ];

        let mut tx = tab_bounds.min.x + 8.0;
        for (label, tab) in &ws_tabs {
            let is_cur = match tab {
                ModuleEditorTab::LevelsDelay => *active_tab == ModuleEditorTab::LevelsDelay,
                ModuleEditorTab::SuperEq => *active_tab == ModuleEditorTab::SuperEq && (*label == "PEQ1" || *label == "XOVER"),
                ModuleEditorTab::GraphicEq => *active_tab == ModuleEditorTab::GraphicEq,
                ModuleEditorTab::IOConfig => *active_tab == ModuleEditorTab::IOConfig,
                _ => false,
            };

            let tw = 80.0;
            let trect = Rect::from_min_size(Pos2::new(tx, tab_bounds.min.y), Vec2::new(tw, tab_h));
            let tresp = ui.interact(trect, ui.make_persistent_id(format!("ws_tab_{}", label)), Sense::click());

            let tab_color = if is_cur { theme.frame_yellow } else { theme.frame_cyan };
            let r = 6.0;
            let tstroke = Stroke::new(1.8_f32, tab_color);

            // Tab contour
            painter.line_segment([Pos2::new(trect.min.x, trect.max.y), Pos2::new(trect.min.x, trect.min.y + r)], tstroke);
            painter.line_segment([Pos2::new(trect.min.x, trect.min.y + r), Pos2::new(trect.min.x + r, trect.min.y)], tstroke);
            painter.line_segment([Pos2::new(trect.min.x + r, trect.min.y), Pos2::new(trect.max.x - r, trect.min.y)], tstroke);
            painter.line_segment([Pos2::new(trect.max.x - r, trect.min.y), Pos2::new(trect.max.x, trect.min.y + r)], tstroke);
            painter.line_segment([Pos2::new(trect.max.x, trect.min.y + r), Pos2::new(trect.max.x, trect.max.y)], tstroke);

            painter.text(
                Pos2::new(trect.center().x - 4.0, trect.center().y),
                egui::Align2::CENTER_CENTER,
                *label,
                egui::FontId::proportional(13.0),
                tab_color,
            );

            // Small dot on PEQ / GEQ tabs
            if *label == "PEQ1" || *label == "GEQ2" {
                let dot_pos = Pos2::new(trect.center().x + 22.0, trect.center().y + 3.0);
                painter.rect_filled(Rect::from_center_size(dot_pos, Vec2::splat(3.0)), 0.0, tab_color);
            }

            if tresp.clicked() {
                *active_tab = *tab;
            }

            tx += tw + 6.0;
        }

        // Top-Right Module Breadcrumb
        let info_str = format!("{} - {} - {}", module.module_letter, module.frame_index, module.name);
        painter.text(
            Pos2::new(tab_bounds.max.x - 42.0, tab_bounds.min.y + 10.0),
            egui::Align2::RIGHT_TOP,
            info_str,
            egui::FontId::proportional(12.5),
            theme.frame_cyan,
        );
        painter.text(
            Pos2::new(tab_bounds.max.x - 42.0, tab_bounds.min.y + 24.0),
            egui::Align2::RIGHT_TOP,
            &module.preset_name,
            egui::FontId::proportional(11.0),
            theme.text_muted,
        );

        // Blue lake logo badge
        let badge_rect = Rect::from_min_size(Pos2::new(tab_bounds.max.x - 34.0, tab_bounds.min.y + 4.0), Vec2::splat(24.0));
        painter.rect_filled(badge_rect, Rounding::same(3.0), theme.lake_blue);
        painter.text(badge_rect.center(), egui::Align2::CENTER_CENTER, "lake", egui::FontId::proportional(11.0), Color32::WHITE);

        // 2. Utility Buttons: Frequency Lock [F11], A/B [F12]
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            let btn_w = 88.0;
            let btn_h = 28.0;

            let (f_rect, _) = ui.allocate_exact_size(Vec2::new(btn_w, btn_h), Sense::click());
            let f_resp = ui.interact(f_rect, ui.make_persistent_id("btn_freq_lock"), Sense::click());
            draw_lake_pill_button(ui.painter(), f_rect, "Frequency Lock", Some("F11"), false, false, f_resp.hovered(), theme);

            let (ab_rect, _) = ui.allocate_exact_size(Vec2::new(60.0, btn_h), Sense::click());
            let ab_resp = ui.interact(ab_rect, ui.make_persistent_id("btn_ab"), Sense::click());
            draw_lake_pill_button(ui.painter(), ab_rect, "A/B", Some("F12"), false, false, ab_resp.hovered(), theme);
        });

        ui.add_space(4.0);

        // 3. Main Lake Graphic EQ Display
        let graph_height = 290.0;
        let (graph_response, painter) =
            ui.allocate_painter(Vec2::new(ui.available_width(), graph_height), Sense::click_and_drag());
        let rect = graph_response.rect;

        // Pure pitch black background
        painter.rect_filled(rect, 0.0, theme.bg_canvas);

        // Logarithmic frequency coordinate transform (20 Hz to 32,000 Hz)
        let min_log = 20.0f32.log10();
        let max_log = 32000.0f32.log10();

        let freq_to_x = |f: f32| -> f32 {
            let log_f = f.max(20.0).min(32000.0).log10();
            let t = (log_f - min_log) / (max_log - min_log);
            rect.min.x + 40.0 + t * (rect.width() - 50.0)
        };

        let x_to_freq = |x: f32| -> f32 {
            let t = ((x - (rect.min.x + 40.0)) / (rect.width() - 50.0)).clamp(0.0, 1.0);
            let log_f = min_log + t * (max_log - min_log);
            10.0f32.powf(log_f)
        };

        // Linear dB coordinate transform (-15 dB to +15 dB)
        let min_db = -15.0f32;
        let max_db = 15.0f32;
        let graph_top = rect.min.y + 20.0;
        let graph_bot = rect.max.y - 30.0;
        let graph_h = graph_bot - graph_top;

        let db_to_y = |db: f32| -> f32 {
            let t = (db - min_db) / (max_db - min_db);
            graph_bot - t * graph_h
        };

        let y_to_db = |y: f32| -> f32 {
            let t = (graph_bot - y) / graph_h;
            min_db + t * (max_db - min_db)
        };

        // Horizontal dB grid lines: 15, 12, 9, 6, 3, 0, -3, -6, -9, -12, -15 dB
        let db_ticks = [15.0, 12.0, 9.0, 6.0, 3.0, 0.0, -3.0, -6.0, -9.0, -12.0, -15.0];
        for &db in &db_ticks {
            let y = db_to_y(db);
            let is_zero = db == 0.0;

            let stroke = if is_zero {
                Stroke::new(1.2_f32, Color32::from_rgb(180, 200, 220))
            } else {
                Stroke::new(0.6_f32, Color32::from_rgb(38, 48, 58))
            };

            painter.line_segment([Pos2::new(rect.min.x + 36.0, y), Pos2::new(rect.max.x - 8.0, y)], stroke);

            let label = format!("{:+.0}dB", db).replace("+0dB", "0dB");
            painter.text(
                Pos2::new(rect.min.x + 32.0, y),
                egui::Align2::RIGHT_CENTER,
                label,
                egui::FontId::monospace(9.5),
                if is_zero { Color32::WHITE } else { theme.text_muted },
            );
        }

        // Vertical Frequency Axis ticks: 20Hz, 31Hz, 62Hz, 125Hz, 250Hz, 500Hz, 1kHz, 2kHz, 4kHz, 8kHz, 16kHz, 32kHz
        let f_ticks = [
            (20.0, "20Hz"),
            (31.0, "31Hz"),
            (62.0, "62Hz"),
            (125.0, "125Hz"),
            (250.0, "250Hz"),
            (500.0, "500Hz"),
            (1000.0, "1kHz"),
            (2000.0, "2kHz"),
            (4000.0, "4kHz"),
            (8000.0, "8kHz"),
            (16000.0, "16kHz"),
            (32000.0, "32kHz"),
        ];

        for &(f, label) in &f_ticks {
            let x = freq_to_x(f);
            painter.line_segment(
                [Pos2::new(x, graph_top), Pos2::new(x, graph_bot + 4.0)],
                Stroke::new(0.6_f32, Color32::from_rgb(32, 42, 50)),
            );
            // Tick mark
            painter.line_segment(
                [Pos2::new(x, graph_bot), Pos2::new(x, graph_bot + 5.0)],
                Stroke::new(1.0_f32, Color32::from_rgb(150, 165, 180)),
            );
            painter.text(
                Pos2::new(x, graph_bot + 14.0),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::monospace(9.0),
                theme.frame_cyan,
            );
        }

        // Draw EQ Frequency Curve:
        // Translucent cyan fill underneath + white solid curve line
        let is_geq = *active_tab == ModuleEditorTab::GraphicEq;
        let curve_points = generate_curve_points(&module.peq_filters, &module.geq_bands, is_geq || module.geq_enabled, 140);
        let y_zero = db_to_y(0.0);

        // Fill polygon underneath the curve to 0 dB line
        let mut fill_mesh = egui::Mesh::default();
        let fill_color = theme.eq_fill_cyan;

        for w in curve_points.windows(2) {
            let p1_x = freq_to_x(w[0].0);
            let p1_y = db_to_y(w[0].1).clamp(graph_top, graph_bot);
            let p2_x = freq_to_x(w[1].0);
            let p2_y = db_to_y(w[1].1).clamp(graph_top, graph_bot);

            fill_mesh.add_triangle(
                fill_mesh.vertices.len() as u32,
                fill_mesh.vertices.len() as u32 + 1,
                fill_mesh.vertices.len() as u32 + 2,
            );
            fill_mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(p1_x, y_zero), uv: egui::epaint::WHITE_UV, color: fill_color });
            fill_mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(p1_x, p1_y), uv: egui::epaint::WHITE_UV, color: fill_color });
            fill_mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(p2_x, p2_y), uv: egui::epaint::WHITE_UV, color: fill_color });

            fill_mesh.add_triangle(
                fill_mesh.vertices.len() as u32 - 3,
                fill_mesh.vertices.len() as u32 - 1,
                fill_mesh.vertices.len() as u32,
            );
            fill_mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(p2_x, y_zero), uv: egui::epaint::WHITE_UV, color: fill_color });
        }
        painter.add(fill_mesh);

        // White curve outline
        for w in curve_points.windows(2) {
            let p1 = Pos2::new(freq_to_x(w[0].0), db_to_y(w[0].1).clamp(graph_top, graph_bot));
            let p2 = Pos2::new(freq_to_x(w[1].0), db_to_y(w[1].1).clamp(graph_top, graph_bot));
            painter.line_segment([p1, p2], Stroke::new(2.0_f32, theme.eq_curve_white));
        }

        // Draggable Filter Handles and Crosshair Markers
        if *selected_filter_idx >= module.peq_filters.len() {
            *selected_filter_idx = 0;
        }

        for (idx, filter) in module.peq_filters.iter_mut().enumerate() {
            let hx = freq_to_x(filter.freq);
            let hy = db_to_y(filter.gain).clamp(graph_top, graph_bot);
            let handle_pos = Pos2::new(hx, hy);
            let is_sel = idx == *selected_filter_idx;

            // Draw iconic yellow '×' crosshair marker at center frequency
            let marker_color = if is_sel { theme.eq_marker_yellow } else { Color32::from_rgb(220, 200, 40) };
            let ms = if is_sel { 5.0 } else { 3.5 };
            painter.line_segment([Pos2::new(hx - ms, hy - ms), Pos2::new(hx + ms, hy + ms)], Stroke::new(2.0_f32, marker_color));
            painter.line_segment([Pos2::new(hx - ms, hy + ms), Pos2::new(hx + ms, hy - ms)], Stroke::new(2.0_f32, marker_color));

            // Vertical line down from marker to axis if selected
            if is_sel {
                painter.line_segment(
                    [Pos2::new(hx, hy), Pos2::new(hx, graph_bot)],
                    Stroke::new(1.0_f32, theme.eq_marker_yellow.gamma_multiply(0.6)),
                );

                // Authentic Yellow Parameter Readout Box above filter
                let box_w = 54.0;
                let box_h = 38.0;
                let box_rect = Rect::from_center_size(
                    Pos2::new(hx.clamp(rect.min.x + 60.0, rect.max.x - 40.0), (hy - 32.0).max(graph_top + 20.0)),
                    Vec2::new(box_w, box_h),
                );

                painter.rect_filled(box_rect, 0.0, Color32::from_rgb(8, 12, 16));
                painter.rect_stroke(box_rect, 0.0, Stroke::new(1.2_f32, theme.frame_yellow));

                let freq_str = if filter.freq >= 1000.0 {
                    format!("{:.2}k", filter.freq / 1000.0)
                } else {
                    format!("{:.1}", filter.freq)
                };
                painter.text(Pos2::new(box_rect.center().x, box_rect.min.y + 7.0), egui::Align2::CENTER_CENTER, freq_str, egui::FontId::monospace(9.5), theme.frame_cyan);
                painter.text(Pos2::new(box_rect.center().x, box_rect.min.y + 19.0), egui::Align2::CENTER_CENTER, format!("{:.2}", filter.bandwidth), egui::FontId::monospace(9.5), theme.frame_cyan);
                painter.text(Pos2::new(box_rect.center().x, box_rect.min.y + 31.0), egui::Align2::CENTER_CENTER, format!("{:+.1}", filter.gain), egui::FontId::monospace(9.5), theme.frame_cyan);
            }

            // Handle drag interaction
            let handle_rect = Rect::from_center_size(handle_pos, Vec2::splat(18.0));
            let handle_id = ui.make_persistent_id(format!("eq_handle_{}_{}", module.id, idx));
            let handle_resp = ui.interact(handle_rect, handle_id, Sense::click_and_drag());

            if handle_resp.clicked() {
                *selected_filter_idx = idx;
            }

            if handle_resp.dragged() {
                *selected_filter_idx = idx;
                let new_x = handle_pos.x + handle_resp.drag_delta().x;
                let new_y = handle_pos.y + handle_resp.drag_delta().y;

                filter.freq = x_to_freq(new_x).clamp(20.0, 32000.0);
                filter.gain = y_to_db(new_y).clamp(-15.0, 15.0);
            }
        }

        ui.add_space(8.0);

        // 4. Filter Parameters Editing Bar
        if let Some(filter) = module.peq_filters.get_mut(*selected_filter_idx) {
            egui::Frame::none()
                .fill(Color32::from_rgb(6, 10, 14))
                .stroke(Stroke::new(1.0_f32, theme.frame_cyan))
                .rounding(Rounding::same(4.0))
                .inner_margin(egui::Margin::symmetric(14.0, 8.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("FILTER #{}:", *selected_filter_idx + 1))
                                .strong()
                                .font(egui::FontId::proportional(13.0))
                                .color(theme.frame_yellow),
                        );

                        ui.add_space(8.0);

                        egui::ComboBox::from_id_salt(format!("filter_type_sel_{}", *selected_filter_idx))
                            .selected_text(filter.filter_type.display_name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut filter.filter_type, FilterType::ParametricBell, "PEQ Bell");
                                ui.selectable_value(&mut filter.filter_type, FilterType::LowShelf, "Low Shelf");
                                ui.selectable_value(&mut filter.filter_type, FilterType::HighShelf, "High Shelf");
                                ui.selectable_value(&mut filter.filter_type, FilterType::HighPass, "HPF (High Pass)");
                                ui.selectable_value(&mut filter.filter_type, FilterType::LowPass, "LPF (Low Pass)");
                                ui.selectable_value(&mut filter.filter_type, FilterType::MesaFilter, "Mesa Raised-Cosine");
                            });

                        ui.add_space(10.0);

                        ui.label(RichText::new("Freq:").color(theme.frame_cyan));
                        ui.add(Slider::new(&mut filter.freq, 20.0..=32000.0).logarithmic(true).suffix(" Hz").max_decimals(1));

                        ui.add_space(10.0);

                        ui.label(RichText::new("Gain:").color(theme.frame_cyan));
                        ui.add(Slider::new(&mut filter.gain, -15.0..=15.0).suffix(" dB").max_decimals(1));

                        ui.add_space(10.0);

                        ui.label(RichText::new("BW:").color(theme.frame_cyan));
                        ui.add(Slider::new(&mut filter.bandwidth, 0.1..=10.0).suffix(" oct").max_decimals(2));

                        ui.add_space(12.0);

                        let bypass_str = if filter.enabled { "BYPASS" } else { "ENABLE" };
                        if ui.button(bypass_str).clicked() {
                            filter.enabled = !filter.enabled;
                        }

                        if ui.button("FLAT (0 dB)").clicked() {
                            filter.gain = 0.0;
                        }
                    });
                });
        }

        // 5. 28-Band Graphic Equalizer Faders (Shown when GEQ2 tab selected)
        if *active_tab == ModuleEditorTab::GraphicEq {
            ui.add_space(6.0);
            egui::Frame::none()
                .fill(Color32::from_rgb(6, 10, 14))
                .stroke(Stroke::new(1.0_f32, theme.frame_yellow))
                .rounding(Rounding::same(4.0))
                .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("28-BAND RAISED COSINE GRAPHIC EQUALIZER (GEQ)")
                                .font(egui::FontId::proportional(12.0))
                                .strong()
                                .color(theme.frame_yellow),
                        );
                        ui.add_space(10.0);
                        if ui.button("FLAT ALL BANDS").clicked() {
                            module.geq_bands = [0.0; 28];
                        }
                    });

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        for i in 0..28 {
                            let f = GEQ_FREQUENCIES[i];
                            let label = if f >= 1000.0 {
                                format!("{:.1}k", f / 1000.0)
                            } else {
                                format!("{:.0}", f)
                            };

                            ui.vertical(|ui| {
                                ui.add(
                                    Slider::new(&mut module.geq_bands[i], -12.0..=12.0)
                                        .vertical()
                                        .show_value(false),
                                );
                                ui.label(
                                    RichText::new(label)
                                        .font(egui::FontId::monospace(8.0))
                                        .color(theme.frame_cyan),
                                );
                            });
                        }
                    });
                });
        }
    });
}
