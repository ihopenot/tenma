use bevy::{
    input::mouse::MouseMotion, prelude::*, render::view::RenderLayers, sprite::MaterialMesh2dBundle,
};
use bevy_mod_picking::prelude::*;

pub fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, DefaultPickingPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, (hello, drag_main_camera))
        .add_event::<Hello>();
    app.run();
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Event)]
pub struct Hello;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<AssetServer>,
) {
    // commands.spawn((
    //     Camera2dBundle {
    //         transform: Transform::from_xyz(0., 0., 0.),
    //         ..default()
    //     },
    //     // RenderLayers::layer(0),
    // ));

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::None,
                order: 1,
                ..default()
            },
            ..default()
        },
        // RenderLayers::layer(1),
        MainCamera,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(100., 0., 0.),
            texture: assets.load("textures/windboard.png"),
            ..default()
        },
        On::<Pointer<Click>>::send_event::<Hello>(),
        // RenderLayers::layer(1),
        PickableBundle::default(),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(50.0)).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_xyz(200., 0., 0.),
            ..default()
        },
        // RenderLayers::layer(0),
        PickableBundle::default(),
    ));
}

impl From<ListenerInput<Pointer<Click>>> for Hello {
    fn from(_: ListenerInput<Pointer<Click>>) -> Self {
        Hello
    }
}

pub fn drag_main_camera(
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    for ev in motion_evr.read() {
        if buttons.pressed(MouseButton::Right) {
            println!("pressed");
            camera.single_mut().translation += Vec3::new(-ev.delta.x, ev.delta.y, 0.0);
        }
    }
}

pub fn hello(mut reader: EventReader<Hello>) {
    reader.read().for_each(|_| println!("hello"));
}