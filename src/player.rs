use crate::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    pub money: u32,
    pub health: u32,
}

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>().add_system_set(
            SystemSet::on_enter(GameState::Gameplay)
                .with_system(spawn_player)
                .with_system(give_money_on_kill.after(spawn_player)),
        );
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player {
            money: 100,
            health: 1000,
        },
        Name::new("Player"),
    ));
}

fn give_money_on_kill(
    mut player_query: Query<&mut Player>,
    mut death_events: EventReader<TargetDeathEvent>,
) {   
    let Ok(mut player) = player_query.get_single_mut() else { return };

    for _event in death_events.iter() {
        player.money += 10;
    }
}
