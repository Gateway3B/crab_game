#[derive(Component)]
struct Crab {
    shooting_timer: Timer,
    level: i32,
    species: Species,
}

struct Species {
    name: String,
    base_hit_points: i32,
}

// fn add_crabs(mut commands: Commands) {
//     commands.spawn(Crab {
//         level: 1,
//         species: Species {
//             name: String::from("Xenocarcinus Depressus"),
//             base_hit_points: (10),
//         },
//     });
// }
