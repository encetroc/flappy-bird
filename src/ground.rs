use bevy::prelude::*;

use crate::{assets::Assets, WINDOW_HEIGHT};

const GROUND_Z_INDEX: f32 = 2.;
pub const GROUND_HEIGHT: f32 = 112.;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (spawn_ground, spawn_bg));
    }
}

fn spawn_ground(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn(SpriteBundle {
        texture: assets.base.clone(),
        sprite: Sprite {
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(1., -WINDOW_HEIGHT / 2., GROUND_Z_INDEX),
            ..default()
        },
        ..default()
    });
}

fn spawn_bg(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn(SpriteBundle {
        texture: assets.bg.clone(),
        transform: Transform {
            translation: Vec3::ZERO,
            scale: Vec3::new(1.2, 1.2, 0.),
            ..default()
        },
        ..default()
    });
}
