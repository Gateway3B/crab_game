use bevy::{pbr::NotShadowCaster, prelude::*, utils::FloatOrd};
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin};
use bevy_mod_picking::*;
use strum::{IntoEnumIterator, IntoStaticStr};
use strum_macros::EnumIter;

mod bullet;
mod main_menu;
mod player;
mod state;
mod target;
mod tower;
mod ui;

pub use bullet::*;
pub use main_menu::*;
pub use player::*;
pub use state::*;
pub use target::*;
pub use tower::*;
pub use ui::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(PickingCameraBundle::default());
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    crab_assets: Res<GameAssets>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
            material: materials.add(Color::rgb(0.76, 0.7, 0.5).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());

    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            0.0, 0.8, 0.0,
        )))
        .insert(Name::new("Tower_Base"))
        .insert(meshes.add(shape::Capsule::default().into()))
        .insert(Highlighting {
            initial: default_collider_color.clone(),
            hovered: Some(selected_collider_color.clone()),
            pressed: Some(selected_collider_color.clone()),
            selected: Some(selected_collider_color),
        })
        .insert(default_collider_color)
        .insert(NotShadowCaster)
        .insert(PickableBundle::default());

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 3500.0,
                shadows_enabled: true,
                range: 50.0,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 18.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));

    for i in 0..3 {
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
                material: materials.add(Color::DARK_GREEN.into()),
                transform: Transform::from_xyz(-2.0 * i as f32, 0.5, 3.0),
                ..default()
            })
            .insert(Target {
                speed: 0.3,
                path_index: 0,
            })
            .insert(Health { value: 3 })
            .insert(Name::new("Target"));
    }
}

fn start_music(audio: Res<Audio>, asset_server: Res<AssetServer>) {
    audio
        .play(asset_server.load("Crab Game Looped Smooth.mp3"))
        .looped();
}

fn start_animation(
    mut player: Query<&mut AnimationPlayer>,
    animations: Res<Crab1Animations>,
    mut done: Local<bool>,
) {
    if !*done {
        for mut player in player.iter_mut() {
            player.play(animations.0[2].clone_weak()).repeat();
            *done = true;
        }
    }
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = 3.0;
    let rotate_speed = 0.3;

    if keyboard.pressed(KeyCode::F) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::R) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::T) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::W) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds());
    }
    if keyboard.pressed(KeyCode::P) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds());
    }
}

enum Crab1Animation {
    Attack1,
    Attack2,
    Idle,
    JumpBackward,
    JumpForward,
    Special1,
    WalkRight,
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        crab_scene: assets.load("Crab1.glb#Scene0"),
    });
    commands.insert_resource(Crab1Animations(vec![
        assets.load("Crab1.glb#Animation0"),
        assets.load("Crab1.glb#Animation1"),
        assets.load("Crab1.glb#Animation2"),
        assets.load("Crab1.glb#Animation3"),
        assets.load("Crab1.glb#Animation4"),
        assets.load("Crab1.glb#Animation5"),
        assets.load("Crab1.glb#Animation6"),
    ]));
}

#[derive(Resource)]
pub struct Crab1Animations(Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct GameAssets {
    pub crab_scene: Handle<Scene>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: String::from("Bevy Tower Defense"),
                resizable: false,
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(TargetPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .add_state(GameState::MainMenu)
        .add_startup_system(asset_loading)
        .add_startup_system(start_music)
        .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_basic_scene))
        // .add_startup_system_to_stage(StartupStage::PostStartup, spawn_basic_scene)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_camera)
        .add_system(start_animation)
        // .add_system(camera_controls)
        .run();
}
