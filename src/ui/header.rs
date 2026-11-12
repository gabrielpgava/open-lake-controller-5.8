use crate::model::AppView;
use crate::theme::LakeTheme;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Sense, Stroke, Ui, Vec2};

pub fn render_header(
    ui: &mut Ui,
    theme: &LakeTheme,
    app_view: &mut AppView,
    _daylight_mode: &mut bool,
    windowed_mode: &mut bool,
    heartbeat: f32,
    active_page: &mut usize,
    pages: &[String],
    is_online: bool,
) {
    let header_height = 40.0;
    let (response, painter) = ui.allocate_painter(Vec2::new(ui.available_width(), header_height), Sense::hover());
    let bounds = response.rect;

    // Pitch black background
    painter.rect_filled(bounds, 0.0, theme.bg_canvas);

    // Continuous Frame Top Outline:
    // A yellow/gold line runs along the bottom of the tab strip into the screen frame.
    let baseline_y = bounds.max.y - 1.0;

    // Render Authentic Lake Tabs on top left
    let tab_h = 32.0;
    let tab_y = bounds.max.y - tab_h;

    let default_tabs = ["Main", "All"];
    let tab_count = if pages.is_empty() { default_tabs.len() } else { pages.len().max(2) };

    let mut current_tx = bounds.min.x + 12.0;

    for idx in 0..tab_count {
        let title = if idx == 0 {
            "Main"
        } else if idx == 1 {
            "All"
        } else {
            pages.get(idx).map(|s| s.as_str()).unwrap_or("Page")
        };

        let is_active = *active_page == idx && *app_view == AppView::HomeWorkarea;
        let tab_w = (title.len() as f32 * 8.5 + 32.0).max(75.0);
        let tab_rect = Rect::from_min_size(Pos2::new(current_tx, tab_y), Vec2::new(tab_w, tab_h));
        current_tx += tab_w + 6.0;

        let tab_id = ui.make_persistent_id(format!("lake_top_tab_{}", idx));
        let tab_resp = ui.interact(tab_rect, tab_id, Sense::click());

        if tab_resp.clicked() {
            *active_page = idx;
            *app_view = AppView::HomeWorkarea;
        }

        let tab_color = if is_active {
            theme.frame_yellow
        } else {
            theme.frame_cyan
        };

        // Draw folder-tab contour with rounded top corners
        let r = 7.0;
        let tab_stroke = Stroke::new(1.8_f32, tab_color);

        // Top arc / lines
        let p1 = Pos2::new(tab_rect.min.x, baseline_y);
        let p2 = Pos2::new(tab_rect.min.x, tab_rect.min.y + r);
        let p3 = Pos2::new(tab_rect.min.x + r, tab_rect.min.y);
        let p4 = Pos2::new(tab_rect.max.x - r, tab_rect.min.y);
        let p5 = Pos2::new(tab_rect.max.x, tab_rect.min.y + r);
        let p6 = Pos2::new(tab_rect.max.x, baseline_y);

        painter.line_segment([p1, p2], tab_stroke);
        painter.line_segment([p2, p3], tab_stroke);
        painter.line_segment([p3, p4], tab_stroke);
        painter.line_segment([p4, p5], tab_stroke);
        painter.line_segment([p5, p6], tab_stroke);

        // Tab label + dot
        painter.text(
            Pos2::new(tab_rect.center().x - 6.0, tab_rect.center().y - 1.0),
            egui::Align2::CENTER_CENTER,
            title,
            egui::FontId::proportional(15.0),
            tab_color,
        );

        // Small square dot beside title (authentic Lake icon)
        let dot_pos = Pos2::new(tab_rect.center().x + (title.len() as f32 * 4.2).min(tab_w * 0.5 - 8.0) + 4.0, tab_rect.center().y + 3.0);
        painter.rect_filled(Rect::from_center_size(dot_pos, Vec2::splat(3.0)), 0.0, tab_color);
    }

    // Top-Right:
    // 0. Beta / Real hardware notice
    let notice_text = "⚠ v0.0.1 (Não testado em equipamento real)";
    painter.text(
        Pos2::new(bounds.center().x + 60.0, bounds.center().y),
        egui::Align2::CENTER_CENTER,
        notice_text,
        egui::FontId::proportional(11.0),
        Color32::from_rgb(255, 180, 50),
    );

    // 1. Lake Logo Badge (Iconic blue square with white "dolby lake")
    let badge_size_w = 75.0;
    let badge_size_h = 24.0;
    let badge_rect = Rect::from_min_size(
        Pos2::new(bounds.max.x - badge_size_w - 14.0, bounds.min.y + (header_height - badge_size_h) * 0.5),
        Vec2::new(badge_size_w, badge_size_h),
    );
    painter.rect_filled(badge_rect, Rounding::same(4.0), theme.lake_blue);
    painter.rect_stroke(badge_rect, Rounding::same(4.0), Stroke::new(1.0_f32, Color32::from_rgb(80, 160, 240)));
    painter.text(
        badge_rect.center(),
        egui::Align2::CENTER_CENTER,
        "dolby lake",
        egui::FontId::proportional(11.5),
        Color32::WHITE,
    );

    // 2. Windowed Mode Button (Fulfills the user request for modern windowed stability while keeping authentic aesthetic)
    let win_btn_w = 95.0;
    let win_btn_h = 24.0;
    let win_btn_rect = Rect::from_min_size(
        Pos2::new(badge_rect.min.x - win_btn_w - 12.0, bounds.min.y + (header_height - win_btn_h) * 0.5),
        Vec2::new(win_btn_w, win_btn_h),
    );
    let win_resp = ui.interact(win_btn_rect, ui.make_persistent_id("lake_win_toggle"), Sense::click());
    let win_label = if *windowed_mode { "🗗 Windowed" } else { "🗖 Fullscreen" };

    let win_bg = if win_resp.hovered() {
        Color32::from_rgb(30, 45, 60)
    } else {
        Color32::from_rgb(15, 22, 30)
    };
    painter.rect_filled(win_btn_rect, Rounding::same(4.0), win_bg);
    painter.rect_stroke(win_btn_rect, Rounding::same(4.0), Stroke::new(1.0_f32, theme.frame_cyan));
    painter.text(
        win_btn_rect.center(),
        egui::Align2::CENTER_CENTER,
        win_label,
        egui::FontId::proportional(11.0),
        theme.frame_cyan,
    );

    if win_resp.clicked() {
        *windowed_mode = !*windowed_mode;
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Fullscreen(!*windowed_mode));
    }

    // 3. Telemetry & Heartbeat
    let (pulse_color, status_str) = if is_online {
        let pulse_alpha = ((heartbeat * 3.0).sin() * 0.4 + 0.6).clamp(0.2, 1.0);
        (theme.meter_green.gamma_multiply(pulse_alpha), "96kHz | Dante Sync | Online")
    } else {
        (Color32::from_rgb(190, 145, 25), "96kHz | Offline Mode | No Processor")
    };
    let led_center = Pos2::new(win_btn_rect.min.x - 14.0, bounds.center().y);
    painter.circle_filled(led_center, 4.0, pulse_color);

    painter.text(
        Pos2::new(led_center.x - 8.0, bounds.center().y),
        egui::Align2::RIGHT_CENTER,
        status_str,
        egui::FontId::monospace(10.5),
        theme.text_muted,
    );
}
