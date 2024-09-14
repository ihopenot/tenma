use std::{collections::HashMap, f32::consts::E};

use crate::{config::Tsumo, gameplay::action::EnumAction};

use super::{action::Action, enums::EnumGameState, state::GameState, tile::Tile};

#[derive(Default, PartialEq, Ord, Eq, PartialOrd)]
pub enum EnumEffectLevel {
    #[default]
    Base,
    Low,
    Middle,
    High,
    ToOther,
    ToSelf,
    Must,
}

#[derive(Default)]
pub struct Effect {
    pub effect_type: EnumEffect,
    pub effect_prob: u8, // 0 - 100
    pub effect_level: EnumEffectLevel,
}

#[derive(Default)]
pub struct EffectSet {
    pub effects: Vec<Effect>,
}

#[derive(Default, PartialEq)]
pub enum EnumEffect {
    ActionDeny, // deny an action, must be level MUST

    // start game
    SetYama(Vec<Tile>),
    SetScores([i32; 4]),
    ChangeState(EnumGameState),

    // tsumo
    RandomTsumo,
    #[default]
    None,
}

impl Effect {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn new(effect_type: EnumEffect, effect_level: EnumEffectLevel, effect_prob: u8) -> Self {
        Self {
            effect_type,
            effect_prob,
            effect_level,
            ..Default::default()
        }
    }

    // base rule
    pub fn new_base(effect_type: EnumEffect) -> Self {
        Self::new(effect_type, EnumEffectLevel::Base, 100)
    }

    pub fn deny() -> Self {
        Self::new(EnumEffect::ActionDeny, EnumEffectLevel::Must, 100)
    }

    // every effect shoud be stackable
    fn stack(&mut self, other: &Effect) {
        assert!(self.effect_type == other.effect_type);

        match self.effect_type {
            EnumEffect::SetScores(_)
            | EnumEffect::ChangeState(_)
            | EnumEffect::RandomTsumo
            | EnumEffect::SetYama(_) => {
                panic!("Not stackable effect");
            }
            EnumEffect::ActionDeny | EnumEffect::None => {}
        }
    }

    fn apply(&self, game_state: &mut GameState) {
        match &self.effect_type {
            EnumEffect::SetYama(tiles) => {
                game_state.yama = tiles.clone();
            }
            EnumEffect::SetScores(scores) => {
                for i in 0..4 {
                    game_state.players[i].score = scores[i];
                }
            }
            EnumEffect::ChangeState(state) => {
                game_state.state = state.clone();
                match state {
                    EnumGameState::Tsumo => {
                        game_state.next_action = Some(Action::new(
                            game_state.current_player,
                            EnumAction::Tsumo,
                            Tile::unkown(),
                            vec![],
                        ));
                    }
                    _ => {
                        panic!("Invalid state change");
                    }
                }
            }

            EnumEffect::RandomTsumo => {
                assert!(game_state.current_action.act_type == EnumAction::Tsumo);
                game_state.current_action.act_tile = game_state.try_draw_random_tile();
            }

            EnumEffect::ActionDeny => {
                panic!("ActionDeny should not be applied directly");
            }
            EnumEffect::None => {}
        }
    }
}

impl EffectSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_effects(&mut self, mut effects: Vec<Effect>) {
        while effects.len() > 0 {
            let e = effects.pop().unwrap();
            self.add_effect(e);
        }
    }

    pub fn add_effect(&mut self, effect: Effect) {
        if effect.effect_type == EnumEffect::ActionDeny {
            assert!(effect.effect_level == EnumEffectLevel::Must);
        }

        if effect.effect_type != EnumEffect::None {
            let mut stacked = false;
            for e in &mut self.effects {
                // only stack same type and same level effect
                if e.effect_type == effect.effect_type && e.effect_level == effect.effect_level {
                    e.stack(&effect);
                    stacked = true;
                    break;
                }
            }
            if !stacked {
                self.effects.push(effect);
            }
        }
    }

    pub fn with_deny(&self) -> bool {
        self.effects
            .iter()
            .any(|e| e.effect_type == EnumEffect::ActionDeny)
    }

    pub fn apply(&mut self, game_state: &mut GameState) {
        self.effects
            .sort_by(|a, b| a.effect_level.cmp(&b.effect_level));
        self.effects.iter().for_each(|e| e.apply(game_state));
    }
}
