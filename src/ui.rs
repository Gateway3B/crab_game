use bevy::{prelude::*, ecs::query::QuerySingleError};

use crate::*;

pub struct UIPlugin;

#[derive(Component)]
pub struct TowerUIRoot;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TowerButtonState {
    cost: u32,
    affordable: bool
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(create_ui_on_selection)
                    .with_system(tower_button_clicked)
                    .with_system(grey_tower_buttons.after(create_ui_on_selection))
            );
    }
}

fn create_ui_on_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selections: Query<&Selection>,
    root: Query<Entity, With<TowerUIRoot>>,
) {
    let at_least_one_selected = selections.iter().any(|selection| selection.selected());

    match root.get_single() {
        Ok(root) => {
            if !at_least_one_selected {
                commands.entity(root).despawn_recursive();
            }
        }
        Err(QuerySingleError::NoEntities(..)) => {
            if at_least_one_selected {
                create_ui(&mut commands, &asset_server);
            }
        }
        _ => unreachable!("Too many ui tower roots!"),
    }
}

fn create_ui(commands: &mut Commands, crab_assets: &AssetServer) {
    let cost = [50, 80, 110];

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(TowerUIRoot)
        .insert(Name::new("UIRoot"))
        .with_children(|commands| {
            for (i, tower) in TowerType::iter().enumerate() {
                commands
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Percent(150. * 9.0 / 16.0), Val::Percent(15.0)),
                            align_self: AlignSelf::FlexEnd,
                            margin: UiRect::all(Val::Percent(2.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(TowerButtonState {
                        cost: cost[i],
                        affordable: false
                    })
                    .insert(tower)
                    .insert(Name::new("Button"))
                    .with_children(|commands| {
                        let text_string: &'static str = tower.into();
                        commands.spawn(TextBundle {
                            text: Text::from_section(
                                text_string,
                                TextStyle {
                                    font: crab_assets.load("MOONB___.TTF"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ),
                            ..default()
                        });
                    });
            }
        });
}

fn tower_button_clicked(
    interaction: Query<(&Interaction, &TowerType, &TowerButtonState), Changed<Interaction>>,
    mut commands: Commands,
    selection: Query<(Entity, &Selection, &Transform)>,
    mut player: Query<&mut Player>,
    assets: Res<GameAssets>,
) {
    let mut player = player.single_mut();
    for (interaction, tower_type, tower_button_state) in &interaction {
        if matches!(interaction, Interaction::Clicked) {
            for (entity, selection, transform) in &selection {
                if selection.selected() {
                    if player.money >= tower_button_state.cost {
                        player.money -= tower_button_state.cost;
                        commands.entity(entity).despawn_recursive();
                        spawn_tower(&mut commands, &assets, transform.translation, *tower_type);
                    }
                }
            }
        }
    }
}

fn grey_tower_buttons(
    mut buttons: Query<(&mut BackgroundColor, &mut TowerButtonState)>,
    player: Query<&Player>
) {
    if let Ok(player) = player.get_single() {
        for (mut tint, mut state) in &mut buttons {
            if player.money >= state.cost {
                state.affordable = true;
                *tint = Color::WHITE.into();
            } else {
                state.affordable = false;
                *tint = Color::DARK_GRAY.into();
            }
        }
    }
}
