use bevy::prelude::*;
use derivative::Derivative;

use super::{
    action::{Action, EnumAction},
    effect::EffectSet,
    enums::{EnumGameState, EnumRulePhase, Error},
    rules::RuleSet,
    tile::Tile,
};
use crate::{gameplay::enums::EnumTsumoType, tuid};

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

#[derive(Resource)]
pub struct GameState {
    pub kyoku: u8, // 局, 半庄[0, 7]
    pub honba: u8, // 本场, [0, -]
    pub bakaze: u8,

    pub dora: [Tile; 4],
    pub uradora: [Tile; 4],
    pub yama: Vec<Tile>,

    pub last_action: Action,

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
    pub fn new(rule: Option<RuleSet>) -> Self {
        let mut ret = Self {
            rule: None,
            kyoku: 0,
            honba: 0,
            bakaze: 0,
            dora: [Tile::unkown(); 4],
            uradora: [Tile::unkown(); 4],
            last_action: Default::default(),
            yama: Default::default(),
            remain: 0,
            players: Default::default(),
            current_player: 0,
            state: Default::default(),
        };
        ret.load_rule(rule);
        ret
    }

    pub fn load_rule(&mut self, rule: Option<RuleSet>) {
        self.rule = rule;
    }

    fn collect_effects(&self, phase: EnumRulePhase) -> Result<EffectSet, Error> {
        let mut effects = EffectSet::new();
        if let Some(rule) = &self.rule {
            match phase {
                EnumRulePhase::PreGameStart => {
                    for r in &rule.gamestart_rules {
                        effects.add_effect(r.pass(self));
                    }
                }
                EnumRulePhase::PreKyokuStart => {
                    for r in &rule.kyokustart_rules {
                        effects.add_effect(r.pass(self));
                    }
                }
                EnumRulePhase::PreTsumo => {
                    for r in &rule.tsumo_rules {
                        effects.add_effect(r.pass(self));
                    }
                }
                EnumRulePhase::PreDahai => {
                    for r in &rule.dahai_rules {
                        effects.add_effect(r.pass(self));
                    }
                }
                _ => return Err(Error::PhaseNotSupported),
            }
        }
        Ok(effects)
    }

    pub fn step_action(&mut self, action: Action) -> Result<(), Error> {
        if self.rule.is_none() {
            return Err(Error::RuleNotSet);
        }

        match action.act_type {
            EnumAction::GameStart => {
                let mut effects = self.collect_effects(EnumRulePhase::PreGameStart).unwrap();
                if effects.with_deny() {
                    return Err(Error::RuleDenied);
                }
                effects.apply(self);
            }
            _ => return Err(Error::ActionNotSupported),
        };
        self.last_action = action;

        Ok(())
    }
}
