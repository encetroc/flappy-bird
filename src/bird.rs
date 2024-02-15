use bevy::prelude::*;

use crate::{assets::Assets, schedule::InGameSet, state::GameState, WINDOW_HEIGHT};

const GRAVITY_ACCELERATION: f32 = -9.8 * 30.;
const FLAP_VELOCITY: f32 = 200.;
const GROUND_HEIGHT: f32 = 112.;
const START_POS: (f32, f32) = (-100., 0.);
const BIRD_Z_INDEX: f32 = 4.;
pub const BIRD_SIZE: Vec3 = Vec3::new(25., 25., 0.);

#[derive(Component)]
pub struct Bird {
    velocity: Vec3,
    angular_speed: f32,
}

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            spawn_bird.in_set(InGameSet::EntityUpdates),
        )
        .add_systems(
            Update,
            (update_velocity, update_position)
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .add_systems(Update, flap.in_set(InGameSet::UserInnput));
    }
}

fn spawn_bird(
    mut commands: Commands,
    assets: Res<Assets>,
    mut bird_query: Query<(&mut Transform, &mut Bird)>,
) {
    commands.spawn(AudioBundle {
        source: assets.flap_sound.clone(),
        settings: PlaybackSettings::DESPAWN,
    });

    let Ok((mut bird_transform, mut bird)) = bird_query.get_single_mut() else {
        commands.spawn((
            SpriteBundle {
                texture: assets.bird.clone(),
                transform: Transform {
                    translation: Vec3::new(0., 0., BIRD_Z_INDEX),
                    ..default()
                },
                ..default()
            },
            Bird {
                velocity: Vec3::new(0., FLAP_VELOCITY, 0.),
                angular_speed: 10.,
            },
        ));
        return;
    };

    bird_transform.translation = Vec3::new(START_POS.0, 0., BIRD_Z_INDEX);
    bird.velocity = Vec3::new(0., FLAP_VELOCITY, 0.);
}

fn update_velocity(mut bird_query: Query<&mut Bird>, time: Res<Time>) {
    let Ok(mut bird) = bird_query.get_single_mut() else {
        return;
    };

    bird.velocity += Vec3::new(0., GRAVITY_ACCELERATION * time.delta_seconds(), 0.);
}

fn update_position(mut bird_transform_query: Query<(&mut Transform, &Bird)>, time: Res<Time>) {
    let Ok((mut bird_transform, bird)) = bird_transform_query.get_single_mut() else {
        return;
    };

    bird_transform.translation += (bird.velocity
        + Vec3::new(
            0.,
            GRAVITY_ACCELERATION * time.delta_seconds() / 2.,
            BIRD_Z_INDEX,
        ))
        * time.delta_seconds();

    let max_clamp = Vec3::new(
        START_POS.0,
        -WINDOW_HEIGHT / 2. + GROUND_HEIGHT,
        BIRD_Z_INDEX + 0.1,
    );
    let min_clamp = Vec3::new(START_POS.0, WINDOW_HEIGHT / 2., BIRD_Z_INDEX + 0.1);

    bird_transform.translation = bird_transform.translation.min(min_clamp).max(max_clamp);
}

fn flap(
    mut bird_query: Query<&mut Bird>,
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
    assets: Res<Assets>,
) {
    let Ok(mut bird) = bird_query.get_single_mut() else {
        return;
    };

    if keyboard.just_pressed(KeyCode::Space) {
        commands.spawn(AudioBundle {
            source: assets.flap_sound.clone(),
            settings: PlaybackSettings::DESPAWN,
        });

        bird.velocity = Vec3::new(0., FLAP_VELOCITY, 0.);
    }
}
