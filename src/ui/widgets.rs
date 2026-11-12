use crate::theme::LakeTheme;
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};

/// Draw an authentic Lake Controller 3D glossy pill button with high-end tactile finish
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

    // Palette selection with refined high-contrast tactile tone
    let (base_top, base_bottom, rim_stroke, text_color) = if is_disabled {
        (
            Color32::from_rgb(95, 105, 115),
            Color32::from_rgb(60, 68, 76),
            Color32::from_rgb(45, 52, 60),
            theme.btn_gray_text,
        )
    } else if is_active {
        (
            Color32::from_rgb(255, 185, 45),
            Color32::from_rgb(215, 125, 10),
            Color32::from_rgb(145, 75, 0),
            Color32::from_rgb(25, 12, 0),
        )
    } else {
        (
            Color32::from_rgb(72, 192, 245),
            Color32::from_rgb(28, 125, 195),
            Color32::from_rgb(12, 70, 120),
            Color32::from_rgb(0, 28, 55),
        )
    };

    // Responsive interaction shift (push-in feel when active/hovered)
    let (draw_rect, fill_top, fill_bottom) = if is_disabled {
        (rect, base_top, base_bottom)
    } else if is_active {
        // Inverted gradient when pushed / active
        (rect, base_bottom, base_top)
    } else if is_hovered {
        (
            Rect::from_min_max(Pos2::new(rect.min.x, rect.min.y - 0.5), Pos2::new(rect.max.x, rect.max.y - 0.5)),
            base_top.gamma_multiply(1.18),
            base_bottom.gamma_multiply(1.12),
        )
    } else {
        (rect, base_top, base_bottom)
    };

    // 1. Soft ambient outer drop shadow for depth
    painter.rect_filled(
        Rect::from_min_max(
            Pos2::new(rect.min.x + 0.5, rect.min.y + 1.5),
            Pos2::new(rect.max.x + 0.5, rect.max.y + 2.5),
        ),
        Rounding::same(r),
        Color32::from_black_alpha(110),
    );

    // 2. Button chassis background (dark inset bezel)
    painter.rect_filled(
        draw_rect,
        Rounding::same(r),
        rim_stroke,
    );

    // 3. Inner body pill fill
    let inner_rect = Rect::from_min_max(
        Pos2::new(draw_rect.min.x + 1.2, draw_rect.min.y + 1.2),
        Pos2::new(draw_rect.max.x - 1.2, draw_rect.max.y - 1.2),
    );
    let inner_r = (inner_rect.height() * 0.5).max(1.0);
    painter.rect_filled(inner_rect, Rounding::same(inner_r), fill_top);

    // 4. Subtle lower half gradient tint
    let lower_half = Rect::from_min_max(
        Pos2::new(inner_rect.min.x, inner_rect.center().y),
        Pos2::new(inner_rect.max.x, inner_rect.max.y),
    );
    painter.rect_filled(lower_half, Rounding::same(inner_r), fill_bottom);

    // 5. 3D Gel Top Gloss Highlight (elliptical tactile reflection)
    if !is_disabled {
        let highlight_h = (inner_rect.height() * 0.42).clamp(4.0, 18.0);
        let highlight_rect = Rect::from_min_max(
            Pos2::new(inner_rect.min.x + 2.5, inner_rect.min.y + 1.5),
            Pos2::new(inner_rect.max.x - 2.5, inner_rect.min.y + highlight_h),
        );
        let gloss_color = if is_active {
            Color32::from_rgba_unmultiplied(255, 255, 240, 95)
        } else {
            Color32::from_rgba_unmultiplied(255, 255, 255, 140)
        };
        painter.rect_filled(highlight_rect, Rounding::same(highlight_h * 0.5), gloss_color);
    }

    // 6. Precise rim stroke for hardware definition
    let border_color = if is_hovered && !is_disabled {
        Color32::from_white_alpha(160)
    } else {
        rim_stroke
    };
    painter.rect_stroke(draw_rect, Rounding::same(r), Stroke::new(1.3_f32, border_color));

    // 7. Label rendering (bold, crisp and centered)
    let lines: Vec<&str> = label.split('\n').collect();
    if lines.len() == 1 {
        painter.text(
            draw_rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::proportional(12.2),
            text_color,
        );
    } else {
        let line_spacing = 13.0;
        let total_h = (lines.len() as f32 - 1.0) * line_spacing;
        let start_y = draw_rect.center().y - total_h * 0.5;
        for (i, line) in lines.iter().enumerate() {
            painter.text(
                Pos2::new(draw_rect.center().x, start_y + (i as f32) * line_spacing),
                egui::Align2::CENTER_CENTER,
                *line,
                egui::FontId::proportional(11.2),
                text_color,
            );
        }
    }

    // 8. F-key tactile indicator in bottom-right corner
    if let Some(fk) = fkey {
        let fk_color = text_color.gamma_multiply(0.72);
        painter.text(
            Pos2::new(draw_rect.max.x - 7.0, draw_rect.max.y - 3.5),
            egui::Align2::RIGHT_BOTTOM,
            fk,
            egui::FontId::monospace(8.8),
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
