use bevy::{math::f32, prelude::*, ui::widget::UiImageSize};
use derivative::Derivative;
use rand::Rng;

use crate::config::{GameState, InGameState};
use crate::resource::Rule;
use crate::ui::ui_plugin;

#[derive(Resource, Derivative)]
#[derivative(Default)]
pub struct Game {
    // 本场
    pub honba: u16,
    // 局
    pub kyoku: u16,
    pub dora: [i32; 4],
    pub uradora: [i32; 4],

    pub bakaze: u8,
    #[derivative(Default(value = "[4; 37]"))]
    pub yama: [u8; 37],
    #[derivative(Default(value = "136"))]
    pub remain: u8,
    
    pub status: [PlayerStatus; 4],
    pub self_id: u8,
}

impl Game {
    pub fn reset(&mut self, game_rule: Res<Rule>) {
        *self = Game::default();

        // 有赤规则
        game_rule.akaari.then(|| {
            self.yama[0] = 4;
            self.yama[8] = 4;
            self.yama[17] = 4;
            self.yama[26] = 4;
        });
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

    pub fn dahai(&mut self, player: u8, slot: u8) {
        let tile = self.status[player as usize].tehai[slot as usize];
        self.status[player as usize].tehai[slot as usize] = 0;
        self.status[player as usize].tehai[slot as usize] = self.draw_tile();
    }
    // pub fn can_draw_tile(&self) -> bool {
    //     self.remain > GameRule.ace_remain
    // }
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct PlayerStatus {
    #[derivative(Default(value = "25000"))]
    pub score: i32,
    pub jikaze: u8,
    #[derivative(Default(value = "[0; 37]"))]
    pub tehai: [u8; 37],
}

pub fn game_plugin(app: &mut App) {
    app
    .insert_resource(Game{..default()})
    .init_state::<InGameState>()
    .add_plugins(ui_plugin)
    .add_systems(OnEnter(GameState::Game), setup_game)
    .add_systems(OnEnter(InGameState::GeneralUI), prepare_game);
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>, mut ingamestate: ResMut<NextState<InGameState>>) {
    ingamestate.set(InGameState::GeneralUI);
}

fn prepare_game(mut game: ResMut<Game>, game_rule: Res<Rule>) {
    game.reset(game_rule);
    game.self_id = rand::thread_rng().gen_range(0..4);
    for i in 0..4 {
        for j in 0..13 {
            game.status[i].tehai[j] = game.draw_tile();
        }
    }
}