use std::collections::HashMap;

use bevy::prelude::*;

use super::{
    effect::{Effect, EffectSet, EnumEffect, EnumEffectLevel},
    enums::{EnumRulePhase, Error},
    state::GameState,
    tile::Tile,
};

#[derive(Default)]
pub struct RuleSet {
    pub pre_gamestart_rules: Vec<Box<dyn Rule>>,   // 开局钩子
    pub post_gamestart_rules: Vec<Box<dyn Rule>>,  // 开局钩子
    pub pre_kyokustart_rules: Vec<Box<dyn Rule>>,  // 局开始钩子
    pub post_kyokustart_rules: Vec<Box<dyn Rule>>, // 局开始钩子
    pub pre_tsumo_rules: Vec<Box<dyn Rule>>,       // 自摸钩子
    pub post_tsumo_rules: Vec<Box<dyn Rule>>,      // 自摸钩子
    pub pre_dahai_rules: Vec<Box<dyn Rule>>,       // 打牌钩子
    pub post_dahai_rules: Vec<Box<dyn Rule>>,      // 打牌钩子
}

impl RuleSet {
    pub fn collect_effects(
        &self,
        phase: EnumRulePhase,
        state: &GameState,
    ) -> Result<EffectSet, Error> {
        let mut effects = EffectSet::new();
        let rules = match phase {
            EnumRulePhase::PreGameStart => &self.pre_gamestart_rules,
            EnumRulePhase::PostGameStart => &self.post_gamestart_rules,
            EnumRulePhase::PreKyokuStart => &self.pre_kyokustart_rules,
            EnumRulePhase::PostKyokuStart => &self.post_kyokustart_rules,
            EnumRulePhase::PreTsumo => &self.pre_tsumo_rules,
            EnumRulePhase::PostTsumo => &self.post_tsumo_rules,
            EnumRulePhase::PreDahai => &self.pre_dahai_rules,
            EnumRulePhase::PostDahai => &self.post_dahai_rules,
            _ => return Err(Error::PhaseNotSupported),
        };
        for r in rules {
            effects.add_effects(r.pass(state));
        }
        Ok(effects)
    }
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

pub trait Rule: Sync + Send {
    fn name(&self) -> &'static str;
    fn pass(&self, game_state: &GameState) -> Vec<Effect>;
}
