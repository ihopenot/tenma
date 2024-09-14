use tenma::make_rulefor;

use super::{
    effect::{Effect, EnumEffect, EnumEffectLevel},
    enums::EnumGameState,
    rules::Rule,
    state::GameState,
    tile::Tile,
};

pub struct BasePreKyokuStartSetYama {
    pub name: &'static str,
    pub tiles: Vec<Tile>,
}
#[make_rulefor(BasePreKyokuStartSetYama)]
fn pass(&self, game_state: &GameState) -> Vec<Effect> {
    vec![Effect::new_base(EnumEffect::SetYama(self.tiles.clone()))]
}

// Game start rules
pub struct BasePreGameStartInitScores {
    pub name: &'static str,
    pub scores: [i32; 4],
}

#[make_rulefor(BasePreGameStartInitScores)]
fn pass(&self, game_state: &GameState) -> Vec<Effect> {
    vec![Effect::new_base(EnumEffect::SetScores(self.scores))]
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
fn pass(&self, game_state: &GameState) -> Vec<Effect> {
    vec![Effect::new(
        EnumEffect::ChangeState(EnumGameState::Tsumo),
        EnumEffectLevel::Must,
        100,
    )]
}

pub struct BasePreTsumoRandomTsumo {
    pub name: &'static str,
}
#[make_rulefor(BasePreTsumoRandomTsumo)]
fn pass(&self, game_state: &GameState) -> Vec<Effect> {
    vec![Effect::new_base(EnumEffect::RandomTsumo)]
}

pub struct BasePreTsumoStateCheck {
    pub name: &'static str,
}
#[make_rulefor(BasePreTsumoStateCheck)]
fn pass(&self, game_state: &GameState) -> Vec<Effect> {
    if game_state.state != EnumGameState::Tsumo
        || game_state.current_action.player != game_state.current_player
    {
        vec![Effect::deny()]
    } else {
        vec![Effect::none()]
    }
}
