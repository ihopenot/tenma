use bevy::{math::f32, prelude::*, ui::widget::UiImageSize};
use derivative::Derivative;
use rand::Rng;

use crate::config::{Dahai, GameState, InGameState, PLAY_MAP, TSUMO_MAP, NAKI_MAP};
use crate::resource::Rule;
use crate::ui::ui_plugin;
use crate::{checkstate, state2id, tu8, tuz, id2state, nextplayer};

const TSUMO_SLOT : usize = 14;

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
    pub ingamestate: InGameState,
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct PlayerStatus {
    #[derivative(Default(value = "25000"))]
    pub score: i32,
    pub jikaze: u8,
    #[derivative(Default(value = "[tu8!(-); 14]"))]
    pub tehai: [u8; 14],
}
pub enum GameError {
    InvalidPlayer,
    InvalidState,
}

pub enum NakiType {
    Chi,
    Pon,
    Kang,
    Pei
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
        self.ingamestate = InGameState::SelfTsumo;
    }

    pub fn can_naki(&self) -> bool {
        #[cfg(feature = "debug")]
        return false;

        #[cfg(not(feature = "debug"))]
        todo!("can_naki")
    }

    pub fn tsumo(&mut self, player: u8) -> Result<u8, GameError> {
        if player != state2id!(self.ingamestate) {
            return Err(GameError::InvalidPlayer);
        }
        let tile = self.draw_tile();
        self.status[player as usize].tehai[TSUMO_SLOT] = tile;
        println!("Player {} tsumo {}", player, tile);
        self.ingamestate = id2state!(player, play);
        Ok(tile)
    }

    pub fn dahai(
        &mut self,
        player: u8,
        slot: u8,
    ) -> Result<(), GameError> {
        let current_player = state2id!(self.ingamestate);
        if player != current_player {
            return Err(GameError::InvalidPlayer);
        }
        if !checkstate!(self.ingamestate, play) {
            return Err(GameError::InvalidState);
        }

        // 如果刚鸣牌，会直接往手里塞一张空白牌
        self.status[player as usize].tehai[slot as usize] = self.status[player as usize].tehai[TSUMO_SLOT];
        self.status[player as usize].tehai[TSUMO_SLOT] = tu8!(-);
        println!("Player {} dahai {}", player, slot);

        if self.can_naki() {
            todo!("naki")
        } else {
            self.ingamestate = id2state!(nextplayer!(current_player), tsumo);
        }
        Ok(())
    }

    pub fn naki(
        &mut self,
        player: u8,
        nakitype: NakiType,
    ) {
        #[cfg(feature = "debug")]
        return;

        #[cfg(not(feature = "debug"))]
        todo!("naki")
    }
    // pub fn can_draw_tile(&self) -> bool {
    //     self.remain > GameRule.ace_remain
    // }
}

pub fn game_plugin(app: &mut App) {
    app.insert_resource(Game { ..default() })
        .init_state::<InGameState>()
        .add_plugins(ui_plugin)
        .add_systems(OnEnter(GameState::Game), setup_game)
        .add_systems(OnEnter(InGameState::GeneralUI), prepare_game)
        .add_systems(OnEnter(InGameState::SelfTsumo), game_tsumo)
        .add_systems(OnEnter(InGameState::RightTsumo), game_tsumo)
        .add_systems(OnEnter(InGameState::AcrossTsumo), game_tsumo)
        .add_systems(OnEnter(InGameState::LeftTsumo), game_tsumo)
        .add_systems(OnEnter(InGameState::LeftPlay), wait_player)
        .add_systems(OnEnter(InGameState::RightPlay), wait_player)
        .add_systems(OnEnter(InGameState::AcrossPlay), wait_player)
        .add_systems(Update, game_dahai.run_if(on_event::<Dahai>()));
}

fn wait_player(
    mut commands: Commands,
    game: ResMut<Game>,
    state: Res<State<InGameState>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    todo!("wait_player")
}

fn game_tsumo(mut game: ResMut<Game>, state: Res<State<InGameState>>, mut next_state: ResMut<NextState<InGameState>>) {
    let player = state2id!(state.get());
    let res = game.tsumo(player);
    next_state.set(game.ingamestate);
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ingamestate: ResMut<NextState<InGameState>>,
) {
    ingamestate.set(InGameState::GeneralUI);
}

fn prepare_game(mut game: ResMut<Game>, game_rule: Res<Rule>, mut next_state: ResMut<NextState<InGameState>>) {
    game.reset(game_rule);
    game.start_new_game();
    next_state.set(game.ingamestate)
}

fn game_dahai(
    mut game: ResMut<Game>,
    mut dahai: EventReader<Dahai>,
    state: Res<State<InGameState>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    // 只处理一次打牌
    for &Dahai { player, slot } in dahai.read() {
        game.dahai(player, slot);
        next_state.set(game.ingamestate);
        break;
    }
}
