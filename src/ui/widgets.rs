use crate::theme::LakeTheme;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};

/// Draw an authentic Lake Controller 3D glossy pill button
pub fn draw_lake_pill_button(
    painter: &egui::Painter,
    rect: Rect,
    label: &str,
    fkey: Option<&str>,
    is_active: bool,
    is_disabled: bool,
    is_hovered: bool,
    theme: &LakeTheme,
) {
    let r = rect.height() * 0.5;

    // Base color
    let (base_color, dark_rim, text_color) = if is_disabled {
        (theme.btn_gray_bg, theme.btn_gray_dark, theme.btn_gray_text)
    } else if is_active {
        (theme.btn_amber_bg, theme.btn_amber_dark, theme.btn_amber_text)
    } else {
        (theme.btn_cyan_bg, theme.btn_cyan_dark, theme.btn_cyan_text)
    };

    let fill_color = if is_hovered && !is_disabled {
        base_color.gamma_multiply(1.15)
    } else {
        base_color
    };

    // 1. Dark bottom shadow / outer bevel
    painter.rect_filled(
        Rect::from_min_max(Pos2::new(rect.min.x, rect.min.y + 1.0), Pos2::new(rect.max.x, rect.max.y + 1.5)),
        Rounding::same(r),
        Color32::from_black_alpha(100),
    );

    // 2. Base body fill
    painter.rect_filled(rect, Rounding::same(r), fill_color);

    // 3. 3D Gel Top Highlight (upper 45%)
    let highlight_h = (rect.height() * 0.44).max(6.0);
    let highlight_rect = Rect::from_min_max(
        Pos2::new(rect.min.x + 3.0, rect.min.y + 2.0),
        Pos2::new(rect.max.x - 3.0, rect.min.y + highlight_h),
    );
    let highlight_color = if is_active {
        Color32::from_rgba_unmultiplied(255, 255, 230, 110)
    } else if is_disabled {
        Color32::from_rgba_unmultiplied(255, 255, 255, 60)
    } else {
        Color32::from_rgba_unmultiplied(255, 255, 255, 120)
    };
    painter.rect_filled(highlight_rect, Rounding::same(r * 0.7), highlight_color);

    // 4. Subtle inner bottom shadow
    let shadow_rect = Rect::from_min_max(
        Pos2::new(rect.min.x + 4.0, rect.max.y - 6.0),
        Pos2::new(rect.max.x - 4.0, rect.max.y - 2.0),
    );
    painter.rect_filled(shadow_rect, Rounding::same(2.0), Color32::from_black_alpha(35));

    // 5. Border stroke
    painter.rect_stroke(rect, Rounding::same(r), Stroke::new(1.2_f32, dark_rim));

    // 6. Label (multi-line supported)
    let lines: Vec<&str> = label.split('\n').collect();
    if lines.len() == 1 {
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::proportional(12.0),
            text_color,
        );
    } else {
        let line_spacing = 13.0;
        let total_h = (lines.len() as f32 - 1.0) * line_spacing;
        let start_y = rect.center().y - total_h * 0.5;
        for (i, line) in lines.iter().enumerate() {
            painter.text(
                Pos2::new(rect.center().x, start_y + (i as f32) * line_spacing),
                egui::Align2::CENTER_CENTER,
                *line,
                egui::FontId::proportional(11.0),
                text_color,
            );
        }
    }

    // 7. F-key shortcut in bottom-right corner
    if let Some(fk) = fkey {
        let fk_color = text_color.gamma_multiply(0.7);
        painter.text(
            Pos2::new(rect.max.x - 7.0, rect.max.y - 4.0),
            egui::Align2::RIGHT_BOTTOM,
            fk,
            egui::FontId::monospace(8.5),
            fk_color,
        );
    }
}

/// Draw an authentic Lake Controller 3D glossy circular sphere / disc
pub fn draw_lake_sphere_disc(
    painter: &egui::Painter,
    center: Pos2,
    radius: f32,
    letter: char,
    model_name: &str,
    sub_label: &str,
    disc_color: Color32,
) {
    // Drop shadow
    painter.circle_filled(
        Pos2::new(center.x + 2.0, center.y + 2.5),
        radius,
        Color32::from_black_alpha(140),
    );

    // Dark outer bevel ring
    painter.circle_filled(center, radius, Color32::from_black_alpha(80));

    // Base colored circle
    painter.circle_filled(center, radius - 1.0, disc_color);

    // 3D Spherical Specular Highlight (offset up-left)
    let hl_center = Pos2::new(center.x - radius * 0.32, center.y - radius * 0.32);
    painter.circle_filled(
        hl_center,
        radius * 0.55,
        Color32::from_white_alpha(70),
    );
    painter.circle_filled(
        Pos2::new(center.x - radius * 0.35, center.y - radius * 0.35),
        radius * 0.32,
        Color32::from_white_alpha(140),
    );
    painter.circle_filled(
        Pos2::new(center.x - radius * 0.37, center.y - radius * 0.37),
        radius * 0.14,
        Color32::from_white_alpha(230),
    );

    // Subtle dark rim at bottom-right
    let br_shadow = Pos2::new(center.x + radius * 0.28, center.y + radius * 0.28);
    painter.circle_filled(
        br_shadow,
        radius * 0.45,
        Color32::from_black_alpha(40),
    );

    // Outer stroke ring
    painter.circle_stroke(center, radius - 0.5, Stroke::new(1.0_f32, Color32::from_white_alpha(60)));

    // Text inside sphere:
    // Large bold letter (A, B, C, D)
    let letter_y = center.y - radius * 0.35;
    painter.text(
        Pos2::new(center.x, letter_y),
        egui::Align2::CENTER_CENTER,
        letter.to_string(),
        egui::FontId::proportional(radius * 0.65),
        Color32::BLACK,
    );

    // Model name
    let model_y = center.y + radius * 0.12;
    painter.text(
        Pos2::new(center.x, model_y),
        egui::Align2::CENTER_CENTER,
        model_name,
        egui::FontId::proportional(radius * 0.38),
        Color32::BLACK,
    );

    // Sub-label (Frame number or MEq)
    let sub_y = center.y + radius * 0.52;
    painter.text(
        Pos2::new(center.x, sub_y),
        egui::Align2::CENTER_CENTER,
        sub_label,
        egui::FontId::proportional(radius * 0.34),
        Color32::BLACK,
    );
}

/// Draw segmented LED bargraph meter
pub fn draw_segmented_meter(
    painter: &egui::Painter,
    rect: Rect,
    level: f32,      // 0.0 to 1.0
    peak_hold: f32,  // 0.0 to 1.0
    clip: bool,
    muted: bool,
    theme: &LakeTheme,
) {
    // Background dark slot
    painter.rect_filled(rect, Rounding::same(1.5), Color32::from_rgb(10, 14, 18));
    painter.rect_stroke(rect, Rounding::same(1.5), Stroke::new(0.8_f32, Color32::from_rgb(35, 45, 55)));

    if muted {
        // Red slashed indicator
        painter.line_segment([rect.left_top(), rect.right_bottom()], Stroke::new(1.2_f32, theme.meter_red));
        return;
    }

    let num_segments = 14;
    let seg_spacing = 1.2;
    let seg_h = (rect.height() - (num_segments as f32 + 1.0) * seg_spacing) / num_segments as f32;
    let seg_w = rect.width() - 2.0;
    let seg_x = rect.min.x + 1.0;

    let active_segments = ((level.clamp(0.0, 1.0) * num_segments as f32).round() as usize).min(num_segments);
    let peak_segment = ((peak_hold.clamp(0.0, 1.0) * num_segments as f32).round() as usize).min(num_segments);

    for s in 0..num_segments {
        let seg_idx_from_bottom = s; // 0 is bottom, num_segments - 1 is top
        let seg_y = rect.max.y - seg_spacing - (s as f32 + 1.0) * (seg_h + seg_spacing);
        let seg_rect = Rect::from_min_size(Pos2::new(seg_x, seg_y), Vec2::new(seg_w, seg_h));

        let is_lit = seg_idx_from_bottom < active_segments;
        let is_peak = seg_idx_from_bottom + 1 == peak_segment;

        // Color coding by height
        let (lit_color, off_color) = if seg_idx_from_bottom >= num_segments - 2 {
            // Top 2 segments are RED (Clip / > -2 dB)
            (theme.meter_red, Color32::from_rgb(38, 8, 8))
        } else if seg_idx_from_bottom >= num_segments - 5 {
            // Next 3 segments are YELLOW (-12 to -2 dB)
            (theme.meter_yellow, Color32::from_rgb(32, 26, 6))
        } else {
            // Lower segments are GREEN (-60 to -12 dB)
            (theme.meter_green, Color32::from_rgb(8, 28, 12))
        };

        let color = if is_lit {
            lit_color
        } else if is_peak {
            lit_color
        } else {
            off_color
        };

        painter.rect_filled(seg_rect, Rounding::same(0.5), color);
    }

    // Top clip LED square
    if clip {
        let clip_rect = Rect::from_min_size(Pos2::new(rect.min.x - 1.0, rect.min.y - 5.0), Vec2::new(rect.width() + 2.0, 3.5));
        painter.rect_filled(clip_rect, Rounding::same(1.0), theme.meter_red);
    }
}
