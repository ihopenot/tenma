use crate::config::{GameState, InGameState};
use crate::game::{self, PlayerStatistics};
use crate::resource::GameTextures;
use bevy::prelude::*;
use bevy::transform::commands;

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
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            GameUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(windboard_width),
                        height: Val::Px(windboard_height),
                        // This takes the icons out of the flexbox flow, to be positioned exactly
                        position_type: PositionType::Relative,
                        ..default()
                    },
                    image: UiImage::new(game_texture.windboard.clone()),
                    transform: Transform {
                        scale: Vec3::splat(windboard_scale),
                        rotation: Quat::from_rotation_z(0.0),
                        ..default()
                    },
                    ..default()
                },
                WindBoard,
            ));
        });
    ingamestate.set(InGameState::GameObjectUI);
}

fn setup_gameobject_ui(
    mut commands: Commands,
    game_texture: Res<GameTextures>,
    player_stastics: Res<PlayerStatistics>,
    mut ingamestate: ResMut<NextState<InGameState>>,
) {
    let self_status = &player_stastics.status[player_stastics.self_id as usize];
    for i in 0..14 {
        commands.spawn(
            (SpriteBundle {
                // style: Style {
                //     width: Val::Px(tile_width),
                //     height: Val::Px(tile_height),
                //     // This takes the icons out of the flexbox flow, to be positioned exactly
                //     position_type: PositionType::Relative,
                //     ..default()
                // },
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
            }),
        );
    }
}
