use bevy::{
    ecs::bundle::Bundle,
    math::{Vec2, Vec3},
    prelude::default,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform
};

use crate::{
    components,
    constants
};

#[derive(Bundle)]
pub struct BrickBundle {
    sprite_bundle: SpriteBundle,
    brick: components::Brick,
    collider: components::Collider,
}

impl BrickBundle {
    pub fn new(brick_position: Vec2) -> BrickBundle {
        BrickBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: constants::brick::BRICK_COLOR,
                    ..default()
                },
                transform: Transform {
                    translation: brick_position.extend(0.0),
                    scale: Vec3::new(constants::brick::BRICK_SIZE.x, constants::brick::BRICK_SIZE.y, 1.0),
                    ..default()
                },
                ..default()
            },
            brick: components::Brick,
            collider: components::Collider,
        }
    }
}
