use crate::config::InGameState;
use bevy::prelude::*;

const tile_width: f32 = 60.0;
const tile_height: f32 = 73.0;
const tile_scale: f32 = 1.5;
const windboard_width: f32 = 400.0;
const windboard_height: f32 = 400.0;
const windboard_scale: f32 = 0.5;

#[derive(Component)]
struct GameUI;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(OnEnter(InGameState::GeneralUI), setup_general_game_ui)
        .add_systems(OnEnter(InGameState::GameObjectUI), setup_gameobject_ui);
}

fn setup_general_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
            let icon = asset_server.load("textures/windboard.png");
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(windboard_width),
                    height: Val::Px(windboard_height),
                    // This takes the icons out of the flexbox flow, to be positioned exactly
                    position_type: PositionType::Relative,
                    ..default()
                },
                image: UiImage::new(icon),
                transform: Transform {
                    scale: Vec3::splat(windboard_scale),
                    rotation: Quat::from_rotation_z(0.0),
                    ..default()
                },
                ..default()
            });
        });
    ingamestate.set(InGameState::GameObjectUI);
}

fn setup_gameobject_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ingamestate: ResMut<NextState<InGameState>>,
) {
    for i in 0..14 {
        let icon = asset_server.load("textures/tiles/34.png");
        commands.spawn(SpriteBundle {
            // style: Style {
            //     width: Val::Px(tile_width),
            //     height: Val::Px(tile_height),
            //     // This takes the icons out of the flexbox flow, to be positioned exactly
            //     position_type: PositionType::Relative,
            //     ..default()
            // },
            texture: icon.clone(),
            transform: Transform {
                scale: Vec3::splat(tile_scale),
                translation: Vec3::new(tile_width * tile_scale * 0.8 * (i as f32 - 7.5), -200.0, i as f32),
                ..default()
            },
            ..default()
        });
    }
}
