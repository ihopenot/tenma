use crate::config::{Dahai, DahaiTile, GameState, InGameState, TileClicked};
use crate::game::{self, Game};
use crate::resource::GameTextures;
use bevy::ecs::query;
use bevy::transform::commands;
use bevy::{prelude::*, transform};
use bevy_mod_picking::prelude::*;

const TILE_WIDTH: f32 = 90.0;
const TILE_HEIGHT: f32 = 102.0;
const TILE_SCALE: f32 = 0.7;
const WB_WIDTH: f32 = 400.0;
const WB_HEIGHT: f32 = 400.0;
const WB_SCALE: f32 = 0.5;

#[derive(Component)]
struct GameUI;

#[derive(Component)]
struct WindBoard;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(OnEnter(InGameState::GeneralUI), setup_general_game_ui)
        .add_systems(OnEnter(InGameState::GameObjectUI), setup_gameobject_ui)
        .add_systems(
            Update,
            (
                game_dahai.run_if(
                    in_state(InGameState::SelfPlay).and_then(any_with_component::<DahaiTile>),
                ),
                handle_tile_click,
            ),
        )
        .add_event::<Dahai>();
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
                scale: Vec3::splat(WB_SCALE),
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
    pub player: u8,
    pub slot: u8,
}

fn setup_gameobject_ui(
    mut commands: Commands,
    game_texture: Res<GameTextures>,
    game: Res<Game>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    let self_status = &game.status[game.self_id as usize];
    for i in 0..14 {
        commands.spawn((
            SpriteBundle {
                texture: game_texture.tile[self_status.tehai[i] as usize].clone(),
                transform: Transform {
                    scale: Vec3::splat(TILE_SCALE),
                    translation: Vec3::new(
                        TILE_WIDTH * TILE_SCALE * 0.8 * (i as f32 - 7.5),
                        -200.0,
                        i as f32,
                    ),
                    ..default()
                },
                ..default()
            },
            TileBind {
                player: game.self_id,
                slot: i as u8,
            },
            PickableBundle::default(),
            On::<Pointer<Over>>::target_component_mut::<Transform>(|_, transform| {
                transform.translation.y += TILE_HEIGHT * TILE_SCALE * 0.3;
            }),
            On::<Pointer<Out>>::target_component_mut::<Transform>(|_, transform| {
                transform.translation.y -= TILE_HEIGHT * TILE_SCALE * 0.3;
            }),
            On::<Pointer<Click>>::target_insert(TileClicked),
            // On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            //     transform.translation += Vec3::new(drag.delta.x, -drag.delta.y, 0.0);
            // }),
            // On::<Pointer<Drop>>::commands_mut(|event, commands| {
            //     println!("{:?}", event.pointer_location);
            // }),
        ));
    }

    next_state.set(match (game.kyoku - game.self_id) % 4 {
        0 => InGameState::SelfPlay,
        1 => InGameState::LeftPlay,
        2 => InGameState::AcrossPlay,
        3 => InGameState::RightPlay,
        _ => {
            assert!(false, "unreachable!");
            InGameState::Disabled
        }
    })
}

//TODO: 打牌UI变化
fn game_dahai(mut dahaiwriter: EventWriter<Dahai>, query: Query<&TileBind, With<DahaiTile>>) {
    let tilebind = query.single();
    dahaiwriter.send(Dahai {
        player: tilebind.player,
        slot: tilebind.slot,
    });
}

fn handle_tile_click(
    query: Query<Entity, With<TileClicked>>,
    mut commands: Commands,
    state: Res<State<InGameState>>,
) {
    // 一帧只处理一个点击事件
    let mut first = true;
    for entity in query.iter() {
        commands.entity(entity).remove::<TileClicked>();
        if !first {
            continue;
        }

        match state.get() {
            InGameState::SelfPlay => {
                println!("add DahaiTile");
                commands.entity(entity).insert(DahaiTile);
            }
            _ => {}
        }
        first = false;
    }
}
