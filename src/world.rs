use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, Query, Res, Resource, Transform};
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::parry::shape::Shape;

pub struct WorldPlugin;

#[derive(Resource)]
pub struct GameWorld {
    pub width: f32,
    pub height: f32,
}

pub const WORLD_WIDTH: f32 = 960.;
pub const WORLD_HEIGHT: f32 = 540.;
pub const CENTER_X: f32 = WORLD_WIDTH / 2.;
pub const CENTER_Y: f32 = WORLD_HEIGHT / 2.;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameWorld {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
        });
        app.add_systems(Update, wrap_around_world);
    }
}

fn wrap_around_world(mut query: Query<(&mut Transform, &Collider)>, world: Res<GameWorld>) {
    for (mut transform, collider) in &mut query {
        let position = transform.translation;
        let size = collider.raw.compute_local_aabb().half_extents();
        let width = size.x * 2.;
        let height = size.y * 2.;

        if position.x > world.width + width {
            transform.translation.x = 0. - width;
        } else if position.x < 0. - width {
            transform.translation.x = world.width + width;
        }

        if position.y > world.height + height {
            transform.translation.y = 0. - height;
        } else if position.y < 0. - height {
            transform.translation.y = world.height + height;
        }
    }
}
