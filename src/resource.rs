use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTextures {
    pub background: Handle<Image>,
    pub tile: [Handle<Image>; 37],
    pub windboard: Handle<Image>,
    // pub explosion: Handle<TextureAtlas>,
    // pub font: Handle<Font>,
}
