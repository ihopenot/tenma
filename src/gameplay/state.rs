use bevy::prelude::*;
use derivative::Derivative;

use super::{
    enums::{EnumGameState, Error},
    rules::RuleSet,
    tile::Tile,
};
use crate::tuid;

#[derive(Resource)]
pub struct GameState {
    pub kyoku: u8, // 局
    pub honba: u8, // 本场
    pub bakaze: u8,

    pub dora: [Tile; 4],
    pub uradora: [Tile; 4],
    pub yama: Vec<Tile>,

    pub remain: u8,

    pub players: [PlayerStatus; 4],
    pub current_player: u8,
    pub state: EnumGameState,

    pub rule: Option<RuleSet>,
}

#[derive(Default)]
pub struct PlayerStatus {
    pub score: i32,
    pub jikaze: u8,

    pub tehai: Vec<Tile>,
    pub last_tsumo: Tile,
    pub furo: Vec<[Tile; 4]>,
}

impl PlayerStatus {}

impl GameState {
    pub fn new(rule: &RuleSet) -> Self {
        let mut ret = Self {
            rule: None,
            kyoku: 0,
            honba: 0,
            bakaze: 0,
            dora: [Tile::unkown(); 4],
            uradora: [Tile::unkown(); 4],
            yama: Default::default(),
            remain: 0,
            players: Default::default(),
            current_player: 0,
            state: Default::default(),
        };
        ret.load_rule(rule);
        ret
    }

    pub fn load_rule(&mut self, rule: &RuleSet) {
        self.rule = Some(rule.clone());

        let mut yama = Vec::new();
        for tile in rule.init_rule.get_tile_set() {
            yama.push(tile.clone());
        }
        let remain = yama.len() as u8;
    }

    pub fn start_game(&mut self) {
        assert!(self.state == EnumGameState::NotStarted);

        self.kyoku = 0;
        self.honba = 0;
        self.bakaze = 0;
        self.current_player = 0;
        self.state = EnumGameState::WaitNewKyoku;
        self.start_kyoku();
    }
}
