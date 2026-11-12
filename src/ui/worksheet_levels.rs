use crate::model::{DelayDisplayUnit, Module, ModuleEditorTab};
use crate::theme::LakeTheme;
use crate::ui::widgets::{draw_lake_pill_button, draw_segmented_meter};
use eframe::egui::{self, Color32, Pos2, Rect, RichText, Rounding, Sense, Stroke, Ui, Vec2};

pub fn render_worksheet_levels(
    ui: &mut Ui,
    theme: &LakeTheme,
    module: &mut Module,
    delay_unit: &mut DelayDisplayUnit,
    active_tab: &mut ModuleEditorTab,
) {
    ui.vertical(|ui| {
        // 1. Top Worksheet Navigation Tabs
        let tab_h = 32.0;
        let (tab_resp, painter) = ui.allocate_painter(Vec2::new(ui.available_width(), tab_h), Sense::hover());
        let tab_bounds = tab_resp.rect;

        painter.rect_filled(tab_bounds, 0.0, theme.bg_canvas);

        let ws_tabs = [
            ("EQ", ModuleEditorTab::SuperEq),
            ("Levels", ModuleEditorTab::LevelsDelay),
            ("I/O Config", ModuleEditorTab::IOConfig),
        ];

        let mut tx = tab_bounds.min.x + 8.0;
        for (label, tab) in &ws_tabs {
            let is_cur = match tab {
                ModuleEditorTab::LevelsDelay => *active_tab == ModuleEditorTab::LevelsDelay,
                ModuleEditorTab::SuperEq => *active_tab == ModuleEditorTab::SuperEq,
                ModuleEditorTab::IOConfig => *active_tab == ModuleEditorTab::IOConfig,
                _ => false,
            };

            let tw = 85.0;
            let trect = Rect::from_min_size(Pos2::new(tx, tab_bounds.min.y), Vec2::new(tw, tab_h));
            let tresp = ui.interact(trect, ui.make_persistent_id(format!("levels_tab_{}", label)), Sense::click());

            let tab_color = if is_cur { theme.frame_yellow } else { theme.frame_cyan };
            let r = 6.0;
            let tstroke = Stroke::new(1.8_f32, tab_color);

            painter.line_segment([Pos2::new(trect.min.x, trect.max.y), Pos2::new(trect.min.x, trect.min.y + r)], tstroke);
            painter.line_segment([Pos2::new(trect.min.x, trect.min.y + r), Pos2::new(trect.min.x + r, trect.min.y)], tstroke);
            painter.line_segment([Pos2::new(trect.min.x + r, trect.min.y), Pos2::new(trect.max.x - r, trect.min.y)], tstroke);
            painter.line_segment([Pos2::new(trect.max.x - r, trect.min.y), Pos2::new(trect.max.x, trect.min.y + r)], tstroke);
            painter.line_segment([Pos2::new(trect.max.x, trect.min.y + r), Pos2::new(trect.max.x, trect.max.y)], tstroke);

            painter.text(
                Pos2::new(trect.center().x, trect.center().y),
                egui::Align2::CENTER_CENTER,
                *label,
                egui::FontId::proportional(13.0),
                tab_color,
            );

            if tresp.clicked() {
                *active_tab = *tab;
            }

            tx += tw + 6.0;
        }

        // Module title
        painter.text(
            Pos2::new(tab_bounds.max.x - 42.0, tab_bounds.min.y + 12.0),
            egui::Align2::RIGHT_TOP,
            format!("{} - {}", module.frame_name, module.name),
            egui::FontId::proportional(14.0),
            theme.frame_yellow,
        );

        // Blue lake logo
        let badge_rect = Rect::from_min_size(Pos2::new(tab_bounds.max.x - 34.0, tab_bounds.min.y + 4.0), Vec2::splat(24.0));
        painter.rect_filled(badge_rect, Rounding::same(3.0), theme.lake_blue);
        painter.text(badge_rect.center(), egui::Align2::CENTER_CENTER, "lake", egui::FontId::proportional(11.0), Color32::WHITE);

        ui.add_space(8.0);

        // Delay unit options
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            ui.label(RichText::new("Delay Units:").color(theme.frame_cyan));
            ui.selectable_value(delay_unit, DelayDisplayUnit::Milliseconds, "ms");
            ui.selectable_value(delay_unit, DelayDisplayUnit::Meters, "m");
            ui.selectable_value(delay_unit, DelayDisplayUnit::Feet, "ft");
        });

        ui.add_space(10.0);

        // 2. Channel Fader Strips Container (Metallic Gray Slots)
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(12.0);

                // Input Channels
                for ch in &mut module.inputs {
                    render_metallic_channel_strip(ui, theme, ch, *delay_unit, false);
                    ui.add_space(12.0);
                }

                // Divider line
                let (d_rect, _d_resp) = ui.allocate_exact_size(Vec2::new(10.0, 320.0), Sense::hover());
                ui.painter().line_segment(
                    [Pos2::new(d_rect.center().x, d_rect.min.y), Pos2::new(d_rect.center().x, d_rect.max.y)],
                    Stroke::new(1.0_f32, theme.frame_yellow),
                );

                // Output Channels
                for ch in &mut module.outputs {
                    render_metallic_channel_strip(ui, theme, ch, *delay_unit, true);
                    ui.add_space(12.0);
                }
            });
        });
    });
}

fn render_metallic_channel_strip(
    ui: &mut Ui,
    theme: &LakeTheme,
    ch: &mut crate::model::ChannelStrip,
    _delay_unit: DelayDisplayUnit,
    _is_output: bool,
) {
    let strip_w = 78.0;
    let strip_h = 320.0;
    let (resp, painter) = ui.allocate_painter(Vec2::new(strip_w, strip_h), Sense::hover());
    let r = resp.rect;

    // 1. Inverted Cyan Readout Pill at Top: `[0.00] 0.00 dB`
    let pill_rect = Rect::from_min_size(Pos2::new(r.min.x + 4.0, r.min.y + 4.0), Vec2::new(strip_w - 8.0, 20.0));
    painter.rect_filled(pill_rect, Rounding::same(3.0), theme.lake_blue);
    painter.text(
        pill_rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("{:+.2} dB", ch.gain_db),
        egui::FontId::monospace(10.0),
        Color32::WHITE,
    );

    // 2. Metallic Gray Fader Slot Body
    let slot_w = 28.0;
    let slot_h = 210.0;
    let slot_rect = Rect::from_min_size(Pos2::new(r.min.x + 8.0, r.min.y + 32.0), Vec2::new(slot_w, slot_h));

    // Metallic gradient background
    painter.rect_filled(slot_rect, Rounding::same(3.0), Color32::from_rgb(115, 125, 135));
    painter.rect_stroke(slot_rect, Rounding::same(3.0), Stroke::new(1.0_f32, Color32::from_rgb(160, 170, 180)));

    // Center groove track
    let groove_x = slot_rect.center().x;
    painter.line_segment(
        [Pos2::new(groove_x, slot_rect.min.y + 6.0), Pos2::new(groove_x, slot_rect.max.y - 6.0)],
        Stroke::new(2.5_f32, Color32::from_rgb(30, 35, 40)),
    );

    // Tick marks on metallic body
    let ticks = [(12.0, "+12"), (0.0, "0"), (-10.0, "-10"), (-20.0, "-20"), (-40.0, "-40")];
    for (tdb, tlabel) in &ticks {
        let ty = slot_rect.max.y - 10.0 - ((tdb + 60.0) / 72.0) * (slot_h - 20.0);
        painter.line_segment([Pos2::new(slot_rect.min.x + 2.0, ty), Pos2::new(slot_rect.min.x + 6.0, ty)], Stroke::new(1.0_f32, Color32::BLACK));
        painter.text(Pos2::new(slot_rect.min.x + 2.0, ty), egui::Align2::RIGHT_CENTER, *tlabel, egui::FontId::monospace(7.0), theme.text_muted);
    }

    // 3D Cyan Oval Fader Cap
    let cap_y = slot_rect.max.y - 10.0 - ((ch.gain_db + 60.0) / 72.0) * (slot_h - 20.0);
    let cap_w = 34.0;
    let cap_h = 16.0;
    let cap_rect = Rect::from_center_size(Pos2::new(groove_x, cap_y), Vec2::new(cap_w, cap_h));

    let fader_id = ui.make_persistent_id(format!("fader_{}", ch.name));
    let fader_resp = ui.interact(cap_rect, fader_id, Sense::click_and_drag());
    if fader_resp.dragged() {
        let delta_db = -fader_resp.drag_delta().y * (72.0 / (slot_h - 20.0));
        ch.gain_db = (ch.gain_db + delta_db).clamp(-60.0, 12.0);
    }

    // Draw glossy cyan fader knob
    painter.rect_filled(cap_rect, Rounding::same(8.0), theme.btn_cyan_bg);
    painter.rect_stroke(cap_rect, Rounding::same(8.0), Stroke::new(1.0_f32, theme.btn_cyan_dark));
    // Center white notch line on knob
    painter.line_segment(
        [Pos2::new(cap_rect.min.x + 4.0, cap_rect.center().y), Pos2::new(cap_rect.max.x - 4.0, cap_rect.center().y)],
        Stroke::new(1.5_f32, Color32::WHITE),
    );

    // 3. Segmented LED Meter beside fader slot
    let meter_x = slot_rect.max.x + 6.0;
    let meter_rect = Rect::from_min_size(Pos2::new(meter_x, slot_rect.min.y), Vec2::new(9.0, slot_h));
    draw_segmented_meter(&painter, meter_rect, ch.meter_level, ch.peak_hold, ch.is_clip, ch.muted, theme);

    // 4. Bottom Controls: Phase +Φ and Mute Button
    let btn_y = slot_rect.max.y + 8.0;

    // Phase +Φ square button
    let phi_rect = Rect::from_min_size(Pos2::new(r.min.x + 4.0, btn_y), Vec2::new(26.0, 22.0));
    let phi_resp = ui.interact(phi_rect, ui.make_persistent_id(format!("phi_{}", ch.name)), Sense::click());
    if phi_resp.clicked() {
        ch.inverted_polarity = !ch.inverted_polarity;
    }
    let phi_label = if ch.inverted_polarity { "-Φ" } else { "+Φ" };
    draw_lake_pill_button(&painter, phi_rect, phi_label, None, ch.inverted_polarity, false, phi_resp.hovered(), theme);

    // Mute Button: Orange MUTED or Cyan UNMUTED
    let mute_rect = Rect::from_min_size(Pos2::new(phi_rect.max.x + 4.0, btn_y), Vec2::new(strip_w - 38.0, 22.0));
    let mute_resp = ui.interact(mute_rect, ui.make_persistent_id(format!("mute_ch_{}", ch.name)), Sense::click());
    if mute_resp.clicked() {
        ch.muted = !ch.muted;
    }

    let mute_label = if ch.muted { "MUTED" } else { "UNMUTE" };
    draw_lake_pill_button(
        &painter,
        mute_rect,
        mute_label,
        None,
        ch.muted,
        false,
        mute_resp.hovered(),
        theme,
    );

    // Channel label at very bottom
    painter.text(
        Pos2::new(r.center().x, btn_y + 32.0),
        egui::Align2::CENTER_CENTER,
        &ch.name,
        egui::FontId::monospace(10.0),
        theme.frame_cyan,
    );
}
