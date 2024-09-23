use bevy::prelude::*;
use derivative::Derivative;
use rand::seq::SliceRandom;

use super::{
    action::{Action, EnumAction},
    effect::EffectSet,
    enums::{EnumGameState, EnumRulePhase, Error},
    rules::RuleSet,
    tile::Tile,
};
use crate::{act2postphase, act2prephase, gameplay::enums::EnumTsumoType, tu8};

macro_rules! kyoku2bakaze {
    ($v:expr) => {
        $v % 4
    };
}

macro_rules! kyoku2player {
    ($v:expr) => {
        $v % 4
    };
}

#[derive(Default, Resource, Clone)]
pub struct GameState {
    pub kyoku: u8, // 局, 半庄[0, 7]
    pub honba: u8, // 本场, [0, -]
    pub bakaze: u8,

    pub dora: Vec<Tile>,
    pub uradora: Vec<Tile>,
    pub yama: Vec<Tile>,

    pub last_action: Action,
    pub current_action: Action,
    pub next_action: Action,

    pub remain: u8,

    pub players: [PlayerStatus; 4],
    pub current_player: u8,
    pub state: EnumGameState,
}

#[derive(Resource)]
pub struct GameRules {
    pub rule: Option<RuleSet>,
}

#[derive(Clone)]
pub struct DeltaGameState {
    pub tsumo_tile: Option<Tile>,
    pub new_dora: Option<Vec<Tile>>,
    pub add_yama: Option<Vec<Tile>>,
    pub remove_yama: Option<Vec<Tile>>,
    pub set_score: Option<[i32; 4]>,
    pub next_action: Option<Action>,
    pub next_state: Option<EnumGameState>,
}

impl Default for DeltaGameState {
    fn default() -> Self {
        Self {
            tsumo_tile: None,
            new_dora: None,
            add_yama: None,
            remove_yama: None,
            set_score: None,
            next_action: None,
            next_state: None,
        }
    }
}

#[derive(Default, Clone)]
pub struct PlayerStatus {
    pub score: i32,
    pub jikaze: u8,

    pub tehai: Vec<Tile>,
    pub last_tsumo: Tile,
    pub furo: Vec<[Tile; 4]>,
}

impl GameRules {
    pub fn load_rule(&mut self, rule: Option<RuleSet>) {
        self.rule = rule;
    }
}

impl GameState {
    pub fn new() -> Self {
        self::default()
    }

    pub fn step_action(&mut self, action: Action, rules: &RuleSet) -> Result<(), Error> {
        self.next_action = Action::none();
        self.current_action = action;

        let mut pre_effects = rules
            .collect_effects(act2prephase!(self.current_action), self)
            .unwrap();
        if pre_effects.with_deny() {
            return Err(Error::RuleDenied);
        }
        pre_effects.apply(self);

        let mut post_effects = rules
            .collect_effects(act2postphase!(self.current_action), self)
            .unwrap();
        if post_effects.with_deny() {
            return Err(Error::RuleDenied);
        }
        post_effects.apply(self);

        self.last_action = self.current_action.clone();
        self.current_action = Action::none();

        if self.next_action.act_type != EnumAction::None {
            self.step_action(self.next_action.clone(), rules)
        } else {
            Ok(())
        }
    }

    pub fn try_draw_random_tile(&self) -> Tile {
        let mut rng = rand::thread_rng();
        self.yama.choose(&mut rng).unwrap().clone()
    }

    pub fn apply_delta(&self, delta: DeltaGameState) -> Result<GameState, Error> {
        let mut new_state = self.clone();

        if delta.tsumo_tile.is_some() {
            new_state.players[self.current_player as usize].tsumo(delta.tsumo_tile.unwrap());
        }

        if delta.new_dora.is_some() {
            new_state.dora.extend(delta.new_dora.unwrap());
        }

        if delta.add_yama.is_some() {
            new_state.yama.extend(delta.add_yama.unwrap());
        }

        if delta.remove_yama.is_some() {
            for tile in delta.remove_yama.unwrap() {
                new_state.yama.retain(|x| x != &tile);
            }
        }

        if delta.set_score.is_some() {
            for i in 0..4 {
                new_state.players[i].score = delta.set_score.unwrap()[i];
            }
        }

        if delta.next_action.is_some() {
            new_state.current_action = delta.next_action.unwrap();
        }

        if delta.next_state.is_some() {
            new_state.state = delta.next_state.unwrap();
        }

        Ok(new_state)
    }
}

impl PlayerStatus {
    pub fn tsumo(&mut self, tile: Tile) {
        self.tehai.push(tile);
        self.last_tsumo = tile;
    }
}
