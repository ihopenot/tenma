use bevy::prelude::default;

#[derive(Default, PartialEq)]
pub enum EnumEffectLevel {
    #[default]
    Base,
    Low,
    Middle,
    High,
    Must,
}

#[derive(Default)]
pub struct Effect {
    pub effect_type: EnumEffect,
    pub effect_data: EffectData,
    pub effect_level: EnumEffectLevel,
    pub effect_prob: u8, // 0 - 100
}

pub struct EffectSet {
    pub effects: Vec<Effect>,
}

#[derive(Default)]
pub enum EnumEffect {
    // start game
    SetScores,
    #[default]
    None,
}

union EffectData {
    pub set_scores: [i32; 4],
    pub none: (),
}

impl Default for EffectData {
    fn default() -> Self {
        Self { none: () }
    }
}

impl Effect {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn new(effect_type: EnumEffect, effect_level: EnumEffectLevel, effect_prob: u8) -> Self {
        Self {
            effect_type,
            effect_level,
            effect_prob,
            ..Default::default()
        }
    }
}
