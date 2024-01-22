use bevy::prelude::{
    Vec2,
    Color,
};

pub const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);

pub const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

// this value is exact, whatever that means
pub const GAP_BETWEEN_BRICKS: f32 = 5.0;

// These values are lower bounds, as the number of bricks is computed
pub const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
pub const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;
