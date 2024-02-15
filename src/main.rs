mod assets;
mod bird;
mod collision;
mod debug;
mod ground;
mod hud;
mod pipe;
mod schedule;
mod state;

use assets::AssetsPlugin;
use bevy::{prelude::*, window::WindowResolution};
use bird::BirdPlugin;
use collision::CollisionPlugin;
use debug::DebugPlugin;
use ground::GroundPlugin;
use hud::HudPlugin;
use pipe::PipePlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;

const WINDOW_ZOOM: f32 = 2.;
pub const WINDOW_WIDTH: f32 = 320.;
pub const WINDOW_HEIGHT: f32 = 568.;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(320. * WINDOW_ZOOM, 568. * WINDOW_ZOOM)
                            .with_scale_factor_override(WINDOW_ZOOM as f64),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, spawn_camera)
        .add_plugins(AssetsPlugin)
        .add_plugins(BirdPlugin)
        .add_plugins(GroundPlugin)
        .add_plugins(PipePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(HudPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(DebugPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
