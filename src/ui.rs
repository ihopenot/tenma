use crate::config::{Dahai, GameState, InGameState};
use crate::game::{self, Game};
use crate::resource::GameTextures;
use bevy::transform::commands;
use bevy::{prelude::*, transform};
use bevy_mod_picking::prelude::*;

const tile_width: f32 = 90.0;
const tile_height: f32 = 102.0;
const tile_scale: f32 = 0.7;
const windboard_width: f32 = 400.0;
const windboard_height: f32 = 400.0;
const windboard_scale: f32 = 0.5;

#[derive(Component)]
struct GameUI;

#[derive(Component)]
struct WindBoard;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(OnEnter(InGameState::GeneralUI), setup_general_game_ui)
        .add_systems(OnEnter(InGameState::GameObjectUI), setup_gameobject_ui);
}

fn setup_general_game_ui(
    mut commands: Commands,
    game_texture: Res<GameTextures>,
    mut ingamestate: ResMut<NextState<InGameState>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: game_texture.windboard.clone(),
            transform: Transform {
                scale: Vec3::splat(windboard_scale),
                rotation: Quat::from_rotation_z(0.0),
                ..default()
            },
            ..default()
        },
        WindBoard,
    ));
    ingamestate.set(InGameState::GameObjectUI);
}

#[derive(Component)]
struct TileBind {
    pub val: u8,
    pub slot: u8,
}

fn setup_gameobject_ui(
    mut commands: Commands,
    game_texture: Res<GameTextures>,
    game: Res<Game>,
    mut ingamestate: ResMut<NextState<InGameState>>,
) {
    let self_status = &game.status[game.self_id as usize];
    for i in 0..14 {
        commands.spawn((
            SpriteBundle {
                texture: game_texture.tile[self_status.tehai[i] as usize].clone(),
                transform: Transform {
                    scale: Vec3::splat(tile_scale),
                    translation: Vec3::new(
                        tile_width * tile_scale * 0.8 * (i as f32 - 7.5),
                        -200.0,
                        i as f32,
                    ),
                    ..default()
                },
                ..default()
            },
            TileBind {
                val: self_status.tehai[i],
                slot: i as u8,
            },
            PickableBundle::default(),
            On::<Pointer<Over>>::target_component_mut::<Transform>(|_, transform| {
                transform.translation.y += tile_height * tile_scale * 0.3;
            }),
            On::<Pointer<Out>>::target_component_mut::<Transform>(|_, transform| {
                transform.translation.y -= tile_height * tile_scale * 0.3;
            }),
            On::<Pointer<Click>>::target_insert(Dahai{
                slot: i as u8,
                player: game.self_id,
            }),
            // On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            //     transform.translation += Vec3::new(drag.delta.x, -drag.delta.y, 0.0);
            // }),
            // On::<Pointer<Drop>>::commands_mut(|event, commands| {
            //     println!("{:?}", event.pointer_location);
            // }),
        ));
    }
}
