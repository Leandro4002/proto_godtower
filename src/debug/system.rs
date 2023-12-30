use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui::{*, self}, EguiContexts};

use crate::simulation::components::*;

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
    mut q_player: Query<(& bevy_rapier2d::dynamics::Velocity, &mut Climber, &mut GroundDetection), With<Player>>,
    mut debug_render_ctx: ResMut<bevy_rapier2d::render::DebugRenderContext>
) {
    if debug_config.show_info {
        egui::Window::new("Debug Info").show(contexts.ctx_mut(), |ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
            ui.label(format!("FPS: {:.2}", debug_config.fps));

            #[cfg(debug_assertions)]
            ui.checkbox(&mut debug_config.show_inspector, "Show inspector");
            ui.checkbox(&mut debug_render_ctx.enabled, "Draw hitboxes");
            ui.separator();
            ui.label(RichText::new("Player").font(FontId::proportional(20.0)));
            for (velocity, mut climber, mut ground_detection) in &mut q_player {
                label_vec2(ui, "velocity", velocity.linvel);
                ui.checkbox(&mut climber.climbing, "climbing");
                ui.checkbox(&mut ground_detection.on_ground, "on_ground");
            }
        });
    }
}

fn label_vec2(ui: &mut Ui, text: &str, val: bevy::math::Vec2) {
    ui.label(format!("{}: {:>8.2},{:>8.2}", text, val.x, val.y));
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
