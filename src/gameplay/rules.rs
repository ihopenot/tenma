use std::collections::HashMap;

use bevy::{log::tracing_subscriber::field::debug, prelude::*};

use super::{
    effect::{Effect, EnumEffect, EnumEffectLevel},
    state::GameState,
    tile::Tile,
};

#[derive(Default)]
pub struct RuleSet {
    pub gamestart_rules: Vec<Box<dyn Rule>>,  // 开局钩子
    pub kyokustart_rules: Vec<Box<dyn Rule>>, // 局开始钩子
    pub tsumo_rules: Vec<Box<dyn Rule>>,      // 自摸钩子
    pub dahai_rules: Vec<Box<dyn Rule>>,      // 打牌钩子
}

#[derive(Resource, Default)]
pub struct RuleDict {
    pub rules: HashMap<&'static str, Box<dyn Rule>>,
}
impl RuleDict {
    pub fn set_rule(&mut self, rule: Box<dyn Rule>) {
        self.rules.insert(rule.name(), rule);
    }
}

type RulePass = fn(&GameState) -> Effect;
pub trait Rule: Sync + Send {
    fn name(&self) -> &'static str;
    fn pass(&self, game_state: &GameState) -> Effect;
}

pub struct BaseRuleSetScores {
    pub name: &'static str,
    pub scores: [i32; 4],
}
impl Rule for BaseRuleSetScores {
    fn name(&self) -> &'static str {
        self.name
    }

    fn pass(&self, game_state: &GameState) -> Effect {
        let mut effect = Effect::new(EnumEffect::SetScores, EnumEffectLevel::Base, 100);
        effect.effect_data.set_scores = self.scores;
        effect
    }
}
