use bevy::{
    app::{App, Plugin}, ecs::{schedule::{IntoSystemConfigs, ScheduleLabel, common_conditions::in_state}, system::{Query, Res}}, time::Time, transform::components::Transform, utils::intern::Interned
};

use crate::{components, states};

pub struct ApplyVelocityPlugin {
    pub schedule: Interned<dyn ScheduleLabel>,
    pub state: Option<states::AppState>,
}

impl Plugin for ApplyVelocityPlugin {
    fn build(&self, app: &mut App) {
        match self.state {
            Some(s) => app.add_systems(self.schedule, apply_velocity.run_if(in_state(s))),
            None => app.add_systems(self.schedule, apply_velocity),
        };
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &components::Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}
