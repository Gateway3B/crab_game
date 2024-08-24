use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::*;

// region: Plugin

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins(WorldInspectorPlugin::new())
            .add_systems(Update, camera_controls.run_if(in_state(GameState::Gameplay)));
    }
}

// enderegion

// region: Structs

// endregion

// region: Systems

fn camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera_query.get_single_mut() else { return };

    let forward = camera.forward();

    let left = camera.left();

    let speed = 3.0;
    let rotate_speed = 0.3;

    if keyboard.pressed(KeyCode::KeyF) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::KeyR) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::KeyT) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::KeyW) {
        camera.rotate_axis(Dir3::Y, rotate_speed * time.delta_seconds());
    }
    if keyboard.pressed(KeyCode::KeyP) {
        camera.rotate_axis(Dir3::Y, -rotate_speed * time.delta_seconds());
    }
}

// endregion
