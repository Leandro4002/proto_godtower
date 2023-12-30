use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::AppState;
use self::systems::*;

mod systems;
mod components;
mod bundles;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Simulation), on_enter_simulation)
            //.add_systems(Update, inputs.run_if(in_state(AppState::Simulation)))
            .add_systems(OnExit(AppState::Simulation), on_exit_simulation)
            .add_systems(Startup,setup)
            .add_systems(Update, spawn_wall_collision)
            .add_systems(Update, movement)
            .add_systems(Update, detect_climb_range)
            .add_systems(Update, ignore_gravity_if_climbing)
            .add_systems(Update, patrol)
            .add_systems(Update, camera_fit_inside_current_level)
            .add_systems(Update, update_level_selection)
            .add_systems(Update, dbg_player_items)
            .add_systems(Update, spawn_ground_sensor)
            .add_systems(Update, ground_detection)
            .add_systems(Update, update_on_ground)
            .add_systems(Update, restart_level)
            .register_ldtk_int_cell::<components::WallBundle>(1)
            .register_ldtk_int_cell::<components::LadderBundle>(2)
            .register_ldtk_int_cell::<components::WallBundle>(3)
            .register_ldtk_entity::<components::PlayerBundle>("Player")
            .register_ldtk_entity::<components::MobBundle>("Mob")
            .register_ldtk_entity::<components::ChestBundle>("Chest")
            .register_ldtk_entity::<components::PumpkinsBundle>("Pumpkins");
    }
}
