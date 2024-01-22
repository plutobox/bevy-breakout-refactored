
use bevy::prelude::{
    Query,
    Text,
    Res,
};

use crate::resources::Scoreboard;

pub fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
