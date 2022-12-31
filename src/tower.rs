use bevy::prelude::*;

use crate::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

#[derive(EnumIter, IntoStaticStr, Inspectable, Component, Clone, Copy, Debug)]
pub enum TowerType {
    Cube,
    Sphere,
    Crab,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(tower_shooting));
    }
}

fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, Added<Target>>,
    time: Res<Time>,
) {
    for (tower_ent, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn_loc = transform.translation() + tower.bullet_offset;

            let direction = targets
                .iter()
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(
                        target_transform.translation(),
                        bullet_spawn_loc,
                    ))
                })
                .map(|closest_target| closest_target.translation() - bullet_spawn_loc);

            if let Some(direction) = direction {
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                        material: materials.add(Color::rgb(0.87, 0.44, 0.42).into()),
                        transform: Transform::from_translation(bullet_spawn_loc),
                        ..default()
                    })
                    .insert(Lifetime {
                        timer: Timer::from_seconds(1000.5, TimerMode::Once),
                    })
                    .insert(Bullet {
                        direction: direction,
                        speed: 9.0,
                    })
                    .insert(Name::new("Bullet"));
            }
        }
    }
}

// fn build_tower(
//     mut commands: Commands,
//     selection: Query<(Entity, &Selection, &Transform)>,
//     keyboard: Res<Input<KeyCode>>,
//     assets: Res<GameAssets>,
//     interaction: Query<(&Interaction, &TowerType), Changed<Interaction>>
// ) {
//     for (interaction, tower_type) in &interaction {
//         if matches!(interaction, Interaction::Clicked) {
//             if selection.selected() {
//                 commands.entity(entity).despawn_recursive();
//                 spawn_tower(&mut commands, &assets, transform.translation, tower_type);
//             }
//         }
//     }
// }

fn tower_button_clicked(interaction: Query<(&Interaction, &TowerType), Changed<Interaction>>) {
    for (interaction, tower_type) in &interaction {
        if matches!(interaction, Interaction::Clicked) {
            info!("Spawning: {:?}", tower_type)
        }
    }
}

pub fn spawn_tower(
    commands: &mut Commands,
    assets: &GameAssets,
    position: Vec3,
    tower_type: TowerType,
) -> Entity {
    let (tower_scene, tower) = tower_type.get_tower(assets);

    commands
        .spawn(SpatialBundle::from_transform(Transform::from_translation(
            position,
        )))
        .insert(Name::new("Crab Tower"))
        .insert(tower_type)
        .insert(tower)
        .insert(Name::new(format!("{tower_type:?}_;Tower")))
        .insert(Tower {
            shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            bullet_offset: Vec3::new(0.0, 0.6, 0.0),
        })
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: tower_scene,
                transform: Transform::from_xyz(0.0, -0.8, 0.0),
                ..default()
            });
        })
        .id()
}

impl TowerType {
    fn get_tower(&self, assets: &GameAssets) -> (Handle<Scene>, Tower) {
        match self {
            TowerType::Cube => (
                assets.crab_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(1.5, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                },
            ),
            TowerType::Sphere => (
                assets.crab_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                },
            ),
            TowerType::Crab => (
                assets.crab_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                },
            ),
        }
    }
}
