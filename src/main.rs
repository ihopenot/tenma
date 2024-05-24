mod menu;
mod config;
mod game;
mod ui;
use config::GameState;
use bevy::prelude::*;

fn main() {
    App::new()
    .init_state::<GameState>()
    .add_plugins(bevy::DefaultPlugins)
    .add_plugins(menu::menu_plugin)
    .add_plugins(game::game_plugin)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}