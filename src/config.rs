use bevy::prelude::*;
use derivative::Derivative;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    // Splash,
    Menu,
    Game,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InGameState {
    GeneralUI,
    GameObjectUI,
    SelfTsumo,
    RightTsumo,
    AcrossTsumo,
    LeftTsumo,
    SelfPlay,
    RightPlay,
    AcrossPlay,
    LeftPlay,
    SelfNaki,
    RightNaki,
    AcrossNaki,
    LeftNaki,
    End,
    #[default]
    Disabled,
}

pub const TSUMO_MAP: [InGameState; 4] = [
    InGameState::SelfTsumo,
    InGameState::RightTsumo,
    InGameState::AcrossTsumo,
    InGameState::LeftTsumo,
];

pub const PLAY_MAP: [InGameState; 4] = [
    InGameState::SelfPlay,
    InGameState::RightPlay,
    InGameState::AcrossPlay,
    InGameState::LeftPlay,
];

pub const NAKI_MAP: [InGameState; 4] = [
    InGameState::SelfNaki,
    InGameState::RightNaki,
    InGameState::AcrossNaki,
    InGameState::LeftNaki,
];

pub enum PlayerSeat {
    Selv,
    Right,
    Across,
    Left,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    // Settings,
    // SettingsDisplay,
    // SettingsSound,
    #[default]
    Disabled,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct DahaiTile;

#[derive(Component, Debug, Clone, Copy)]
pub struct TileClicked;

#[derive(Event, Debug)]
pub struct Dahai {
    pub player: u8,
    pub slot: u8,
}