use bevy::{math::f32, prelude::*, ui::widget::UiImageSize};
use derivative::Derivative;
use rand::Rng;

use crate::config::{GameState, InGameState, GameRule};
use crate::ui::ui_plugin;

#[derive(Resource, Derivative)]
#[derivative(Default)]
struct GameStatistics {
    // 本场
    honba: u16,
    // 局
    kyoku: u16,
    dora: [i32; 4],
    uradora: [i32; 4],

    bakaze: u8,
    #[derivative(Default(value = "[4; 34]"))]
    yama: [u8; 34],
    #[derivative(Default(value = "136"))]
    remain: u8,
}

impl GameStatistics {
    pub fn reset(&mut self) {
        *self = GameStatistics::default();
    }

    pub fn draw_tile(&mut self) -> u8 {
        let mut rng = rand::thread_rng();
        let tile_id = rng.gen_range(0..self.remain);

        let mut sum = 0;
        let mut tile = 0;
        for i in 0..34 {
            sum += self.yama[i];
            if sum > tile_id {
                tile = i;
                break;
            }
        }

        self.remain -= 1;
        tile as u8
    }

    pub fn can_draw_tile(&self) -> bool {
        self.remain > GameRule.ace_remain
    }
}

#[derive(Derivative)]
#[derivative(Default)]
struct PlayerStatus {
    #[derivative(Default(value = "25000"))]
    score: i32,
    jikaze: u8,
    #[derivative(Default(value = "[0; 34]"))]
    tehai: [u8; 34],
}

#[derive(Resource, Default)]
struct PlayerStatistics {
    status: [PlayerStatus; 4],
    self_id: u8,
}

#[derive(Component)]
struct GameInfoPublic;

pub fn game_plugin(app: &mut App) {
    app
    .insert_resource(GameStatistics{..default()})
    .insert_resource(PlayerStatistics{..default()})
    .init_state::<InGameState>()
    .add_plugins(ui_plugin)
    .add_systems(OnEnter(GameState::Game), setup_game)
    .add_systems(OnEnter(InGameState::GeneralUI), prepare_game);
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>, mut ingamestate: ResMut<NextState<InGameState>>) {
    ingamestate.set(InGameState::GeneralUI);
}

fn prepare_game(mut game_statistics: ResMut<GameStatistics>, mut player_statistics: ResMut<PlayerStatistics>) {
    game_statistics.reset();
    player_statistics.self_id = rand::thread_rng().gen_range(0..4);
    for i in 0..4 {
        for j in 0..13 {
            player_statistics.status[i].tehai[j] = game_statistics.draw_tile();
        }
    }
}