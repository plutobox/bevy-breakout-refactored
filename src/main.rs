mod bundles;
mod components;
mod constants;
mod events;
mod resources;
mod systems;

use bevy::{
    prelude::{App, ClearColor, DefaultPlugins, FixedUpdate, IntoSystemConfigs, Startup, Update},
    window,
};

use crate::{
    constants::background::BACKGROUND_COLOR,
    events::CollisionEvent,
    resources::Scoreboard,
    systems::{
        apply_velocity, check_for_collisions, move_paddle, play_collision_sound, setup,
        update_scoreboard,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                move_paddle,
                check_for_collisions,
                play_collision_sound,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(Update, (update_scoreboard, window::close_on_esc))
        .run();
}
