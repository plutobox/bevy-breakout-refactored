use bevy::prelude::{
    Commands,
    EventReader,
    Res,
    PlaybackSettings,
    AudioBundle,
};

use crate::{
    resources::CollisionSound,
    events::CollisionEvent,
};

pub fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
) {
    // Play a sound once per frame if a collision occurred.
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        collision_events.clear();
        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            // auto-despawn the entity when playback finishes
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
