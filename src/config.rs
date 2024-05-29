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
    SelfPlay,
    RightPlay,
    AcrossPlay,
    LeftPlay,
    WaitNaki,
    End,
    #[default]
    Disabled,
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
pub struct Dahai {
    pub slot: u8,
    pub player: u8,
}