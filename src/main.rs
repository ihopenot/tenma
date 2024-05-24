mod menu;
mod config;
mod game;
mod ui;
mod resource;
use config::GameState;
use bevy::prelude::*;
use resource::{GameTextures, Rule};

fn main() {
    App::new()
    .init_state::<GameState>()
    .add_plugins(bevy::DefaultPlugins)
    .add_plugins(menu::menu_plugin)
    .add_plugins(game::game_plugin)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let game_textures = GameTextures {
        background: asset_server.load("textures/background.png"),
        tile: std::array::from_fn(|_i| {
            asset_server.load(format!("textures/tiles/{}.png", _i))
        }),
        windboard: asset_server.load("textures/windboard.png"),
    };
    commands.insert_resource(game_textures);

    let game_rule = Rule::default();
    commands.insert_resource(game_rule);

}