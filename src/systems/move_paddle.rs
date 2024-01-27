use bevy::{
    app::{App, Plugin, Update}, ecs::{
        query::With, schedule::{common_conditions::in_state, IntoSystemConfigs, ScheduleLabel}, system::{Query, Res}
    }, input::{
        keyboard::KeyCode, 
        Input,
    }, time::Time, transform::components::Transform, utils::intern::Interned
};

use crate::{
    components, constants, states
};

pub struct MovePaddlePlugin {
    pub schedule: Interned<dyn ScheduleLabel>,
    pub state: Option<states::AppState>,
}

impl Plugin for MovePaddlePlugin {
    fn build(&self, app: &mut App) {
        match self.state {
            Some(s) => app.add_systems(self.schedule, move_paddle.run_if(in_state(s))),
            None => app.add_systems(self.schedule, move_paddle),
        };
    }
}

fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<components::Paddle>>,
    time: Res<Time>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_paddle_position =
        paddle_transform.translation.x + direction * constants::paddle::PADDLE_SPEED * time.delta_seconds();

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let left_bound = constants::wall::LEFT_WALL + constants::wall::WALL_THICKNESS / 2.0 + constants::paddle::PADDLE_SIZE.x / 2.0 + constants::paddle::PADDLE_PADDING;
    let right_bound = constants::wall::RIGHT_WALL - constants::wall::WALL_THICKNESS / 2.0 - constants::paddle::PADDLE_SIZE.x / 2.0 - constants::paddle::PADDLE_PADDING;

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}
