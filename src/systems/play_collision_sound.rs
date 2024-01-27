use bevy::{
    app::{App, Plugin, Update}, audio::{AudioBundle, PlaybackSettings}, ecs::{
        event::EventReader, schedule::{common_conditions::in_state, IntoSystemConfigs, ScheduleLabel}, system::{Commands, Query, Res}
    }, utils::intern::Interned
};

use crate::{
    components, events, resources, states
};

pub struct PlayCollisionSoundPlugin {
    pub schedule: Interned<dyn ScheduleLabel>,
    pub state: Option<states::AppState>,
}

impl Plugin for PlayCollisionSoundPlugin {
    fn build(&self, app: &mut App) {
        match self.state {
            Some(s) => app.add_systems(self.schedule, play_collision_sound.run_if(in_state(s))),
            None => app.add_systems(self.schedule, play_collision_sound),
        };
    }
}

fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<events::CollisionEvent>,
    collision_sound_query: Query<&components::CollisionSound>,
    sound: Res<resources::CollisionSound>,
) {
    // Play a sound only once per frame
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        collision_events.clear();

        // This checks that the sound isnt already playing
        if collision_sound_query.is_empty() {
            commands.spawn((
                AudioBundle {
                    source: sound.0.clone(),
                    // auto-despawn the entity when playback finishes
                    settings: PlaybackSettings::DESPAWN,
                },
                components::CollisionSound,
            ));
        }
    }
}
