use bevy::prelude::*;

use crate::{
    bird::{Bird, BIRD_SIZE},
    pipe::{Pipe, ScoreArea, PIPE_GAP, PIPE_SIZE},
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (draw_pipe_shape, draw_bird_shape, draw_score_area_shape),
        );
    }
}

fn draw_pipe_shape(mut gizmos: Gizmos, pipe_query: Query<(&Transform, &Pipe)>) {
    for (transform, pipe) in pipe_query.iter() {
        gizmos.rect_2d(
            transform.translation.truncate() + Vec2::new(0., PIPE_SIZE.1 / 2. * pipe.flip_y),
            0.,
            Vec2::new(PIPE_SIZE.0, PIPE_SIZE.1),
            Color::RED,
        );
    }
}

fn draw_score_area_shape(mut gizmos: Gizmos, score_area_query: Query<&Transform, With<ScoreArea>>) {
    for score_area_transform in score_area_query.iter() {
        gizmos.rect_2d(
            score_area_transform.translation.truncate(),
            0.,
            Vec2::new(PIPE_SIZE.0, PIPE_GAP),
            Color::BLUE,
        );
    }
}

fn draw_bird_shape(mut gizmos: Gizmos, bird_transform_query: Query<&Transform, With<Bird>>) {
    let Ok(bird_transform) = bird_transform_query.get_single() else {
        return;
    };

    gizmos.rect_2d(
        bird_transform.translation.truncate(),
        0.,
        BIRD_SIZE.truncate(),
        Color::GREEN,
    );
}
