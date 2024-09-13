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
    Effect::new(
        EnumEffect::ChangeState(EnumGameState::Tsumo),
        EnumEffectLevel::Must,
        100,
    )
}

pub struct BasePreTsumoRandomTsumo {
    pub name: &'static str,
}
#[make_rulefor(BasePreTsumoRandomTsumo)]
fn pass(&self, game_state: &GameState) -> Effect {
    Effect::new_base(EnumEffect::RandomTsumo)
}

pub struct BasePreTsumoStateCheck {
    pub name: &'static str,
}
#[make_rulefor(BasePreTsumoStateCheck)]
fn pass(&self, game_state: &GameState) -> Effect {
    if game_state.state != EnumGameState::Tsumo
        || game_state.current_action.player != game_state.current_player
    {
        Effect::deny()
    } else {
        Effect::none()
    }
}
