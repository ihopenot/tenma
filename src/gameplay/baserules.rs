use tenma::make_rulefor;

use super::{
    effect::{Effect, EnumEffect, EnumEffectLevel},
    enums::EnumGameState,
    rules::Rule,
    state::GameState,
};

// Game start rules
pub struct BasePreGameStartInitScores {
    pub name: &'static str,
    pub scores: [i32; 4],
}

#[make_rulefor(BasePreGameStartInitScores)]
fn pass(&self, game_state: &GameState) -> Effect {
    Effect::new_base(EnumEffect::SetScores(self.scores))
}
// impl Rule for BaseGameStartInitScores {
//     fn name(&self) -> &'static str {
//         self.name
//     }

//     fn pass(&self, game_state: &GameState) -> Effect {
//         Effect::new_base(EnumEffect::SetScores(self.scores))
//     }
// }

pub struct BasePostGameStartEnterTsumo {
    pub name: &'static str,
}
#[make_rulefor(BasePostGameStartEnterTsumo)]
fn pass(&self, game_state: &GameState) -> Effect {
    Effect::new_base(EnumEffect::ChangeState(EnumGameState::Tsumo))
}
