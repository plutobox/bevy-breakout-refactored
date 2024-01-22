use bevy::prelude::{
    Resource,
    Handle,
    AudioSource
};

#[derive(Resource)]
pub struct CollisionSound(pub Handle<AudioSource>);
