use avian2d::prelude::*;
use bevy::prelude::*;

use super::{
    movement::{JumpImpulse, MovementAcceleration, MovementDampingFactor},
    Player,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        Sprite::from_color(Color::hsl(2.0, 0.96, 0.7), Vec2::new(32.0, 32.0)),
        Collider::rectangle(32.0, 32.0),
        RigidBody::Dynamic,
        Transform::from_xyz(0.0, 0.0, 0.0),
        MovementAcceleration(250.0),
        JumpImpulse(650.0),
        MovementDampingFactor(0.5),
        // What does that mean here
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(1.0),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(2.0),
    ));

    // TODO: move this to another module or remove it. this is just a demonstration
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(600.0, 32.0))),
        MeshMaterial2d(materials.add(Color::hsl(144.0, 94.0, 54.0))),
        Collider::rectangle(600.0, 32.0),
        RigidBody::Static,
        Transform::from_xyz(0.0, -300.0, 0.0),
    ));
}
