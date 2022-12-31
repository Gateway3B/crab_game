use std::{collections::HashMap, f32::consts::PI};

use bevy::prelude::{shape::Quad, *};

use bevy_inspector_egui::Inspectable;
use strum::{EnumIter, IntoStaticStr};

use crate::*;

// region: Plugin

pub struct CrabPlugin;

impl Plugin for CrabPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CrabComp>().add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(start_animation)
                .with_system(bind_animations),
        );
    }
}

// endregion

// region: Structs

#[derive(EnumIter, PartialEq, Eq, Hash)]
pub enum CrabAnimations {
    Attack1,
    Attack2,
    Idle,
    JumpBackward,
    JumpForward,
    Special1,
    WalkRight,
}

#[derive(EnumIter, IntoStaticStr, Inspectable, Component, Clone, Copy, Debug)]
pub enum CrabType {
    One,
}

pub enum CrabMoveEffect {
    Damage,
    Status,
}

pub enum CrabMoveTarget {
    Enemies,
    Allies,
    All,
}

#[derive(Component, Inspectable)]
pub struct CrabMove {
    name: String,
    ce: i8, // Crab Energy
    effect: CrabMoveEffect,
    target: CrabMoveTarget,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CrabComp;

#[derive(Component)]
pub struct AnimationPlayerId {
    id: Entity,
}

// endregion

// region: Systems

fn bind_animations(
    mut players: Query<(Entity, &mut AnimationPlayer)>,
    children: Query<&Children>,
    crabs: Query<Entity, (With<CrabComp>, Without<AnimationPlayerId>)>,
    mut commands: Commands,
) {
    for entity in &crabs {
        let Some((animation_player_entity, _)) = players
            .iter_many_mut(children.iter_descendants(entity))
            .fetch_next() else { continue };

        let Some(mut entity_commands) = commands.get_entity(entity) else { continue };

        entity_commands.insert(AnimationPlayerId {
            id: animation_player_entity,
        });
    }
}

fn start_animation(
    crabs: Query<(Entity, &AnimationPlayerId), With<CrabComp>>,
    mut players: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
    mut commands: Commands,
    crab1_assets: Res<Crab1Assets>,
) {
    if *done {
        return;
    }

    for (crab_entity, animation_player_id) in crabs.iter() {
        let Ok(mut player) = players.get_mut(animation_player_id.id) else { return };

        player
            .play(crab1_assets.animations[CrabAnimations::Idle as usize].clone_weak())
            .repeat();
    }
}

pub fn spawn_crab(
    commands: &mut Commands,
    crab_assets: &Crab1Assets,
    transform: Transform,
    crab_type: CrabType,
) -> Entity {
    commands
        .spawn(SpatialBundle::from_transform(transform))
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: crab_assets.model.clone(),
                transform: Transform::from_xyz(0.0, 2.0, 0.0).with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    0.0,
                    PI,
                    0.0,
                )),
                ..default()
            });
        })
        .insert(CrabComp)
        .insert(Name::new("Crab"))
        .id()
}

// endregion
