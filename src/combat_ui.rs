use std::time::Duration;

use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::*;

#[derive(Component)]
pub struct CombatUIRoot;

pub struct CombatUIPlugin;

impl Plugin for CombatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), spawn_combat_ui)
            .add_systems(Update, click_handler.run_if(in_state(GameState::Gameplay)));
    }
}

fn spawn_combat_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect {
                    top: Val::Auto,
                    ..Default::default()
                },
                width: Val::Percent(100.),
                height: Val::Percent(30.),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            ..default()
        })
        .insert(CombatUIRoot)
        .insert(Name::from("CombatUIRoot"))
        .with_children(|commands| {
            CrabAnimation::iter().for_each(|animation| {
                spawn_button(
                    commands,
                    &asset_server,
                    &animation.to_string(),
                    Color::srgb(0., 0., 1.),
                    animation,
                );
            });
        });
}

fn spawn_button(
    commands: &mut ChildBuilder,
    asset_server: &AssetServer,
    text: &str,
    color: Color,
    button_type: CrabAnimation,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(32.0),
                height: Val::Percent(30.0),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            transform: Transform {
                scale: Vec3 {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5
                },
                ..Default::default()
            },
            background_color: color.into(),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load("MOONB___.TTF"),
                        font_size: 64.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        })
        .insert(button_type)
        .insert(Name::from("Button"))
        .id()
}

fn click_handler(
    interaction: Query<(&Interaction, &CrabAnimation), Changed<Interaction>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
) {
    let mut action = None;
    for (interaction, button) in &interaction {
        if matches!(interaction, Interaction::Pressed) {
            action = Some(button.clone() as usize)
        }
    }

    if action.is_none() {
        return;
    }

    for (mut player, mut transitions) in &mut animation_players {
        transitions
            .play(
                &mut player,
                animations.animations[action.unwrap_or_default()],
                Duration::from_millis(250),
            )
            .repeat();
    }
}
