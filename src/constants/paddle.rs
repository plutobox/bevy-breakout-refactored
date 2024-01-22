use bevy::prelude::{
    Vec3,
    Color
};

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
pub const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
pub const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
pub const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
pub const PADDLE_PADDING: f32 = 10.0;

// These values are exact
pub const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;

pub const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
