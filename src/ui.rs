use crate::config::{
    Clean, Dahai, DahaiTile, ProgramState, InGameState, PlayerSeat, TehaiPos, TileBind, TileClicked,
    Tsumo, TSUMO_SLOT,
};
use crate::game::{self, GameError, GameState};
use crate::resource::GameTextures;
use crate::{id2loc, tu8, tuz};
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

fn setup_gameobject_ui(
    mut commands: Commands,
    game_texture: Res<GameTextures>,
    game: Res<ProgramState>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    let self_status = &game.players[game.self_id as usize];
    let mut slot: u8 = 0;
    for i in 0..tuz!(all) {
        for _ in 0..self_status.tehai[i] {
            spain_tehai_to_pos(
                &mut commands,
                SpriteBundle {
                    texture: game_texture.tile[i].clone(),
                    ..default()
                },
                TileBind {
                    player: game.self_id,
                    tile: i as u8,
                },
                TehaiPos {
                    seat: PlayerSeat::Selv,
                    slot,
                },
            );
            slot += 1;
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
    query: Query<(Entity, &TileBind, &TehaiPos), With<DahaiTile>>,
) {
    let (entity, tilebind, pos) = query.single();
    commands.entity(entity).remove::<DahaiTile>();
    if pos.seat != PlayerSeat::Selv {
        println!("not self dahai");
        return;
    }
    dahaiwriter.send(Dahai {
        bind: tilebind.clone(),
        pos: pos.clone(),
    });
}

#[derive(Debug)]
pub enum UIError {
    NoSuchTile,
}

fn move_tehai_to_slot(commands: &mut Commands, entity: Entity, pos: &TehaiPos) {
    commands.entity(entity).insert(pos.clone());
    commands.entity(entity).insert(Transform {
        translation: get_tehai_translation(pos),
        scale: Vec3::splat(TILE_SCALE),
        ..default()
    });
}

fn ui_clean(
    mut commands: Commands,
    mut cleanreader: EventReader<Clean>,
    query: Query<(Entity, &TehaiPos), With<TehaiPos>>,
) {
    assert!(cleanreader.len() == 1);
    for &Clean { pos } in cleanreader.read() {
        for (entity, tp) in query.iter() {
            if tp
                == &(TehaiPos {
                    seat: pos.seat,
                    slot: TSUMO_SLOT,
                })
            {
                move_tehai_to_slot(&mut commands, entity, &pos);
            }
        }
    }
}

fn ui_dahai(
    mut commands: Commands,
    mut dahaireader: EventReader<Dahai>,
    mut cleanwriter: EventWriter<Clean>,
    query: Query<(Entity, &TehaiPos), With<TehaiPos>>,
) {
    assert!(dahaireader.len() == 1);
    for &Dahai { bind, pos } in dahaireader.read() {
        for (entity, tp) in query.iter() {
            if tp == &pos {
                commands.entity(entity).despawn();
            }
        }
        if pos.slot != TSUMO_SLOT {
            cleanwriter.send(Clean { pos: pos.clone() });
            println!("clean send {} {}", pos.seat as u8, pos.slot);
        }
    }
}

fn ui_tsumo(
    mut commands: Commands,
    game_texture: Res<GameTextures>,
    mut tsumoreader: EventReader<Tsumo>,
) {
    assert!(tsumoreader.len() == 1);
    for &Tsumo { player, tile } in tsumoreader.read() {
        spain_tehai_to_pos(
            &mut commands,
            SpriteBundle {
                texture: game_texture.tile[tile as usize].clone(),
                ..default()
            },
            TileBind { player, tile },
            TehaiPos {
                seat: id2loc!(player),
                slot: TSUMO_SLOT,
            },
        );
    }
}

fn get_tehai_translation(pos: &TehaiPos) -> Vec3 {
    match pos.seat {
        PlayerSeat::Selv => Vec3::new(
            TILE_WIDTH * TILE_SCALE * 0.8 * (pos.slot as f32 - 7.5),
            -200.0,
            pos.slot as f32,
        ),
        PlayerSeat::Across => Vec3::new(
            TILE_WIDTH * TILE_SCALE * 0.8 * (pos.slot as f32 - 7.5),
            200.0,
            pos.slot as f32,
        ),
        PlayerSeat::Left => Vec3::new(
            -200.0,
            TILE_HEIGHT * TILE_SCALE * 0.8 * (pos.slot as f32 - 7.5),
            pos.slot as f32,
        ),
        PlayerSeat::Right => Vec3::new(
            200.0,
            TILE_HEIGHT * TILE_SCALE * 0.8 * (pos.slot as f32 - 7.5),
            pos.slot as f32,
        ),
    }
}

fn spain_tehai_to_pos(
    commands: &mut Commands,
    mut sprite: SpriteBundle,
    bind: TileBind,
    pos: TehaiPos,
) {
    sprite.transform = Transform {
        translation: get_tehai_translation(&pos),
        scale: Vec3::splat(TILE_SCALE),
        ..default()
    };
    commands.spawn((
        sprite,
        bind,
        pos,
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
