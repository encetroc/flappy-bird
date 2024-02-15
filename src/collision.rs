use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    assets::Assets,
    bird::{Bird, BIRD_SIZE},
    ground::GROUND_HEIGHT,
    pipe::{Pipe, ScoreArea, PIPE_GAP, PIPE_SIZE},
    schedule::InGameSet,
    WINDOW_HEIGHT,
};

const ROOF_LIMIT: f32 = WINDOW_HEIGHT / 2.;
const FLOOR_LIMIT: f32 = -WINDOW_HEIGHT / 2. + GROUND_HEIGHT;

#[derive(Component)]
pub struct Collider;

#[derive(Event)]
pub struct CollisionEvent;

#[derive(Event)]
pub struct ScoreEvent {
    pub entity: Entity,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_pipe_collisions,
                handle_ground_roof_collision,
                handle_score_area_collisions,
            )
                .in_set(InGameSet::CollisionDetection),
        )
        .add_event::<CollisionEvent>()
        .add_event::<ScoreEvent>();
    }
}

fn handle_pipe_collisions(
    bird_query: Query<&Transform, With<Bird>>,
    collider_query: Query<(&Transform, &Pipe), With<Collider>>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    let bird_transform = bird_query.single();

    for (transform, pipe) in &collider_query {
        let collision = collide(
            bird_transform.translation,
            BIRD_SIZE.truncate(),
            transform.translation + Vec3::new(0., PIPE_SIZE.1 / 2. * pipe.flip_y, 0.),
            Vec2::new(PIPE_SIZE.0, PIPE_SIZE.1),
        );

        if let Some(_) = collision {
            collision_event_writer.send(CollisionEvent);
        }
    }
}

fn handle_score_area_collisions(
    bird_query: Query<&Transform, With<Bird>>,
    collider_query: Query<(Entity, &Transform), With<ScoreArea>>,
    mut score_event_writer: EventWriter<ScoreEvent>,
) {
    let bird_transform = bird_query.single();

    for (entity, transform) in &collider_query {
        let collision = collide(
            bird_transform.translation,
            BIRD_SIZE.truncate(),
            transform.translation,
            Vec2::new(PIPE_SIZE.0, PIPE_GAP),
        );

        if let Some(_) = collision {
            score_event_writer.send(ScoreEvent { entity });
        }
    }
}

fn handle_ground_roof_collision(
    bird_transform_query: Query<&Transform, With<Bird>>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    let Ok(bird_transform) = bird_transform_query.get_single() else {
        return;
    };

    if (bird_transform.translation.y >= ROOF_LIMIT) | (bird_transform.translation.y <= FLOOR_LIMIT)
    {
        collision_event_writer.send(CollisionEvent);
    }
}
