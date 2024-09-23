use tenma::make_rulefor;

use super::{
    effect::{Effect, EnumEffect, EnumEffectLevel},
    enums::EnumGameState,
    rules::Rule,
    state::GameState,
    tile::Tile,
};

// ============== Game start rules ===============
pub struct BasePreGameStartInitScores {
    pub name: &'static str,
    pub scores: [i32; 4],
}

#[make_rulefor(BasePreGameStartInitScores)]
fn pass(&self, _: &GameState) -> Vec<Effect> {
    vec![Effect::new_base(EnumEffect::SetScores(self.scores))]
}

pub struct BasePostGameStartEnterKyoku {
    pub name: &'static str,
}
#[make_rulefor(BasePostGameStartEnterKyoku)]
fn pass(&self, _: &GameState) -> Vec<Effect> {
    vec![Effect::new_change_state(EnumGameState::KyokuStart)]
}

// =============== Kyoku start rules ================
pub struct BasePreKyokuStartSetYama {
    pub name: &'static str,
    pub tiles: Vec<Tile>,
}
#[make_rulefor(BasePreKyokuStartSetYama)]
fn pass(&self, _: &GameState) -> Vec<Effect> {
    vec![Effect::new_base(EnumEffect::SetYama(self.tiles.clone()))]
}

pub struct BasePreKyokuStartRevealDora {
    pub name: &'static str,
}
#[make_rulefor(BasePreKyokuStartRevealDora)]
fn pass(&self, _: &GameState) -> Vec<Effect> {
    vec![Effect::new_base(EnumEffect::RevealDora(1))]
}

pub struct BasePostKyokuStartEnterTsumo {
    pub name: &'static str,
}
#[make_rulefor(BasePostKyokuStartEnterTsumo)]
fn pass(&self, _: &GameState) -> Vec<Effect> {
    vec![Effect::new_change_state(EnumGameState::Tsumo)]
}

// ================= Tsumo rules ===================
pub struct BasePreTsumoRandomTsumo {
    pub name: &'static str,
}
#[make_rulefor(BasePreTsumoRandomTsumo)]
fn pass(&self, _: &GameState) -> Vec<Effect> {
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
