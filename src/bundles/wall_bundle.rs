use bevy::{
    ecs::bundle::Bundle,
    math::Vec2,
    prelude::default,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform
};

use crate::{components, constants};

#[derive(Bundle)]
pub struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    pub sprite_bundle: SpriteBundle,
    pub collider: components::Collider,
}

/// Which side of the arena is this wall located on?
pub enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(constants::wall::LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(constants::wall::RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., constants::wall::BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., constants::wall::TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = constants::wall::TOP_WALL - constants::wall::BOTTOM_WALL;
        let arena_width = constants::wall::RIGHT_WALL - constants::wall::LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(constants::wall::WALL_THICKNESS, arena_height + constants::wall::WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + constants::wall::WALL_THICKNESS, constants::wall::WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    pub fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: constants::wall::WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: components::Collider,
        }
    }
}
