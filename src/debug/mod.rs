use self::system::*;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod system;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugConfig::default())
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Update, (inputs, debug_window, update_info_values));

        #[cfg(debug_assertions)]
        {
            app.add_plugins(
                WorldInspectorPlugin::new()
                    .run_if(|debug_config: Res<DebugConfig>| debug_config.show_inspector),
            );
        }
    }
}

/// Configuration of the Debug infos
/// These options are modified in the Debug Window (F1)
/// Some options might only be available in debug builds
#[derive(Resource, Default)]
pub struct DebugConfig {
    show_info: bool,
    show_inspector: bool,
    fps: f64,
    /// At wich rate the diagnostics info are updated
    update_timer: Timer,
    pub draw_hitboxes: bool,
}