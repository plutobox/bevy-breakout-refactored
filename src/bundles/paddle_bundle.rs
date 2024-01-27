use bevy::{
    ecs::bundle::Bundle,
    math::Vec3,
    prelude::default,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform
};

use crate::{
    components,
    constants
};

#[derive(Bundle)]
pub struct PaddleBundle {
    sprite_bundle: SpriteBundle,
    paddle: components::Paddle,
    collider: components::Collider,
}

impl PaddleBundle {
    pub fn new(translation: Vec3) -> PaddleBundle {
        PaddleBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: translation,
                    scale: constants::paddle::PADDLE_SIZE,
                    ..default()
                },
                sprite: Sprite {
                    color: constants::paddle::PADDLE_COLOR,
                    ..default()
                },
                ..default()
            },
            paddle: components::Paddle,
            collider: components::Collider,
        }
    }
}
