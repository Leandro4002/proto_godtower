use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts};

use super::DebugConfig;

/// Listen for Debug related inputs
pub fn inputs(keycode: Res<Input<KeyCode>>, mut debug_config: ResMut<DebugConfig>) {
    if keycode.just_pressed(KeyCode::F1) {
        debug_config.show_info = !debug_config.show_info;
    }
}

/// Draw Debug Window
pub fn debug_window(
    mut debug_config: ResMut<DebugConfig>,
    mut contexts: EguiContexts,
) {
    if debug_config.show_info {
        egui::Window::new("Debug Info").show(contexts.ctx_mut(), |ui| {
            ui.label(format!("FPS: {:.2}", debug_config.fps));

            #[cfg(debug_assertions)]
            ui.checkbox(&mut debug_config.show_inspector, "Show inspector");
            ui.checkbox(&mut debug_config.draw_hitboxes, "Draw hitboxes");
        });
    }
}

/// Update the diagnostics infos
pub fn update_info_values(
    mut debug_config: ResMut<DebugConfig>,
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time>,
) {
    if debug_config.show_info {
        debug_config.update_timer.tick(time.delta());

        if debug_config.update_timer.finished() {
            // Update FPS infos
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    debug_config.fps = value;
                }
            }
        }
    }
}
