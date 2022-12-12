use bevy::{prelude::*, app::AppExit};

use crate::*;

#[derive(Component)]
pub struct MenuUIRoot;

#[derive(Component)]
pub enum MainMenuButton {
    StartButton,
    QuitButton
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(spawn_main_menu))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(click_handler));
    }
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(MenuUIRoot)
        .with_children(|commands| {
            commands
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: UiRect::all(Val::Percent(3.0)),
                        ..default()
                    },
                    text: Text::from_section("Tower Defense Tutorial", TextStyle {
                        font: asset_server.load("MOONB___.TTF"),
                        font_size: 96.0,
                        color: Color::BLACK
                    }),
                    ..default()
                });
            spawn_button(commands, &asset_server, "Start Game", Color::GREEN, MainMenuButton::StartButton);
            spawn_button(commands, &asset_server, "Quit Game", Color::GREEN, MainMenuButton::QuitButton);            
        });
}

fn spawn_button(commands: &mut ChildBuilder, asset_server: &AssetServer, text: &str, color: Color, button_type: MainMenuButton) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(65.0), Val::Percent(15.0)),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Percent(2.0)),
                ..default()
            },
            background_color: color.into(),
            ..default()
        })
        .with_children(|commands| {
            commands
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: UiRect::all(Val::Percent(3.0)),
                        ..default() 
                    },
                    text: Text::from_section(text, TextStyle {
                        font: asset_server.load("MOONB___.TTF"),
                        font_size: 64.0,
                        color: Color::BLACK
                    }),
                    ..default()
                });
        })
        .insert(button_type)
        .id()
}

fn click_handler(
    interaction: Query<(&Interaction, &MainMenuButton), Changed<Interaction>>,
    mut commands: Commands,
    menu_ui_root: Query<Entity, With<MenuUIRoot>>,
    mut game_state: ResMut<State<GameState>>,
    mut exit: EventWriter<AppExit>,
    mut mouse_input: ResMut<Input<MouseButton>>
) {
    for (interaction, button) in &interaction {
        if matches!(interaction, Interaction::Clicked) {
            match button {
                MainMenuButton::StartButton => {
                    let menu_root = menu_ui_root.single();
                    commands.entity(menu_root).despawn_recursive();
                    game_state.set(GameState::Gameplay)
                        .unwrap_or_else(|err| {
                            println!("{err:?}");
                            exit.send(AppExit);
                        });
                    mouse_input.clear();
                },
                MainMenuButton::QuitButton => {
                    exit.send(AppExit);
                }
            }
        }
    }
}