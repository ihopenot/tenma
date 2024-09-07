use crate::gameplay::state::GameState;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameEngine {
    pub game_state: GameState,
}

pub fn game_engine_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    let engine = GameEngine {
        game_state: GameState::new(None),
    };
    commands.insert_resource(engine);
}
