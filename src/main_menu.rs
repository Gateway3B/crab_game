use bevy::{app::AppExit, prelude::*};

use crate::*;

#[derive(Component)]
pub struct MenuUIRoot;

#[derive(Component)]
pub enum MainMenuButton {
    StartButton,
    QuitButton,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(Update, click_handler.run_if(in_state(GameState::MainMenu)));
    }
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(MenuUIRoot)
        .insert(Name::from("MenuUIRoot"))
        .with_children(|commands| {
            commands
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: UiRect::all(Val::Percent(3.0)),
                        ..default()
                    },
                    text: Text::from_section(
                        "Crab Game",
                        TextStyle {
                            font: asset_server.load("MOONB___.TTF"),
                            font_size: 96.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                })
                .insert(Name::from("Title"));
            spawn_button(
                commands,
                &asset_server,
                "Start Game",
                Color::srgb(0., 1., 0.),
                MainMenuButton::StartButton,
            );
            spawn_button(
                commands,
                &asset_server,
                "Quit Game",
                Color::srgb(1., 0., 0.),
                MainMenuButton::QuitButton,
            );
        });
}

fn spawn_button(
    commands: &mut ChildBuilder,
    asset_server: &AssetServer,
    text: &str,
    color: Color,
    button_type: MainMenuButton,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(65.0),
                height: Val::Percent(15.0),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Percent(2.0)),
                ..default()
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
    interaction: Query<(&Interaction, &MainMenuButton), Changed<Interaction>>,
    mut commands: Commands,
    menu_ui_root: Query<Entity, With<MenuUIRoot>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in &interaction {
        if matches!(interaction, Interaction::Pressed) {
            match button {
                MainMenuButton::StartButton => {
                    let menu_root = menu_ui_root.single();
                    commands.entity(menu_root).despawn_recursive();
                    game_state.set(GameState::Gameplay);
                }
                MainMenuButton::QuitButton => {
                    exit.send(AppExit::Success);
                }
            }
        }
    }
}
