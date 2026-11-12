use crate::model::{AppView, BottomMenuContext, Group, Module, ModuleType};
use crate::theme::LakeTheme;
use crate::ui::widgets::{draw_lake_sphere_disc, draw_segmented_meter};
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Sense, Stroke, Ui, Vec2};

pub fn render_workarea(
    ui: &mut Ui,
    theme: &LakeTheme,
    modules: &mut Vec<Module>,
    _groups: &[Group],
    selected_module_id: &mut Option<usize>,
    app_view: &mut AppView,
    _active_page_idx: usize,
    menu_context: &BottomMenuContext,
) {
    let (response, painter) =
        ui.allocate_painter(ui.available_size_before_wrap(), Sense::click_and_drag());
    let bounds = response.rect;

    // 1. Pitch Black Background
    painter.rect_filled(bounds, 0.0, theme.bg_canvas);

    // 2. Yellow Main Screen Contour (Iconic Lake golden yellow frame outline)
    let frame_pad = 8.0;
    let frame_rect = Rect::from_min_max(
        Pos2::new(bounds.min.x + frame_pad, bounds.min.y),
        Pos2::new(bounds.max.x - frame_pad, bounds.max.y - 2.0),
    );
    // Draw left, right and bottom borders (top is tab baseline)
    let frame_stroke = Stroke::new(1.8_f32, theme.frame_yellow);
    let r = 8.0;

    // Left vertical border
    painter.line_segment([Pos2::new(frame_rect.min.x, frame_rect.min.y), Pos2::new(frame_rect.min.x, frame_rect.max.y - r)], frame_stroke);
    // Left bottom corner
    painter.line_segment([Pos2::new(frame_rect.min.x, frame_rect.max.y - r), Pos2::new(frame_rect.min.x + r, frame_rect.max.y)], frame_stroke);
    // Bottom border
    painter.line_segment([Pos2::new(frame_rect.min.x + r, frame_rect.max.y), Pos2::new(frame_rect.max.x - r, frame_rect.max.y)], frame_stroke);
    // Right bottom corner
    painter.line_segment([Pos2::new(frame_rect.max.x - r, frame_rect.max.y), Pos2::new(frame_rect.max.x, frame_rect.max.y - r)], frame_stroke);
    // Right vertical border
    painter.line_segment([Pos2::new(frame_rect.max.x, frame_rect.max.y - r), Pos2::new(frame_rect.max.x, frame_rect.min.y)], frame_stroke);

    // Deselect if canvas background is clicked
    if response.clicked() {
        *selected_module_id = None;
        for m in modules.iter_mut() {
            m.is_selected = false;
        }
    }

    // Shelf height if "Modules" menu is open
    let shelf_h = if *menu_context == BottomMenuContext::Modules { 90.0 } else { 0.0 };
    let work_bounds = Rect::from_min_max(
        bounds.min,
        Pos2::new(bounds.max.x, bounds.max.y - shelf_h),
    );

    // 3. Render Modules as Authentic Lake Module Icons with Meters
    let icon_w = 145.0;
    let icon_h = 135.0;

    for module in modules.iter_mut() {
        let m_pos = Pos2::new(work_bounds.min.x + module.pos.0, work_bounds.min.y + module.pos.1);
        let m_rect = Rect::from_min_size(m_pos, Vec2::new(icon_w, icon_h));

        let m_id = ui.make_persistent_id(format!("mod_icon_{}", module.id));
        let m_resp = ui.interact(m_rect, m_id, Sense::click_and_drag());

        if m_resp.dragged() {
            module.pos.0 += m_resp.drag_delta().x;
            module.pos.1 += m_resp.drag_delta().y;
            module.pos.0 = module.pos.0.max(15.0).min(work_bounds.width() - icon_w - 15.0);
            module.pos.1 = module.pos.1.max(15.0).min(work_bounds.height() - icon_h - 15.0);
        }

        if m_resp.clicked() {
            *selected_module_id = Some(module.id);
        }

        if m_resp.double_clicked() {
            *selected_module_id = Some(module.id);
            *app_view = AppView::ModuleEditor;
        }

        let is_selected = *selected_module_id == Some(module.id);
        module.is_selected = is_selected;

        // Draw Module Icon Container:
        // Deep black interior with rounded border
        painter.rect_filled(m_rect, Rounding::same(10.0), Color32::from_rgb(4, 6, 8));

        let _border_color = if is_selected {
            theme.frame_yellow
        } else {
            theme.frame_cyan
        };
        let border_stroke = if is_selected {
            Stroke::new(2.2_f32, theme.frame_yellow)
        } else {
            Stroke::new(1.4_f32, theme.frame_cyan)
        };
        painter.rect_stroke(m_rect, Rounding::same(10.0), border_stroke);

        // 3D Glossy Sphere Disc
        let disc_radius = 29.0;
        let disc_center = Pos2::new(m_rect.min.x + 42.0, m_rect.min.y + 44.0);

        let (disc_color, model_text, sub_text) = match module.module_type {
            ModuleType::Contour | ModuleType::LM26 => (
                theme.disc_contour_coral,
                "LM 26",
                "3",
            ),
            ModuleType::LM44 | ModuleType::MesaEq => (
                theme.disc_mesa_blue,
                "LM 44",
                "MEq",
            ),
            ModuleType::PLM10000Q | ModuleType::PLM20000Q => (
                theme.disc_plm_amber,
                "PLM 20k",
                "2",
            ),
        };

        draw_lake_sphere_disc(
            &painter,
            disc_center,
            disc_radius,
            module.module_letter,
            model_text,
            sub_text,
            disc_color,
        );

        // Segmented LED Meters next to sphere (Input 1 & Output 1-2)
        let meter_start_x = m_rect.min.x + 82.0;
        let meter_y = m_rect.min.y + 14.0;
        let meter_w = 7.0;
        let meter_h = 62.0;

        // Input 1 meter
        if let Some(in1) = module.inputs.first() {
            let m_rect_in = Rect::from_min_size(Pos2::new(meter_start_x, meter_y), Vec2::new(meter_w, meter_h));
            draw_segmented_meter(&painter, m_rect_in, in1.meter_level, in1.peak_hold, in1.is_clip, in1.muted, theme);
        }

        // Output 1 & 2 meters
        if let Some(out1) = module.outputs.first() {
            let m_rect_out1 = Rect::from_min_size(Pos2::new(meter_start_x + meter_w + 3.0, meter_y), Vec2::new(meter_w, meter_h));
            draw_segmented_meter(&painter, m_rect_out1, out1.meter_level, out1.peak_hold, out1.is_clip, out1.muted, theme);
        }
        if let Some(out2) = module.outputs.get(1) {
            let m_rect_out2 = Rect::from_min_size(Pos2::new(meter_start_x + (meter_w + 3.0) * 2.0, meter_y), Vec2::new(meter_w, meter_h));
            draw_segmented_meter(&painter, m_rect_out2, out2.meter_level, out2.peak_hold, out2.is_clip, out2.muted, theme);
        }

        // Labels under meters
        painter.text(
            Pos2::new(meter_start_x + 3.0, meter_y + meter_h + 3.0),
            egui::Align2::CENTER_TOP,
            "IN",
            egui::FontId::monospace(8.0),
            theme.lime_accent,
        );
        painter.text(
            Pos2::new(meter_start_x + meter_w + 12.0, meter_y + meter_h + 3.0),
            egui::Align2::CENTER_TOP,
            "OUT",
            egui::FontId::monospace(8.0),
            theme.cyan_accent,
        );

        // Module Preset Label below sphere (e.g. "CL3way", "Main FOH L")
        let label_y = m_rect.min.y + 92.0;
        painter.text(
            Pos2::new(m_rect.center().x, label_y),
            egui::Align2::CENTER_CENTER,
            &module.name,
            egui::FontId::proportional(12.5),
            theme.frame_cyan,
        );

        // Authentic orange/coral underline indicator
        painter.line_segment(
            [Pos2::new(m_rect.center().x - 18.0, label_y + 9.0), Pos2::new(m_rect.center().x + 18.0, label_y + 9.0)],
            Stroke::new(2.2_f32, Color32::from_rgb(255, 90, 20)),
        );

        // Quick EQ / Levels mini-button at bottom
        let eq_btn_rect = Rect::from_min_size(
            Pos2::new(m_rect.min.x + 12.0, m_rect.max.y - 23.0),
            Vec2::new(m_rect.width() - 24.0, 18.0),
        );
        let eq_btn_resp = ui.interact(eq_btn_rect, ui.make_persistent_id(format!("open_eq_{}", module.id)), Sense::click());

        let eq_btn_bg = if eq_btn_resp.hovered() {
            theme.btn_cyan_bg
        } else {
            Color32::from_rgb(12, 18, 25)
        };
        let eq_btn_text = if eq_btn_resp.hovered() {
            Color32::BLACK
        } else {
            theme.frame_cyan
        };

        painter.rect_filled(eq_btn_rect, Rounding::same(9.0), eq_btn_bg);
        painter.rect_stroke(eq_btn_rect, Rounding::same(9.0), Stroke::new(1.0_f32, theme.frame_cyan));
        painter.text(
            eq_btn_rect.center(),
            egui::Align2::CENTER_CENTER,
            "EQ / LEVELS",
            egui::FontId::monospace(9.0),
            eq_btn_text,
        );

        if eq_btn_resp.clicked() {
            *selected_module_id = Some(module.id);
            *app_view = AppView::ModuleEditor;
        }
    }

    // 4. If "Modules" (F3) is active, draw the authentic module shelf at the bottom
    let mut add_module_request: Option<(ModuleType, &'static str, &'static str, f32)> = None;

    if *menu_context == BottomMenuContext::Modules {
        let shelf_rect = Rect::from_min_max(
            Pos2::new(bounds.min.x + 10.0, bounds.max.y - shelf_h),
            Pos2::new(bounds.max.x - 10.0, bounds.max.y - 4.0),
        );

        painter.rect_filled(shelf_rect, Rounding::same(6.0), Color32::from_rgb(8, 12, 16));
        painter.rect_stroke(shelf_rect, Rounding::same(6.0), Stroke::new(1.2_f32, theme.frame_yellow));

        let shelf_items: [(ModuleType, &'static str, &'static str, Color32); 6] = [
            (ModuleType::LM44, "Virtual LM 44", "4x Mesa EQ\nModules", theme.disc_mesa_blue),
            (ModuleType::LM26, "Virtual LM 26", "2x Contour\nModules", theme.disc_contour_coral),
            (ModuleType::Contour, "Virtual Contour", "Classic Loudspeaker\nProcessor", theme.disc_contour_coral),
            (ModuleType::PLM20000Q, "Virtual PLM 20000Q", "4-Ch Touring\nAmplifier", theme.disc_plm_amber),
            (ModuleType::PLM10000Q, "Virtual PLM 10000Q", "4-Ch Powered\nProcessor", theme.disc_plm_amber),
            (ModuleType::MesaEq, "Virtual Mesa EQ", "Independent EQ\nEngine", theme.disc_mesa_blue),
        ];

        let slot_w = 122.0;
        let slot_h = 74.0;
        let slot_y = shelf_rect.min.y + 8.0;

        for (i, (m_type, title, sub, color)) in shelf_items.iter().enumerate() {
            let slot_x = shelf_rect.min.x + 12.0 + (i as f32) * (slot_w + 10.0);
            if slot_x + slot_w > shelf_rect.max.x {
                break;
            }
            let s_rect = Rect::from_min_size(Pos2::new(slot_x, slot_y), Vec2::new(slot_w, slot_h));
            let s_resp = ui.interact(s_rect, ui.make_persistent_id(format!("shelf_mod_{}", i)), Sense::click());

            let is_s_hovered = s_resp.hovered();
            let s_bg = if is_s_hovered { Color32::from_rgb(28, 38, 52) } else { Color32::from_rgb(18, 24, 32) };
            painter.rect_filled(s_rect, Rounding::same(4.0), s_bg);
            painter.rect_stroke(s_rect, Rounding::same(4.0), Stroke::new(1.0_f32, if is_s_hovered { theme.frame_yellow } else { theme.frame_cyan }));

            // Thumbnail mini sphere
            draw_lake_sphere_disc(
                &painter,
                Pos2::new(s_rect.min.x + 20.0, s_rect.center().y),
                15.0,
                'A',
                "",
                "",
                *color,
            );

            // Title & sub-text
            painter.text(
                Pos2::new(s_rect.min.x + 40.0, s_rect.min.y + 16.0),
                egui::Align2::LEFT_CENTER,
                *title,
                egui::FontId::proportional(11.0),
                theme.prompt_yellow,
            );

            let sub_lines: Vec<&str> = sub.split('\n').collect();
            for (li, line) in sub_lines.iter().enumerate() {
                painter.text(
                    Pos2::new(s_rect.min.x + 40.0, s_rect.min.y + 36.0 + (li as f32) * 12.0),
                    egui::Align2::LEFT_CENTER,
                    *line,
                    egui::FontId::monospace(8.5),
                    theme.text_muted,
                );
            }

            if s_resp.clicked() {
                add_module_request = Some((*m_type, title, "Virtual Rack", slot_x - bounds.min.x));
            }
        }
    }

    if let Some((m_type, title, frame, start_x)) = add_module_request {
        let new_id = modules.iter().map(|m| m.id).max().unwrap_or(0) + 1;
        let mut new_m = Module::new(
            new_id,
            title,
            frame,
            "192.168.1.150",
            m_type,
            (start_x.max(30.0).min(work_bounds.width() - 160.0), 70.0),
        );
        new_m.is_virtual = true;
        new_m.is_online = false;
        *selected_module_id = Some(new_id);
        modules.push(new_m);
    }
}
