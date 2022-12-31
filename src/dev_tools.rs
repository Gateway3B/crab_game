use bevy_inspector_egui::WorldInspectorPlugin;

use crate::*;

// region: Plugin

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(camera_controls));
    }
}

// enderegion

// region: Structs

// endregion

// region: Systems

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera_query.get_single_mut() else { return };

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

// endregion
