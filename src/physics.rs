use avian2d::{math::Vector, prelude::Gravity, PhysicsPlugins};
use bevy::prelude::*;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default().with_length_unit(20.0))
            .insert_resource(Gravity(Vector::NEG_Y * 1000.0));
    }
}
