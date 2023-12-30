#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    prelude::*,
    window::{EnabledButtons, PresentMode, PrimaryWindow, WindowMode},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use main_menu::MainMenuPlugin;
use simulation::SimulationPlugin;
//use debug::DebugPlugin;

mod debug;
mod main_menu;
mod simulation;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgba_u8(0, 0, 0, 0)))
        .insert_resource(LevelSelection::index(0))
        // Default plugins
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Prototype v1 God Tower".into(),
                        resolution: (960., 540.).into(),
                        resizable: false,
                        enabled_buttons: EnabledButtons {
                            minimize: true,
                            maximize: false,
                            close: true,
                        },
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        ))
        .insert_resource(Volume(7))
        // LDtk
        .add_plugins((LdtkPlugin, RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.0),
            ..Default::default()
        })
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        // Custom
        .add_plugins((MainMenuPlugin, SimulationPlugin/*, DebugPlugin*/)) // DebugPlugin crashes the program
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, fullscreen)
        .run();
}

fn fullscreen(keys: Res<Input<KeyCode>>, mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    if keys.just_pressed(KeyCode::F11) {
        let mut window = q_windows.single_mut();
        if window.mode == WindowMode::Fullscreen {
            window.mode = WindowMode::Windowed;
        } else {
            window.mode = WindowMode::Fullscreen;
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(u32);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Simulation,
}
