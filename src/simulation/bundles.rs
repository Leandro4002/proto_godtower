use bevy::prelude::*;

use super::components::Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite: SpriteBundle
}

impl PlayerBundle {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            player: Player::default(),
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/player.png"),
                transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(2., 2., 1.)),
                ..default()
            }
        }
    }
}