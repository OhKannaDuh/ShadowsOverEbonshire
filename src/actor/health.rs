use crate::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            max: 100.0,
            current: 100.0,
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Name::new("Health Bar"))]
pub struct HealthBar;

impl HealthBar {
    pub fn add_to_entity(entity: Entity, _commands: &mut Commands) {
        debug!("Adding HealthBar to entity: {:?}", entity);
        // commands.entity(entity).with_children(|parent| {
        //     parent.spawn((
        //         Name::new("Health Bar Background"),
        //         Sprite {
        //             color: Color::BLACK,
        //             custom_size: Some(Vec2::new(40.0, 6.0)),
        //             ..default()
        //         },
        //         Transform::from_xyz(0.0, 24.0, 0.0),
        //     ));

        //     parent.spawn((
        //         HealthBar,
        //         Sprite {
        //             color: Color::srgb(0.0, 1.0, 0.0),
        //             custom_size: Some(Vec2::new(40.0, 6.0)),
        //             ..default()
        //         },
        //         Transform::from_xyz(0.0, 24.0, 1.0),
        //     ));
        // });
    }
}
