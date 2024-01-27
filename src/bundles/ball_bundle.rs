use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    math::Vec3,
    prelude::default,
    render::mesh::Mesh,
    sprite::{Material2d, MaterialMesh2dBundle},
    transform::components::Transform
};

use crate::{
    components,
    constants
};

#[derive(Bundle)]
pub struct BallBundle<M: Material2d> {
    material_mesh_2d_bundle: MaterialMesh2dBundle<M>,
    ball: components::Ball,
    velocity: components::Velocity,
    collider: components::Collider,
}

impl<M: Material2d> BallBundle<M> {
    pub fn new(mesh: Handle<Mesh>, material: Handle<M>, starting_position: Option<Vec3>) -> BallBundle<M> {
        BallBundle {
            material_mesh_2d_bundle: MaterialMesh2dBundle {
                mesh: mesh.into(),
                material: material,
                transform: Transform::from_translation(starting_position.unwrap_or(constants::ball::BALL_STARTING_POSITION)).with_scale(constants::ball::BALL_SIZE),
                ..default()
            },
            ball: components::Ball,
            velocity: components::Velocity(constants::ball::INITIAL_BALL_DIRECTION.normalize() * constants::ball::BALL_SPEED),
            collider: components::Collider,
        }
    }
}
