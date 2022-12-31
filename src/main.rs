use bevy::prelude::*;

mod audio;
mod combat_ui;
mod crab;
mod dev_tools;
mod loading;
mod main_menu;
mod scenes;
mod state;

pub use audio::*;
pub use combat_ui::*;
pub use crab::*;
pub use dev_tools::*;
pub use loading::*;
pub use main_menu::*;
pub use scenes::*;
pub use state::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-3.5, 5.0, 7.5).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Name::new("Camera"));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3))) // Background color
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: String::from("Crab Game"),
                resizable: false,
                ..Default::default()
            },
            ..default()
        }))
        .add_state(GameState::MainLoading)
        .add_startup_system(spawn_camera)
        .add_plugin(InternalAudioPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(CombatUIPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(CrabPlugin)
        .add_plugin(ScenesPlugin)
        .add_plugin(DevToolsPlugin)
        .run();
}
