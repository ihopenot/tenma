use std::{collections::HashMap, f32::consts::E};

use super::{enums::EnumGameState, state::GameState};

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
    SetScores([i32; 4]),
    ChangeState(EnumGameState),
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

    // every effect shoud be stackable
    fn stack(&mut self, other: &Effect) {
        assert!(self.effect_type == other.effect_type);

        match self.effect_type {
            EnumEffect::SetScores(_) | EnumEffect::ChangeState(_) => {
                panic!("Not stackable effect");
            }
            _ => {}
        }
    }

    fn apply(&self, game_state: &mut GameState) {
        match self.effect_type {
            EnumEffect::SetScores(scores) => {
                for i in 0..4 {
                    game_state.players[i].score = scores[i];
                }
            }
            EnumEffect::ChangeState(state) => {
                game_state.state = state;
            }

            EnumEffect::ActionDeny => {
                panic!("ActionDeny should not be applied directly");
            }
            EnumEffect::None => {}
            _ => {}
        }
    }
}

impl EffectSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_effect(&mut self, effect: Effect) {
        if effect.effect_type == EnumEffect::ActionDeny {
            assert!(effect.effect_level == EnumEffectLevel::Must);
        }

        if effect.effect_type != EnumEffect::None {
            let mut stacked = false;
            for e in &mut self.effects {
                if e.effect_type == effect.effect_type {
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
