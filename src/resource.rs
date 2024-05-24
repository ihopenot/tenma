use bevy::prelude::*;
use derivative::Derivative;

#[derive(Resource)]
pub struct GameTextures {
    pub background: Handle<Image>,
    pub tile: [Handle<Image>; 37],
    pub windboard: Handle<Image>,
    // pub explosion: Handle<TextureAtlas>,
    // pub font: Handle<Font>,
}

#[derive(Derivative, Resource)]
#[derivative(Default)]
pub struct Rule {
    #[derivative(Default(value = "14"))]
    pub ace_remain: u8,
    #[derivative(Default(value = "true"))]
    pub akaari: bool,
}