use bevy::math::Vec3Swizzles;

use crate::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
    pub path_index: usize
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}

#[derive(Resource)]
pub struct TargetPath {
    waypoints: Vec<Vec2>
}

pub struct TargetDeathEvent;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .register_type::<Health>()
            .add_event::<TargetDeathEvent>()
            .insert_resource(TargetPath {
                waypoints: vec![
                    Vec2::new(6.0, 2.0),
                    Vec2::new(6.0, 9.0),
                    Vec2::new(9.0, 9.0)
                ]
            })
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(move_targets)
                    .with_system(target_death)
            );
    }
}

fn move_targets(
    mut targets: Query<(&mut Target, &mut Transform)>, 
    time: Res<Time>,
    path: Res<TargetPath>
) {
    for (mut target, mut transform) in &mut targets {
        let delta = target.speed * time.delta_seconds();
        let delta_target = path.waypoints[target.path_index] - transform.translation.xz();

        if delta_target.length() > delta {
            let movement = delta_target.normalize() * delta;
            transform.translation += movement.extend(0.0).xzy();
            let y = transform.translation.y;
            transform.look_at(path.waypoints[target.path_index].extend(y).xzy(), Vec3::Y);
        } else {
            target.path_index += 1;
        }

        transform.translation.x += target.speed * time.delta_seconds();
    }
}

fn target_death(
    mut commands: Commands, 
    targets: Query<(Entity, &Health)>,
    mut death_event_writer: EventWriter<TargetDeathEvent>
) {
    for (ent, health) in &targets {
        if health.value <= 0 {
            commands.entity(ent).despawn_recursive();
            death_event_writer.send(TargetDeathEvent);
        }
    }
}

fn hurt_player(
    mut commands: Commands,
    targets: Query<(Entity, &Target)>,
    path: Res<TargetPath>,
    mut player: Query<&mut Player>,
) {
    for (entity, target) in &targets {
        if target.path_index >= path.waypoints.len() {
            commands.entity(entity).despawn_recursive();

            let mut player = player.single_mut();
            if player.health > 0 {
                player.health -= 1;
            }

            if player.health == 0 {
                info!("GAME OVER");
            }
        }
    }
}
