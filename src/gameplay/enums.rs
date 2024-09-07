use bevy::prelude::default;

pub enum Error {
    InvalidRuleSet,
}

#[derive(Default, PartialEq)]
pub enum EnumGameState {
    #[default]
    NotStarted,
    Tsumo,
    Dahai,
    Naki,
    WaitNewKyoku,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum TsumoType {
    #[default]
    Yama,
    Rinshan,
}
