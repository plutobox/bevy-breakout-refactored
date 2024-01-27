use bevy::{
    app::{App, Plugin, Update}, ecs::{
        entity::Entity, event::EventWriter, query::With, schedule::{common_conditions::in_state, IntoSystemConfigs, ScheduleLabel}, system::{Commands, Query, ResMut}
    }, sprite::collide_aabb::{collide, Collision}, transform::components::Transform, utils::intern::Interned
};

use crate::{components, events, resources, states};

pub struct CheckForCollisionsPlugin {
    pub schedule: Interned<dyn ScheduleLabel>,
    pub state: Option<states::AppState>,
}

impl Plugin for CheckForCollisionsPlugin {
    fn build(&self, app: &mut App) {
        match self.state {
            Some(s) => app.add_systems(self.schedule, check_for_collisions.run_if(in_state(s))),
            None => app.add_systems(self.schedule, check_for_collisions),
        };
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut scoreboard: ResMut<resources::Scoreboard>,
    mut ball_query: Query<(&mut components::Velocity, &Transform), With<components::Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&components::Brick>), With<components::Collider>>,
    mut collision_events: EventWriter<events::CollisionEvent>,
) {

    ball_query.for_each_mut(|(mut ball_velocity, ball_transform)| {
        let ball_size = ball_transform.scale.truncate();
        collider_query
            .iter()
            .filter(|(_, other_transform, _)| {
                // filter out a ball's collision with itself, this is annoying
                *other_transform != ball_transform
             })
            .filter_map(|(collider_entity, other_transform, maybe_brick)| {
                // check collision occurred
                collide(
                    ball_transform.translation,
                    ball_size,
                    other_transform.translation,
                    other_transform.scale.truncate(),
                ).map(|collision| {
                    (collision, collider_entity, other_transform, maybe_brick)
                })
            })
            .for_each(|(collision, collider_entity, _, maybe_brick)| {
                // Sends a collision event so that other systems can react to the collision
                collision_events.send_default();
    
                // Bricks should be despawned and increment the scoreboard on collision
                if maybe_brick.is_some() {
                    scoreboard.score += 1;
                    commands.entity(collider_entity).despawn();
                }
    
                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;
    
                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => { /* do nothing */ }
                }
    
                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }
    
                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            });
    });
}
