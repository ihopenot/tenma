use crate::config::{
    Clean, Dahai, DahaiTile, GameState, InGameState, PlayerSeat, TileClicked, Tsumo,
};
use crate::game::{self, Game, GameError};
use crate::resource::GameTextures;
use crate::{id2loc, tu8};
use bevy::ecs::{entity, query, world};
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
        .add_event::<Dahai>()
        .add_event::<Tsumo>()
        .add_event::<Clean>()
        .add_systems(Update, ui_dahai.run_if(on_event::<Dahai>()))
        .add_systems(Update, ui_tsumo.run_if(on_event::<Tsumo>()))
        .add_systems(Update, ui_clean.run_if(on_event::<Clean>()));
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

#[derive(Component, PartialEq)]
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
        if self_status.tehai[i] != tu8!(-) {
            spawn_to_pos(
                &mut commands,
                SpriteBundle {
                    texture: game_texture.tile[self_status.tehai[i] as usize].clone(),
                    transform: Transform {
                        translation: get_tile_translation(PlayerSeat::Selv, i as u8),
                        scale: Vec3::splat(TILE_SCALE),
                        ..default()
                    },
                    ..default()
                },
                TileBind {
                    player: game.self_id,
                    slot: i as u8,
                },
            );
        }
    }

    next_state.set(match (game.kyoku - game.self_id) % 4 {
        0 => InGameState::SelfTsumo,
        1 => InGameState::LeftTsumo,
        2 => InGameState::AcrossTsumo,
        3 => InGameState::RightTsumo,
        _ => {
            assert!(false, "unreachable!");
            InGameState::Disabled
        }
    })
}

//TODO: 打牌UI变化
fn game_dahai(
    mut commands: Commands,
    mut dahaiwriter: EventWriter<Dahai>,
    query: Query<(Entity, &TileBind), With<DahaiTile>>,
) {
    let (entity, tilebind) = query.single();
    dahaiwriter.send(Dahai {
        player: tilebind.player,
        slot: tilebind.slot,
    });
    commands.entity(entity).remove::<DahaiTile>();
}

#[derive(Debug)]
pub enum UIError {
    NoSuchTile,
}

fn move_tile_to_slot(commands: &mut Commands, entity: Entity, player: u8, slot: u8) {
    commands.entity(entity).insert(TileBind { player, slot });
    commands.entity(entity).insert(Transform {
        translation: get_tile_translation(id2loc!(player), slot),
        scale: Vec3::splat(TILE_SCALE),
        ..default()
    });
}

fn ui_clean(
    mut commands: Commands,
    mut cleanreader: EventReader<Clean>,
    query: Query<(Entity, &TileBind), With<TileBind>>,
) {
    assert!(cleanreader.len() == 1);
    for &Clean { player, slot } in cleanreader.read() {
        println!("cleaning {} {}", player, slot);
        for (entity, tb) in query.iter() {
            if tb == &(TileBind { player, slot: 13 }) {
                println!("clean tsumo found");
                move_tile_to_slot(&mut commands, entity, player, slot);
            }
        }
    }
}

fn ui_dahai(
    mut commands: Commands,
    mut dahaireader: EventReader<Dahai>,
    mut cleanwriter: EventWriter<Clean>,
    query: Query<(Entity, &TileBind), With<TileBind>>,
) {
    assert!(dahaireader.len() == 1);
    for &Dahai { player, slot } in dahaireader.read() {
        for (entity, tb) in query.iter() {
            if tb == &(TileBind { player, slot }) {
                commands.entity(entity).despawn();
            }
        }
        cleanwriter.send(Clean { player, slot });
        println!("clean send {} {}", player, slot)
    }
}

fn ui_tsumo(
    mut commands: Commands,
    game_texture: Res<GameTextures>,
    mut tsumoreader: EventReader<Tsumo>,
) {
    assert!(tsumoreader.len() == 1);
    for &Tsumo { player, tile } in tsumoreader.read() {
        spawn_to_pos(
            &mut commands,
            SpriteBundle {
                texture: game_texture.tile[tile as usize].clone(),
                transform: Transform {
                    translation: get_tile_translation(id2loc!(player), 13),
                    scale: Vec3::splat(TILE_SCALE),
                    ..default()
                },
                ..default()
            },
            TileBind { player, slot: 13 },
        );
    }
}

fn get_tile_translation(player_loc: PlayerSeat, slot: u8) -> Vec3 {
    match player_loc {
        PlayerSeat::Selv => Vec3::new(
            TILE_WIDTH * TILE_SCALE * 0.8 * (slot as f32 - 7.5),
            -200.0,
            slot as f32,
        ),
        PlayerSeat::Across => Vec3::new(
            TILE_WIDTH * TILE_SCALE * 0.8 * (slot as f32 - 7.5),
            200.0,
            slot as f32,
        ),
        PlayerSeat::Left => Vec3::new(
            -200.0,
            TILE_HEIGHT * TILE_SCALE * 0.8 * (slot as f32 - 7.5),
            slot as f32,
        ),
        PlayerSeat::Right => Vec3::new(
            200.0,
            TILE_HEIGHT * TILE_SCALE * 0.8 * (slot as f32 - 7.5),
            slot as f32,
        ),
    }
}

fn spawn_to_pos(commands: &mut Commands, sprite: SpriteBundle, bind: TileBind) {
    commands.spawn((
        sprite,
        bind,
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
