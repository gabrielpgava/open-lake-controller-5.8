use eframe::egui::{Color32, Stroke};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct LakeTheme {
    pub is_daylight: bool,
    pub bg_canvas: Color32,
    pub frame_yellow: Color32,
    pub frame_cyan: Color32,
    pub lake_blue: Color32,
    pub btn_cyan_bg: Color32,
    pub btn_cyan_dark: Color32,
    pub btn_cyan_text: Color32,
    pub btn_amber_bg: Color32,
    pub btn_amber_dark: Color32,
    pub btn_amber_text: Color32,
    pub btn_gray_bg: Color32,
    pub btn_gray_dark: Color32,
    pub btn_gray_text: Color32,
    pub disc_contour_coral: Color32,
    pub disc_mesa_blue: Color32,
    pub disc_plm_amber: Color32,
    pub meter_green: Color32,
    pub meter_yellow: Color32,
    pub meter_red: Color32,
    pub meter_off: Color32,
    pub prompt_text: Color32,
    pub prompt_yellow: Color32,
    pub eq_curve_white: Color32,
    pub eq_fill_cyan: Color32,
    pub eq_marker_yellow: Color32,
    pub eq_marker_red: Color32,
    pub text_primary: Color32,
    pub text_muted: Color32,
    pub cyan_accent: Color32,
    pub lime_accent: Color32,
    pub orange_accent: Color32,
    pub red_clip: Color32,
    pub yellow_warn: Color32,
    pub pill_btn_bg: Color32,
    pub pill_btn_border: Color32,
    pub grid_line: Color32,
    pub bg_dark: Color32,
    pub bg_card: Color32,
    pub bg_card_selected: Color32,
    pub border: Color32,
    pub border_highlight: Color32,
}

#[allow(dead_code)]
impl LakeTheme {
    pub fn dark() -> Self {
        Self {
            is_daylight: false,
            bg_canvas: Color32::from_rgb(0, 0, 0),
            frame_yellow: Color32::from_rgb(244, 208, 48),
            frame_cyan: Color32::from_rgb(60, 185, 240),
            lake_blue: Color32::from_rgb(0, 120, 215),
            btn_cyan_bg: Color32::from_rgb(64, 184, 244),
            btn_cyan_dark: Color32::from_rgb(20, 100, 175),
            btn_cyan_text: Color32::from_rgb(0, 30, 60),
            btn_amber_bg: Color32::from_rgb(245, 168, 28),
            btn_amber_dark: Color32::from_rgb(180, 95, 0),
            btn_amber_text: Color32::from_rgb(20, 10, 0),
            btn_gray_bg: Color32::from_rgb(130, 140, 150),
            btn_gray_dark: Color32::from_rgb(70, 78, 88),
            btn_gray_text: Color32::from_rgb(15, 20, 25),
            disc_contour_coral: Color32::from_rgb(235, 75, 60),
            disc_mesa_blue: Color32::from_rgb(120, 160, 230),
            disc_plm_amber: Color32::from_rgb(235, 155, 30),
            meter_green: Color32::from_rgb(20, 220, 40),
            meter_yellow: Color32::from_rgb(245, 210, 20),
            meter_red: Color32::from_rgb(255, 35, 35),
            meter_off: Color32::from_rgb(15, 28, 18),
            prompt_text: Color32::from_rgb(230, 230, 230),
            prompt_yellow: Color32::from_rgb(250, 240, 150),
            eq_curve_white: Color32::from_rgb(255, 255, 255),
            eq_fill_cyan: Color32::from_rgba_unmultiplied(65, 165, 235, 80),
            eq_marker_yellow: Color32::from_rgb(255, 235, 30),
            eq_marker_red: Color32::from_rgb(255, 45, 45),
            text_primary: Color32::from_rgb(240, 245, 250),
            text_muted: Color32::from_rgb(130, 145, 165),
            cyan_accent: Color32::from_rgb(60, 185, 240),
            lime_accent: Color32::from_rgb(20, 220, 40),
            orange_accent: Color32::from_rgb(255, 155, 20),
            red_clip: Color32::from_rgb(255, 40, 40),
            yellow_warn: Color32::from_rgb(244, 208, 48),
            pill_btn_bg: Color32::from_rgb(64, 184, 244),
            pill_btn_border: Color32::from_rgb(20, 100, 175),
            grid_line: Color32::from_rgb(30, 35, 42),
            bg_dark: Color32::from_rgb(0, 0, 0),
            bg_card: Color32::from_rgb(8, 10, 14),
            bg_card_selected: Color32::from_rgb(14, 18, 26),
            border: Color32::from_rgb(60, 185, 240),
            border_highlight: Color32::from_rgb(244, 208, 48),
        }
    }

    pub fn daylight() -> Self {
        let mut d = Self::dark();
        d.is_daylight = true;
        d.bg_canvas = Color32::from_rgb(228, 232, 238);
        d.bg_dark = Color32::from_rgb(228, 232, 238);
        d.bg_card = Color32::from_rgb(240, 244, 248);
        d.bg_card_selected = Color32::from_rgb(215, 230, 245);
        d.text_primary = Color32::from_rgb(20, 25, 35);
        d.text_muted = Color32::from_rgb(85, 100, 120);
        d.prompt_text = Color32::from_rgb(40, 45, 55);
        d
    }

    pub fn card_stroke(&self, selected: bool) -> Stroke {
        if selected {
            Stroke::new(2.0_f32, self.frame_yellow)
        } else {
            Stroke::new(1.2_f32, self.frame_cyan)
        }
    }
}
