use std::f32::consts::PI;

use crate::*;

// region: Plugin

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_basic_scene));
    }
}

// enderegion

// region: Structs

// endregion

// region: Systems

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    crab: Res<Crab1Assets>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 200.0 })),
            material: materials.add(Color::rgb(0.76, 0.7, 0.5).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 200.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE.clone().set_a(0.5).to_owned(),
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        })
        .insert(Name::new("Water"));

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

    let transform = Transform::from_xyz(0.0, 0.0, 0.0);
    spawn_crab(&mut commands, &crab, transform, CrabType::One);
    let transform = Transform::from_xyz(5.0, 0.0, 0.0);
    spawn_crab(&mut commands, &crab, transform, CrabType::One);
}

// endregion
