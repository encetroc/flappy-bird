use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Assets {
    pub bird: Handle<Image>,
    pub base: Handle<Image>,
    pub pipe: Handle<Image>,
    pub bg: Handle<Image>,
    pub gameover: Handle<Image>,
    pub start_game: Handle<Image>,
    pub flap_sound: Handle<AudioSource>,
    pub point: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Assets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<Assets>, asset_server: Res<AssetServer>) {
    *scene_assets = Assets {
        bird: asset_server.load("sprites/yellowbird-upflap.png"),
        base: asset_server.load("sprites/base.png"),
        pipe: asset_server.load("sprites/pipe-green.png"),
        bg: asset_server.load("sprites/background-day.png"),
        gameover: asset_server.load("sprites/gameover.png"),
        start_game: asset_server.load("sprites/message.png"),
        flap_sound: asset_server.load("audio/wing.ogg"),
        point: asset_server.load("audio/point.ogg"),
        hit: asset_server.load("audio/hit.ogg"),
    }
}
