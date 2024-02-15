use bevy::prelude::*;

use crate::{assets::Assets, collision::CollisionEvent};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    InGame,
    GameOver,
    #[default]
    MainMenu,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(Update, (start_game).run_if(in_state(GameState::MainMenu)))
            .add_systems(Update, (stop_game).run_if(in_state(GameState::InGame)))
            .add_systems(
                Update,
                (goto_main_menu).run_if(in_state(GameState::GameOver)),
            );
    }
}

fn start_game(mut next_state: ResMut<NextState<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::InGame);
    }
}

fn stop_game(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut collision_event_reader: EventReader<CollisionEvent>,
    assets: Res<Assets>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::GameOver);
    }

    for _ in collision_event_reader.read().into_iter() {
        commands.spawn(AudioBundle {
            source: assets.hit.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
        next_state.set(GameState::GameOver);
    }
}

fn goto_main_menu(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::MainMenu);
    }
}
