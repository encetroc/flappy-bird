use bevy::prelude::*;
use rand::Rng;

use crate::{
    assets::Assets,
    collision::{Collider, ScoreEvent},
    schedule::InGameSet,
    state::GameState,
};

pub const PIPE_GAP: f32 = 100.;
pub const PIPE_SIZE: (f32, f32) = (52., 320.);
const PIPE_SPAWN_X: f32 = 200.;
const PIPE_VELOCITY: f32 = 100.;
const PIPE_GAP_RANGE: f32 = 100.;
const PIPE_Z_INDEX: f32 = 1.;
const SCORE_AREA_OFFSET: f32 = 30.0;

#[derive(Component)]
pub struct ScoreArea;

#[derive(Resource)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct PipePlugin;

#[derive(Component)]
pub struct Pipe {
    pub flip_y: f32,
}

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        })
        // .add_systems(PostStartup, spawn_pipe)
        .add_systems(
            Update,
            (move_pipe, spawn_pipe)
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .add_systems(
            Update,
            (despawn_pipe, despawn_score_area).in_set(InGameSet::DespawnEntities),
        )
        .add_systems(OnEnter(GameState::MainMenu), despawn_all_pipes);
    }
}

fn spawn_pipe(
    mut commands: Commands,
    assets: Res<Assets>,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
) {
    spawn_timer.timer.tick(time.delta());

    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let gap_variation: f32 = rng.gen_range(-PIPE_GAP_RANGE..PIPE_GAP_RANGE);

    commands.spawn((
        SpriteBundle {
            texture: assets.pipe.clone(),
            sprite: Sprite {
                flip_y: true,
                anchor: bevy::sprite::Anchor::BottomCenter,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(PIPE_SPAWN_X, PIPE_GAP + gap_variation, PIPE_Z_INDEX),
                ..default()
            },
            ..default()
        },
        Pipe { flip_y: 1. },
        Collider,
    ));

    commands.spawn((
        SpriteBundle {
            texture: assets.pipe.clone(),
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::TopCenter,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(PIPE_SPAWN_X, gap_variation, PIPE_Z_INDEX),
                ..default()
            },
            ..default()
        },
        Pipe { flip_y: -1. },
        Collider,
    ));

    commands.spawn((
        Transform {
            translation: Vec3::new(
                PIPE_SPAWN_X + SCORE_AREA_OFFSET,
                PIPE_GAP / 2. + gap_variation,
                0.,
            ),
            ..default()
        },
        ScoreArea,
    ));
}

fn move_pipe(
    mut pipe_transform_query: Query<&mut Transform, Or<(With<Pipe>, With<ScoreArea>)>>,
    time: Res<Time>,
) {
    for mut pipe_transform in pipe_transform_query.iter_mut() {
        pipe_transform.translation += Vec3::new(-PIPE_VELOCITY * time.delta_seconds(), 0., 0.);
    }
}

fn despawn_pipe(mut commands: Commands, pipe_query: Query<(Entity, &Transform), With<Pipe>>) {
    for (entity, pipe_transform) in pipe_query.iter() {
        if pipe_transform.translation.x < -200. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_all_pipes(
    mut commands: Commands,
    pipe_query: Query<Entity, Or<(With<Pipe>, With<ScoreArea>)>>,
) {
    for entity in pipe_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_score_area(
    mut score_event_reader: EventReader<ScoreEvent>,
    mut commands: Commands,
    assets: Res<Assets>,
) {
    for &ScoreEvent { entity } in score_event_reader.read() {
        commands.entity(entity).despawn_recursive();

        commands.spawn(AudioBundle {
            source: assets.point.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
