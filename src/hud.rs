use bevy::{prelude::*, transform::commands};

use crate::{assets::Assets, state::GameState};

#[derive(Component)]
struct GameOver;

#[derive(Component)]
struct MainMenu;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), game_over)
            .add_systems(OnExit(GameState::GameOver), despawn_gameover)
            .add_systems(OnEnter(GameState::MainMenu), main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
    }
}

fn game_over(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn((
        SpriteBundle {
            texture: assets.gameover.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 5.),
                ..default()
            },
            ..default()
        },
        GameOver,
    ));
}

fn despawn_gameover(mut commands: Commands, gameover_query: Query<Entity, With<GameOver>>) {
    let Ok(entity) = gameover_query.get_single() else {
        return;
    };

    commands.entity(entity).despawn_recursive();
}

fn main_menu(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn((
        SpriteBundle {
            texture: assets.start_game.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 5.),
                ..default()
            },
            ..default()
        },
        MainMenu,
    ));
}

fn despawn_main_menu(mut commands: Commands, gameover_query: Query<Entity, With<MainMenu>>) {
    let Ok(entity) = gameover_query.get_single() else {
        return;
    };

    commands.entity(entity).despawn_recursive();
}
