use bevy::prelude::*;

use crate::simulation::bundles::PlayerBundle;
use crate::simulation::components::*;
use crate::MainCamera;

const PLAYER_SPEED: f32 = 300.;

pub fn inputs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_player: Query<(&mut Player, &mut Transform)>,
) {
    let delta_time = time.delta_seconds();

    for (mut player, mut transform) in q_player.iter_mut() {
        // Get Direction Inputs
        let direction = Vec3::new(
            (keys.pressed(KeyCode::D) as i8 - keys.pressed(KeyCode::A) as i8) as f32,
            (keys.pressed(KeyCode::W) as i8 - keys.pressed(KeyCode::S) as i8) as f32,
            0.,
        )
        .normalize_or_zero();

        transform.translation += direction * PLAYER_SPEED * delta_time;
    }
}

pub fn on_enter_simulation(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("on_enter_simulation");

    commands.spawn(PlayerBundle::new(&asset_server));
}

pub fn on_exit_simulation() {
    info!("on_exit_simulation");
}
