use crate::actor::Health;
use crate::actor::Speed;
use crate::prelude::*;

use crate::actor::Team;
use crate::actor::TeamFlag;
use crate::input::Action;
use crate::player::Player;
use crate::player::PlayerPlugin;
use crate::weapon::components::*;

#[add_system(schedule = OnEnter(GameState::InGame), plugin = PlayerPlugin)]
fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    info!("Spawning player");
    // let weapon_entity = commands
    //     .spawn((
    //         Name::new("Orbit Weapon"),
    //         OrbitWeapon {
    //             orbit_speed: 3.2,
    //             orbit_radius: 100.0,
    //             orbit_direction: OrbitDirection::Clockwise,
    //             satellite_count: 12,
    //             satellite_image: assets.load("textures/weapons/dagger.png"),
    //             contact_damage: 100.0,
    //             contact_cooldown_per_entity: 0.5,
    //             contact_cooldown_map: Default::default(),
    //         },
    //         Transform::default(),
    //         GlobalTransform::default(),
    //     ))
    //     .id();

    let weapon_entity = commands
        .spawn((
            Name::new("Orbit Weapon"),
            OrbitAndLungeWeapon {
                orbit_speed: 6.4,
                orbit_radius: 100.0,
                orbit_direction: OrbitDirection::Clockwise,
                satellite_count: 12,
                satellite_image: assets.load("textures/weapons/dagger.png"),
                lunge_damage: 100.0,
                lunge_range: 64.0,
                lunge_cooldown: 1.0,
            },
            Transform::default(),
            GlobalTransform::default(),
        ))
        .id();

    commands
        .spawn((
            Player,
            Name::new("Player"),
            TeamFlag(Team::Player),
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            Action::default_input_map(),
            EquippedWeapons(vec![weapon_entity]),
            ShowAabbGizmo {
                color: Some(Color::srgb(0.0, 1.0, 0.0)),
            },
            Speed(256.0),
        ))
        .add_child(weapon_entity);
}

#[add_system(schedule = Update, plugin = PlayerPlugin, run_if = in_state(GameState::InGame))]
fn check_player_health(query: Query<&Health, With<Player>>) {
    for health in query.iter() {
        if health.current <= 0.0 {
            info!("Player health reached zero! Game Over!");
        }
    }
}
