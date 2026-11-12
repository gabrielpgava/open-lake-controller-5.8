use crate::model::{AppView, BottomMenuContext, Module, ModuleEditorTab};
use crate::theme::LakeTheme;
use crate::ui::widgets::draw_lake_pill_button;
use eframe::egui::{self, Pos2, Rect, Sense, Stroke, Ui, Vec2};

pub fn render_bottom_bar(
    ui: &mut Ui,
    theme: &LakeTheme,
    app_view: &mut AppView,
    active_tab: &mut ModuleEditorTab,
    menu_context: &mut BottomMenuContext,
    mut selected_module: Option<&mut Module>,
    _daylight_mode: &mut bool,
    _windowed_mode: &mut bool,
) {
    let bar_height = 82.0;
    let (response, painter) = ui.allocate_painter(Vec2::new(ui.available_width(), bar_height), Sense::hover());
    let bounds = response.rect;

    // Pitch black background
    painter.rect_filled(bounds, 0.0, theme.bg_canvas);

    // Yellow prompt line separator (authentic Lake Controller yellow divider)
    painter.line_segment(
        [Pos2::new(bounds.min.x + 8.0, bounds.min.y + 1.0), Pos2::new(bounds.max.x - 8.0, bounds.min.y + 1.0)],
        Stroke::new(1.5_f32, theme.frame_yellow),
    );

    // Top status / prompt text
    let prompt_y = bounds.min.y + 11.0;
    let prompt_str = match *app_view {
        AppView::HomeWorkarea => {
            match *menu_context {
                BottomMenuContext::Modules => "Select a module to add or drag onto the workspace",
                BottomMenuContext::Groups => "Select a group to adjust group gain, delay, and mutes",
                _ => "Tap a button below to configure the system, select a Module or Group above to EQ and set Levels",
            }
        }
        AppView::ModuleEditor => match *active_tab {
            ModuleEditorTab::SuperEq | ModuleEditorTab::GraphicEq => {
                "Designer Mode: Adjust Equalizer parameters or select \"Levels\" tab to set Levels"
            }
            ModuleEditorTab::LevelsDelay | ModuleEditorTab::LimiterMax => {
                "Adjust Faders, Mutes and Delays or select \"EQ\" tab"
            }
            ModuleEditorTab::IOConfig => "Select input routing and digital clock options",
        },
        AppView::SystemStoreRecall => "System Store and Recall: Save or load Lake System configurations",
        AppView::SoloMuteMatrix => "Solo / Mute Matrix: Mute or solo individual channels across frames",
        AppView::UserPreferences => "User Preferences: Display, network, and window configuration",
        AppView::NetworkConfig => "Network Configuration: Select active network interface (NIC), configure UDP 6004, and discover Lake processors",
    };

    painter.text(
        Pos2::new(bounds.min.x + 14.0, prompt_y),
        egui::Align2::LEFT_CENTER,
        prompt_str,
        egui::FontId::proportional(12.5),
        theme.prompt_text,
    );

    painter.text(
        Pos2::new(bounds.max.x - 14.0, prompt_y),
        egui::Align2::RIGHT_CENTER,
        "No Configuration",
        egui::FontId::proportional(12.5),
        theme.prompt_text,
    );

    // Buttons Row
    let btn_row_y = bounds.min.y + 24.0;
    let btn_h = 50.0;

    // 1. Far Left Page Scroll / Up Arrow Button (Cyan narrow vertical)
    let up_w = 26.0;
    let up_rect = Rect::from_min_size(Pos2::new(bounds.min.x + 10.0, btn_row_y), Vec2::new(up_w, btn_h));
    let up_resp = ui.interact(up_rect, ui.make_persistent_id("lake_nav_up"), Sense::click());
    draw_lake_pill_button(
        &painter,
        up_rect,
        "▲\n1",
        None,
        false,
        false,
        up_resp.hovered(),
        theme,
    );

    // 2. Far Left Fast-Forward Button (Gray metallic)
    let ff_w = 40.0;
    let ff_rect = Rect::from_min_size(Pos2::new(up_rect.max.x + 6.0, btn_row_y), Vec2::new(ff_w, btn_h));
    let ff_resp = ui.interact(ff_rect, ui.make_persistent_id("lake_nav_ff"), Sense::click());
    draw_lake_pill_button(
        &painter,
        ff_rect,
        "▶▶",
        None,
        false,
        false,
        ff_resp.hovered(),
        theme,
    );

    // 3. Far Right Buttons: Undo and Backward
    let undo_w = 48.0;
    let undo_rect = Rect::from_min_size(Pos2::new(bounds.max.x - undo_w - 10.0, btn_row_y), Vec2::new(undo_w, btn_h));
    let undo_resp = ui.interact(undo_rect, ui.make_persistent_id("lake_undo"), Sense::click());
    draw_lake_pill_button(
        &painter,
        undo_rect,
        "Undo",
        None,
        false,
        false,
        undo_resp.hovered(),
        theme,
    );

    let rw_w = 40.0;
    let rw_rect = Rect::from_min_size(Pos2::new(undo_rect.min.x - rw_w - 6.0, btn_row_y), Vec2::new(rw_w, btn_h));
    let rw_resp = ui.interact(rw_rect, ui.make_persistent_id("lake_nav_rw"), Sense::click());
    draw_lake_pill_button(
        &painter,
        rw_rect,
        "◀◀",
        None,
        false,
        false,
        rw_resp.hovered(),
        theme,
    );

    // 4. Main 10 Buttons in the middle
    let center_start_x = ff_rect.max.x + 8.0;
    let center_end_x = rw_rect.min.x - 8.0;
    let available_w = (center_end_x - center_start_x).max(100.0);
    let num_main_btns = 10;
    let btn_spacing = 6.0;
    let btn_w = ((available_w - (num_main_btns as f32 - 1.0) * btn_spacing) / num_main_btns as f32).clamp(55.0, 130.0);

    match *app_view {
        AppView::HomeWorkarea => {
            let defs = [
                ("Home", "F1", *menu_context == BottomMenuContext::Home),
                ("System\nStore/Recall", "F2", *menu_context == BottomMenuContext::SystemStoreRecall),
                ("Modules", "F3", *menu_context == BottomMenuContext::Modules),
                ("Groups", "F4", *menu_context == BottomMenuContext::Groups),
                ("Solo/Mute", "F5", *menu_context == BottomMenuContext::SoloMute),
                ("Pages", "F6", *menu_context == BottomMenuContext::Pages),
                ("User\nPreferences", "F7", *menu_context == BottomMenuContext::UserPreferences),
                ("Icon\nControl", "F8", *menu_context == BottomMenuContext::IconControl),
                ("Network", "F9", *menu_context == BottomMenuContext::Network),
                ("Quit\nController", "F10", false),
            ];

            for (idx, (label, fkey, is_active)) in defs.iter().enumerate() {
                let bx = center_start_x + (idx as f32) * (btn_w + btn_spacing);
                let rect = Rect::from_min_size(Pos2::new(bx, btn_row_y), Vec2::new(btn_w, btn_h));
                let resp = ui.interact(rect, ui.make_persistent_id(format!("main_btn_{}", idx)), Sense::click());

                draw_lake_pill_button(
                    &painter,
                    rect,
                    label,
                    Some(fkey),
                    *is_active,
                    false,
                    resp.hovered(),
                    theme,
                );

                if resp.clicked() {
                    match idx {
                        0 => {
                            *menu_context = BottomMenuContext::Home;
                            *app_view = AppView::HomeWorkarea;
                        }
                        1 => {
                            *menu_context = BottomMenuContext::SystemStoreRecall;
                            *app_view = AppView::SystemStoreRecall;
                        }
                        2 => {
                            *menu_context = BottomMenuContext::Modules;
                        }
                        3 => {
                            *menu_context = BottomMenuContext::Groups;
                        }
                        4 => {
                            *menu_context = BottomMenuContext::SoloMute;
                            *app_view = AppView::SoloMuteMatrix;
                        }
                        5 => {
                            *menu_context = BottomMenuContext::Pages;
                        }
                        6 => {
                            *menu_context = BottomMenuContext::UserPreferences;
                            *app_view = AppView::UserPreferences;
                        }
                        7 => {
                            *menu_context = BottomMenuContext::IconControl;
                        }
                        8 => {
                            *menu_context = BottomMenuContext::Network;
                            *app_view = AppView::NetworkConfig;
                        }
                        9 => {
                            // Quit Controller
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        _ => {}
                    }
                }
            }
        }

        AppView::ModuleEditor => {
            let eq_defs = [
                ("Home", "F1", false, false),
                ("EQ/Levels\nEXIT", "F2", true, false), // Golden amber active!
                ("Overlay\nFunctions", "F3", false, false),
                ("Overlay\nBYPASS", "F4", false, false),
                ("Overlay\nFlat", "F5", false, false),
                ("Filter\nBYPASS", "F6", false, false),
                ("Filter\nFlat", "F7", false, false),
                ("Filter\nDelete", "F8", false, true), // Gray
                ("Filter\nEdit", "F9", false, false),
                ("Analyzer", "F10", false, false),
            ];

            for (idx, (label, fkey, is_active, is_disabled)) in eq_defs.iter().enumerate() {
                let bx = center_start_x + (idx as f32) * (btn_w + btn_spacing);
                let rect = Rect::from_min_size(Pos2::new(bx, btn_row_y), Vec2::new(btn_w, btn_h));
                let resp = ui.interact(rect, ui.make_persistent_id(format!("eq_btn_{}", idx)), Sense::click());

                draw_lake_pill_button(
                    &painter,
                    rect,
                    label,
                    Some(fkey),
                    *is_active,
                    *is_disabled,
                    resp.hovered(),
                    theme,
                );

                if resp.clicked() {
                    match idx {
                        0 | 1 => {
                            // Home or Exit
                            *app_view = AppView::HomeWorkarea;
                        }
                        4 | 6 => {
                            // Flat
                            if let Some(m) = selected_module.as_deref_mut() {
                                for f in &mut m.peq_filters {
                                    f.gain = 0.0;
                                }
                                m.geq_bands = [0.0; 28];
                            }
                        }
                        5 => {
                            // Filter Bypass
                            if let Some(m) = selected_module.as_deref_mut() {
                                for f in &mut m.peq_filters {
                                    f.enabled = !f.enabled;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        _ => {
            // In other views, offer Home [F1] and Back
            let bx = center_start_x;
            let rect = Rect::from_min_size(Pos2::new(bx, btn_row_y), Vec2::new(140.0, btn_h));
            let resp = ui.interact(rect, ui.make_persistent_id("return_home_btn"), Sense::click());

            draw_lake_pill_button(
                &painter,
                rect,
                "Home",
                Some("F1"),
                true,
                false,
                resp.hovered(),
                theme,
            );

            if resp.clicked() {
                *app_view = AppView::HomeWorkarea;
            }
        }
    }
}
