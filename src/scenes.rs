use bevy::render::render_resource::{AsBindGroup, ShaderRef};

use crate::*;

// region: Plugin

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MaterialPlugin::<WaterMaterial>::default())
            .add_systems(OnEnter(GameState::Gameplay), spawn_basic_scene);
    }
}

// enderegion

// region: Structs

// endregion

// region: Materials

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
struct WaterMaterial {}

impl Material for WaterMaterial {
    fn fragment_shader() -> ShaderRef {
        "water_shader.wgsl".into()
    }

    // fn vertex_shader() -> ShaderRef {
    //     "water_shader.wgsl".into()
    // }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

// endregion

// region: Systems

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut water_materials: ResMut<Assets<WaterMaterial>>,
    crab: Res<Crab1Assets>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(200., 200.)),
            material: materials.add(Color::srgb(0.76, 0.7, 0.5)),
            ..default()
        })
        .insert(Name::new("Ground"));

    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(200., 200.)),
            material: water_materials.add(WaterMaterial {}),
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
    // let transform = Transform::from_xyz(5.0, 0.0, 0.0);
    // spawn_crab(&mut commands, &crab, transform, CrabType::One);
}

// endregion
