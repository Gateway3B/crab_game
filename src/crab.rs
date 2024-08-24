use std::{f32::consts::PI, time::Duration};

use bevy::animation::animate_targets;
use bevy_inspector_egui::InspectorOptions;
use strum::{Display, EnumIter, IntoStaticStr};

use crate::*;

// region: Plugin

pub struct CrabPlugin;

impl Plugin for CrabPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CrabComp>()
            .add_systems(OnEnter(GameState::MainMenu), setup_animations)
            .add_systems(Update, start_default_crab_animation.before(animate_targets).run_if(in_state(GameState::Gameplay)));
    }
}

// endregion

// region: Structs

#[derive(Resource)]
pub struct Animations {
    pub animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}

#[derive(EnumIter, PartialEq, Eq, Hash, Component, Display, Clone)]
pub enum CrabAnimation {
    #[strum(to_string = "Attack 1")]
    Attack1,
    #[strum(to_string = "Attack 2")]
    Attack2,
    #[strum(to_string = "Idle")]
    Idle,
    #[strum(to_string = "Jump Backward")]
    JumpBackward,
    #[strum(to_string = "Jump Forward")]
    JumpForward,
    #[strum(to_string = "Special 1")]
    Special1,
    #[strum(to_string = "Walk Right")]
    WalkRight,
}

#[derive(EnumIter, IntoStaticStr, InspectorOptions, Component, Clone, Copy, Debug)]
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

#[derive(Component, InspectorOptions)]
pub struct CrabMove {
    _name: String,
    _ce: i8, // Crab Energy
    _effect: CrabMoveEffect,
    _target: CrabMoveTarget,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CrabComp;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CrabModelComp;

#[derive(Component)]
pub struct AnimationPlayerId {
    _id: Entity,
}

// endregion

// region: Systems

fn setup_animations(
    mut commands: Commands,
    crab1_assets: Res<Crab1Assets>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(crab1_assets.animations.clone(), 1.0, graph.root).collect();

    let graph = graphs.add(graph);
    commands.insert_resource(Animations {
        animations,
        graph: graph.clone()
    });
}

fn start_default_crab_animation(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Without<Handle<AnimationGraph>>>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();

        transitions
            .play(&mut player, animations.animations[CrabAnimation::Idle as usize], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(animations.graph.clone())
            .insert(transitions);
    }
}

pub fn spawn_crab(
    commands: &mut Commands,
    crab_assets: &Crab1Assets,
    transform: Transform,
    _crab_type: CrabType,
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
            })
            .insert(CrabModelComp);
        })
        .insert(CrabComp)
        .insert(Name::new("Crab"))
        .id()
}

// endregion
