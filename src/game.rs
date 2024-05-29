use bevy::{math::f32, prelude::*, ui::widget::UiImageSize};
use derivative::Derivative;
use rand::Rng;

use crate::config::{Dahai, GameState, InGameState};
use crate::resource::Rule;
use crate::ui::ui_plugin;
use crate::{tu8, tuz};

#[derive(Resource, Derivative)]
#[derivative(Default)]
pub struct Game {
    // 本场
    pub honba: u8,
    // 局
    pub kyoku: u8,
    pub dora: [u8; 4],
    pub uradora: [u8; 4],

    pub bakaze: u8,
    #[derivative(Default(value = "[0; tuz!(?)]"))]
    pub yama: [u8; tuz!(?)],
    #[derivative(Default(value = "136"))]
    pub remain: u8,

    pub status: [PlayerStatus; 4],
    pub self_id: u8,
}

impl Game {
    pub fn reset(&mut self, game_rule: Res<Rule>) {
        *self = Game::default();

        for i in 0..tuz!(?) {
            self.yama[i] = 4;
        }

        // 有赤规则
        if game_rule.akaari {
            self.yama[tuz!(5m)] = 3;
            self.yama[tuz!(5p)] = 3;
            self.yama[tuz!(5s)] = 3;
            self.yama[tuz!(5mr)] = 1;
            self.yama[tuz!(5pr)] = 1;
            self.yama[tuz!(5sr)] = 1;
        } else {
            self.yama[tuz!(5mr)] = 0;
            self.yama[tuz!(5pr)] = 0;
            self.yama[tuz!(5sr)] = 0;
        }

        // 决定谁是东起
        #[cfg(feature = "debug")]
        let player_id = 0;
        #[cfg(not(feature = "debug"))]
        let player_id = rand::thread_rng().gen_range(0..4);
        self.self_id = player_id;
    }

    pub fn draw_tile(&mut self) -> u8 {
        let mut rng = rand::thread_rng();
        let tile_id = rng.gen_range(0..self.remain);

        let mut sum = 0;
        let mut tile = 0;
        for i in 0..tuz!(?) {
            sum += self.yama[i];
            if sum > tile_id {
                tile = i;
                break;
            }
        }

        self.remain -= 1;
        tile as u8
    }

    pub fn start_new_game(&mut self) {
        for i in 0..4 {
            for j in 0..13 {
                self.status[i].tehai[j] = self.draw_tile();
            }
        }
    }

    pub fn can_naki(&self) -> bool {
        #[cfg(feature = "debug")]
        return false;

        #[cfg(not(feature = "debug"))]
        todo!("can_naki")
    }

    pub fn dahai(
        &mut self,
        player: u8,
        slot: u8,
        state: Res<State<InGameState>>,
        mut next_state: ResMut<NextState<InGameState>>,
    ) {
        self.status[player as usize].tehai[slot as usize] = self.status[player as usize].tsumo;
        self.status[player as usize].tsumo = tu8!(-);
        println!("Player {} dahai {}", player, slot);

        if self.can_naki() {
            next_state.set(InGameState::WaitNaki);
        } else {
            match state.get() {
                InGameState::SelfPlay => {
                    next_state.set(InGameState::RightPlay);
                }
                InGameState::RightPlay => {
                    next_state.set(InGameState::AcrossPlay);
                }
                InGameState::AcrossPlay => {
                    next_state.set(InGameState::LeftPlay);
                }
                InGameState::LeftPlay => {
                    next_state.set(InGameState::SelfPlay);
                }
                _ => {}
            }
        }
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
    pub tsumo: u8,
}

pub fn game_plugin(app: &mut App) {
    app.insert_resource(Game { ..default() })
        .init_state::<InGameState>()
        .add_plugins(ui_plugin)
        .add_systems(OnEnter(GameState::Game), setup_game)
        .add_systems(OnEnter(InGameState::GeneralUI), prepare_game)
        .add_systems(Update, game_dahai.run_if(on_event::<Dahai>()));
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ingamestate: ResMut<NextState<InGameState>>,
) {
    ingamestate.set(InGameState::GeneralUI);
}

fn prepare_game(mut game: ResMut<Game>, game_rule: Res<Rule>) {
    game.reset(game_rule);
    game.start_new_game();
}

fn game_dahai(
    mut game: ResMut<Game>,
    mut dahai: EventReader<Dahai>,
    state: Res<State<InGameState>>,
    next_state: ResMut<NextState<InGameState>>,
) {
    // 只处理一次打牌
    for &Dahai { player, slot } in dahai.read() {
        game.dahai(player, slot, state, next_state);
        break;
    }
}
