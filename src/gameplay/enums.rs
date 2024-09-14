use bevy::prelude::default;

#[derive(Debug)]
pub enum Error {
    RuleNotSet,
    ActionNotSupported,
    PhaseNotSupported,
    RuleDenied,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum EnumGameState {
    #[default]
    GameStart,
    KyokuStart,
    Tsumo,
    Dahai,
    Naki,
    KyokuEnd,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum EnumTsumoType {
    #[default]
    Yama,
    Rinshan,
}

#[derive(PartialEq, Clone, Copy)]
pub enum EnumRulePhase {
    PreGameStart,
    PostGameStart,
    PreKyokuStart,
    PostKyokuStart,
    PreTsumo,
    PostTsumo,
    PreDahai,
    PostDahai,
}
