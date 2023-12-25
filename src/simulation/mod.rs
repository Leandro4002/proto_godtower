use bevy::prelude::*;

use crate::AppState;

use self::systems::*;
mod systems;
mod components;
mod bundles;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Simulation), on_enter_simulation)
            .add_systems(Update, inputs.run_if(in_state(AppState::Simulation)))
            .add_systems(OnExit(AppState::Simulation), on_exit_simulation);
    }
}
