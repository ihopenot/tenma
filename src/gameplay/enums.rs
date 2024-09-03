pub enum Error {
    InvalidRuleSet,
}

#[derive(Default, PartialEq)]
pub enum EnumGameState {
    #[default]
    NotStarted,
    InKyoku,
    WaitNewKyoku,
}
